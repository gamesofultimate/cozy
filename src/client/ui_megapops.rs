use crate::shared::components::{Barrier, Boss, BossSensor, Enemy, Health, Spawner};
use crate::shared::ui_components::{UIMegaPops, UIMegaPopsBarrier};
use engine::{
  application::{
    components::{SpriteComponent, TextComponent},
    scene::{Scene, TransformComponent},
  },
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};

pub struct UIMegaPopsSystem {
  activate_ui: bool,
  activate_health_ui: bool,
  timer_second_phase: bool,
  timer: Seconds,
}

impl Initializable for UIMegaPopsSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {
      activate_ui: false,
      activate_health_ui: false,
      timer_second_phase: false,
      timer: Seconds::new(0.0),
    }
  }
}

impl UIMegaPopsSystem {
  fn display_health(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap().clone();
    let mut boss_health = None;
    let mut boss_barrier = None;
    let mut total_portal_health = 0.0;
    let mut current_portal_health = 0.0;
    self.activate_ui = false;

    for (_, sensor) in scene.query_mut::<&mut BossSensor>() {
      self.activate_ui = sensor.activate;
    }

    if (self.activate_ui || self.activate_health_ui) && self.timer <= Seconds::new(1.0) {
      self.timer += delta_time;
    }

    for (_, (_, health, barrier, _)) in scene.query_mut::<(&mut Enemy, &Health, &Barrier, &Boss)>()
    {
      boss_health = Some(health.clone());
      boss_barrier = Some(barrier.clone());
      if barrier.current_barrier == 0.0 {
        self.activate_health_ui = true;
        if self.timer_second_phase == false {
          self.timer = Seconds::new(0.0);
          self.timer_second_phase = true;
        }
      } else {
        self.activate_health_ui = false;
      }
    }

    for (_, (_, health, _)) in scene.query_mut::<(&mut Enemy, &Health, &Spawner)>() {
      current_portal_health += health.current_health;
      total_portal_health = health.total_health;
    }

    if let Some(boss_health) = boss_health {
      for (_, (sprite, _ui_megapops, transform)) in scene
        .query_mut::<(&mut SpriteComponent, &UIMegaPops, &mut TransformComponent)>()
        .without::<UIMegaPopsBarrier>()
      {
        if self.activate_ui && self.activate_health_ui && boss_health.current_health >= 0.0 {
          let boss_percent = boss_health.current_health / boss_health.total_health;
          transform.scale.x = (boss_percent * 0.5) * *self.timer;
          sprite.opacity = 1.0;
        } else if !self.activate_health_ui {
          sprite.opacity = 0.0;
        } else {
          sprite.opacity = 0.0;
          self.activate_ui = false;
        }
      }
      for (_, (text, _ui_megapops)) in scene.query_mut::<(&mut TextComponent, &UIMegaPops)>() {
        if self.activate_ui && boss_health.current_health >= 0.0 {
          text.opacity = 1.0;
        } else {
          text.opacity = 0.0;
        }
      }
    }

    if let Some(boss_barrier) = boss_barrier {
      for (_, (sprite, _ui_megapops, _ui_megapops_barrier, transform)) in scene.query_mut::<(
        &mut SpriteComponent,
        &UIMegaPops,
        &UIMegaPopsBarrier,
        &mut TransformComponent,
      )>() {
        if self.activate_ui {
          //let boss_percent = boss_barrier.current_barrier / boss_barrier.total_barrier; //keeping the old way here if we need to go back to it
          let boss_percent =
            current_portal_health / (total_portal_health * boss_barrier.total_barrier);
          transform.scale.x = (boss_percent * 0.5) * *self.timer;
          sprite.opacity = 1.0;
        } else {
          sprite.opacity = 0.0;
        }
      }
    }
  }
}

impl System for UIMegaPopsSystem {
  fn get_name(&self) -> &'static str {
    "UIMegaPopsSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.display_health(scene, backpack);
  }
}
