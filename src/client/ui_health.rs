use crate::shared::components::Health;
use crate::shared::ui_components::UIHealth;
use crate::shared::ui_components::UIType;
use engine::application::scene::TransformComponent;
use engine::utils::units::Seconds;
use engine::{
  application::{
    components::{SelfComponent, SpriteComponent, TextComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
};
use nalgebra::Vector4;

pub struct UIHealthSystem {}

impl Initializable for UIHealthSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl UIHealthSystem {
  fn display_health(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut player_health = None;
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    for (_, (health, _)) in scene.query_mut::<(&mut Health, &SelfComponent)>() {
      if health.start_blood_timer {
        health.blood_timer += delta_time;
      }
      if health.blood_timer > Seconds::new(2.0) {
        health.start_blood_timer = false;
      }
      player_health = Some(health.clone());
    }

    if let Some(player_health) = player_health {
      for (_, (text, ui_health)) in scene.query_mut::<(&mut TextComponent, &UIHealth)>() {
        let player_percent = (player_health.current_health / player_health.total_health) * 100.0;
        if player_percent <= player_health.low_percentage && ui_health.use_colored_text {
          text.color = Vector4::new(1.0, 0.0, 0.0, 1.0);
        } else {
          text.color = Vector4::new(1.0, 1.0, 1.0, 1.0);
        }
        text.text = format!("{:.1}%", player_percent);
      }

      for (_, (sprite, transform, ui_health)) in
        scene.query_mut::<(&mut SpriteComponent, &mut TransformComponent, &UIHealth)>()
      {
        let player_percent = player_health.current_health / player_health.total_health;

        if ui_health.ui_type == UIType::Permanent {
          transform.scale.x = player_percent * 0.5;
          sprite.opacity = ui_health.target_opacity;
        } else if ui_health.ui_type == UIType::LowMarker {
          if player_percent < 0.20 {
            sprite.opacity = ui_health.target_opacity;
          } else {
            sprite.opacity = 0.0;
          }
        } else if ui_health.ui_type == UIType::DirectionalDamageRight {
          if player_health.blood_timer < Seconds::new(2.0)
            && player_health.start_blood_timer
            && player_health.angle < -45.0
          {
            sprite.opacity = ui_health.target_opacity * (2.0 - *player_health.blood_timer);
          } else {
            sprite.opacity = 0.0;
          }
        } else if ui_health.ui_type == UIType::DirectionalDamageLeft {
          if player_health.blood_timer < Seconds::new(2.0)
            && player_health.start_blood_timer
            && player_health.angle > 45.0
          {
            sprite.opacity = ui_health.target_opacity * (2.0 - *player_health.blood_timer);
          } else {
            sprite.opacity = 0.0;
          }
        } else {
          if player_percent < ui_health.show_below_health {
            sprite.opacity = ui_health.target_opacity - player_percent;
            transform.scale.x = 0.25 * ui_health.max_scale_multiplier + (sprite.opacity);
            transform.scale.y = 0.25 * ui_health.max_scale_multiplier + (sprite.opacity);
          } else {
            sprite.opacity = 0.0;
          }
        }
      }
    }
  }
}

impl System for UIHealthSystem {
  fn get_name(&self) -> &'static str {
    "UIHealthSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.display_health(scene, backpack);
  }
}
