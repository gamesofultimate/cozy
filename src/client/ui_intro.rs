use crate::shared::ui_components::UIIntro;
use engine::{
  application::{
    components::{SpriteComponent, TextComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
  utils::{easing::*, units::Seconds},
  Entity,
};

use std::collections::HashMap;

const HIDE_DURATION: f32 = 0.5;
const SHOW_DURATION: f32 = 1.5;

#[derive(PartialEq)]
enum UIIntroState {
  Idle,
  Showing,
  Hiding,
}

pub struct UIIntroSystem {
  cached_opacities: HashMap<Entity, (f32, f32)>,

  hide_current_timer: Seconds,
  show_current_timer: Seconds,

  state: UIIntroState,
}

impl Initializable for UIIntroSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {
      cached_opacities: HashMap::new(),

      hide_current_timer: Seconds::new(0.0),
      show_current_timer: Seconds::new(0.0),

      state: UIIntroState::Idle,
    }
  }
}

impl UIIntroSystem {
  fn is_intro_showing(intro: &UIIntro) -> bool {
    if *intro.current_timer < *intro.ease_in_duration {
      return true;
    }

    false
  }

  fn is_intro_hiding(intro: &UIIntro) -> bool {
    if *intro.current_timer > *intro.ease_in_duration + *intro.hold_duration
      && *intro.current_timer
        < *intro.ease_in_duration + *intro.hold_duration + *intro.ease_out_duration
    {
      return true;
    }

    false
  }

  fn handle_state(&mut self, scene: &mut Scene) {
    if self.state == UIIntroState::Idle {
      let query = scene.query_mut::<&UIIntro>();
      for (_, intro) in query {
        if intro.enabled {
          self.state = UIIntroState::Showing;
        }
      }
    } else if self.state == UIIntroState::Showing {
      let query = scene.query_mut::<&UIIntro>();
      for (_, intro) in query {
        if Self::is_intro_hiding(intro) {
          self.state = UIIntroState::Hiding;
        }
      }
    } else if self.state == UIIntroState::Hiding {
      let query = scene.query_mut::<&UIIntro>();
      for (_, intro) in query {
        if !intro.enabled {
          self.state = UIIntroState::Idle;
        }
      }
    }
  }

  fn hide_intro_text(&mut self, scene: &mut Scene) {
    if self.state != UIIntroState::Idle {
      return;
    }

    let query = scene.query_mut::<&UIIntro>();
    for (_, _intro) in query {
      self.hide_current_timer = Seconds::new(0.0);
      self.show_current_timer = Seconds::new(0.0);
    }
  }

  fn cache_default_ui(&mut self, scene: &mut Scene) {
    if !self.cached_opacities.is_empty() {
      return;
    }

    if self.state != UIIntroState::Idle {
      return;
    }

    let query = scene.query_mut::<&mut TextComponent>();
    for (entity, text) in query {
      self.cached_opacities.insert(entity, (text.opacity, 0.0));
    }

    let query = scene.query_mut::<&mut SpriteComponent>();
    for (entity, sprite) in query {
      self.cached_opacities.insert(entity, (0.0, sprite.opacity));
    }
  }

  fn hide_default_ui(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if self.state != UIIntroState::Showing {
      return;
    }

    let delta_time = *backpack.get::<Seconds>().unwrap();
    self.hide_current_timer += delta_time;
    if *self.hide_current_timer > HIDE_DURATION {
      self.hide_current_timer = Seconds::new(HIDE_DURATION);
    }

    for (entity, (text_opacity, sprite_opacity)) in &self.cached_opacities {
      let easing_time = 1.0 - *self.hide_current_timer / HIDE_DURATION;
      let easing_factor = ease(Easing::Linear, easing_time);

      match scene.get_components_mut::<&mut TextComponent>(entity.clone()) {
        Some(text) => text.opacity = text_opacity * easing_factor,
        None => {}
      }

      match scene.get_components_mut::<&mut SpriteComponent>(entity.clone()) {
        Some(sprite) => sprite.opacity = sprite_opacity * easing_factor,
        None => {}
      }
    }
  }

  fn show_intro_text(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = *backpack.get::<Seconds>().unwrap();

    let query = scene.query_mut::<(&mut UIIntro, &mut TextComponent)>();
    for (_, (intro, text)) in query {
      if !intro.enabled {
        continue;
      }

      intro.current_timer += delta_time;

      text.opacity = 1.0;

      let mut current_timer = *intro.current_timer;

      if current_timer < *intro.ease_in_duration {
        let easing_time = current_timer / *intro.ease_in_duration;
        let easing_factor = ease(Easing::Linear, easing_time);
        text.opacity = easing_factor * intro.max_opacity;
      } else if current_timer < *intro.ease_in_duration + *intro.hold_duration {
        // Do nothing
      } else if current_timer
        < *intro.ease_in_duration + *intro.hold_duration + *intro.ease_out_duration
      {
        current_timer -= *intro.ease_in_duration + *intro.hold_duration;

        let easing_time = current_timer / *intro.ease_out_duration;
        let easing_factor = 1.0 - ease(Easing::Linear, easing_time);
        text.opacity = easing_factor * intro.max_opacity;
      } else {
        text.opacity = 0.0;
        intro.enabled = false;
      }
    }
  }

  fn restore_default_ui(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if self.state != UIIntroState::Hiding {
      return;
    }

    let delta_time = *backpack.get::<Seconds>().unwrap();
    self.show_current_timer += delta_time;
    if *self.show_current_timer > SHOW_DURATION {
      self.show_current_timer = Seconds::new(SHOW_DURATION);
    }

    for (entity, (text_opacity, sprite_opacity)) in &self.cached_opacities {
      let easing_time = *self.show_current_timer / SHOW_DURATION;
      let easing_factor = ease(Easing::Linear, easing_time);

      match scene.get_components_mut::<&mut TextComponent>(entity.clone()) {
        Some(text) => text.opacity = text_opacity * easing_factor,
        None => {}
      }

      match scene.get_components_mut::<&mut SpriteComponent>(entity.clone()) {
        Some(sprite) => sprite.opacity = sprite_opacity * easing_factor,
        None => {}
      }
    }
  }
}

impl System for UIIntroSystem {
  fn get_name(&self) -> &'static str {
    "UIIntroSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_state(scene);
    self.hide_intro_text(scene);
    self.cache_default_ui(scene);
    self.hide_default_ui(scene, backpack);
    self.show_intro_text(scene, backpack);
    self.restore_default_ui(scene, backpack);
  }
}
