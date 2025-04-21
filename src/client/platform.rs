use crate::shared::components::{Platform, PlatformType};
use engine::{
  application::{
    components::PhysicsComponent,
    scene::{Scene, TransformComponent},
  },
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};

use crate::shared::components::{Boss, Health};

pub struct PlatformSystem {
  physics: PhysicsController,
}

impl Initializable for PlatformSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    Self { physics }
  }
}

impl PlatformSystem {
  fn move_platform(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    let mut activate_after_boss = false;
    let query_boss = scene.query_mut::<(&Boss, &Health)>();

    for (_, (_, health)) in query_boss {
      if health.current_health <= 0.1 {
        activate_after_boss = true;
      }
    }

    let query = scene.query_mut::<(&mut Platform, &TransformComponent, &PhysicsComponent)>();
    for (_, (platform, transform, physics)) in query {
      platform.has_activated_after_boss = activate_after_boss;
      if platform.platform_type == PlatformType::OnBossBeaten && platform.has_activated_after_boss {
        platform.platform_type = PlatformType::Toggle;
        platform.enabled = true;
      }

      if !platform.enabled {
        continue;
      }

      if platform.current_timer > platform.one_way_duration {
        platform.current_timer = Seconds::new(0.0);
        platform.translation = -platform.translation;

        if platform.platform_type == PlatformType::Toggle
          || (platform.platform_type == PlatformType::LoopOnce && platform.is_returning)
          || (platform.platform_type == PlatformType::OnBossBeaten && platform.is_returning)
        {
          platform.enabled = false;
        }

        platform.is_returning = !platform.is_returning;
      }

      platform.current_timer += delta_time;

      let speed = platform.translation / *platform.one_way_duration;
      let movement = speed * *delta_time;
      let next_position = transform.translation + movement;
      self
        .physics
        .set_kinematic_translation(physics, next_position);
    }
  }
}

impl System for PlatformSystem {
  fn get_name(&self) -> &'static str {
    "PlatformSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.move_platform(scene, backpack);
  }
}
