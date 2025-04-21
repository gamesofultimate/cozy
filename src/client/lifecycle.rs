use crate::shared::components::{Enemy, Lifecycle};

use engine::{
  application::{
    components::AudioSourceComponent, components::PhysicsComponent, components::SourceState,
    scene::Scene,
  },
  systems::{
    ai::GoalComponent, physics::PhysicsController, Backpack, Initializable, Inventory, System,
  },
  utils::units::Seconds,
};

pub struct LifecycleSystem {
  physics_controller: PhysicsController,
}

impl Initializable for LifecycleSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics_controller = inventory.get::<PhysicsController>().clone();

    Self { physics_controller }
  }
}

impl System for LifecycleSystem {
  fn get_name(&self) -> &'static str {
    "LifecycleSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    let mut to_despawn = vec![];

    let mut dead = vec![];

    for (entity, (lifecycle, physics, maybe_enemy, maybe_audio)) in scene.query_mut::<(
      &mut Lifecycle,
      Option<&PhysicsComponent>,
      Option<&Enemy>,
      Option<&mut AudioSourceComponent>,
    )>() {
      if !lifecycle.is_dead {
        continue;
      }

      dead.push(entity.clone());

      if let Some(_enemy) = maybe_enemy {
        if let Some(audio) = maybe_audio {
          match audio.state {
            SourceState::Playing => audio.state = SourceState::Stopped,
            _ => {}
          }
        }
      }

      if lifecycle.despawn_physics_immediately
        && let Some(physics) = physics
      {
        lifecycle.despawn_physics_immediately = false;
        self.physics_controller.despawn(physics);
      }

      lifecycle.lifetime_timer += Seconds::new(*delta_time);

      if lifecycle.lifetime_timer >= lifecycle.lifetime {
        to_despawn.push(entity.clone());
      }
    }

    for entity in dead {
      let _ = scene.remove_component::<GoalComponent>(entity);
    }

    for entity in to_despawn {
      let _ = scene.remove_entity(entity);
    }
  }
}
