use crate::shared::components::{Barrier, Health, Lifecycle, Player, Spawner};
use engine::{
  application::scene::Scene,
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};

pub struct HealthSystem {}

impl Initializable for HealthSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl HealthSystem {
  fn clear_recently_damaged(&mut self, scene: &mut Scene) {
    for (_, health) in scene.query_mut::<&mut Health>() {
      health.recently_damaged = false;
    }
  }

  fn check_for_barrier(&mut self, scene: &mut Scene, _backpack: &Backpack) {
    let mut spawners = 0;
    for (_, _) in scene.query_mut::<&Spawner>() {
      spawners += 1;
    }

    for (_, barrier) in scene.query_mut::<&mut Barrier>() {
      barrier.current_barrier = spawners as f32;
    }
  }

  fn remove_dead_entities(&mut self, scene: &mut Scene, backpack: &Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap().clone();

    let mut to_remove = vec![];
    for (entity, (health, maybe_lifecycle, maybe_player)) in
      scene.query_mut::<(&Health, Option<&mut Lifecycle>, Option<&mut Player>)>()
    {
      if health.current_health > 0.0 {
        continue;
      }

      match maybe_lifecycle {
        Some(lifecycle) => {
          // Lifecycle will take care of despawning this entity
          lifecycle.is_dead = true;
          continue;
        }
        None => {}
      }

      if maybe_player.is_some() {
        let player = maybe_player.unwrap();
        if player.death_transition == false {
          player.death_transition_timer = Seconds::new(0.0);
          player.death_transition = true;
        }
        player.death_transition_timer += delta_time;
        //log::info!("maybe_player.death_transition_timer: {:?}", player.death_transition_timer);

        if player.death_transition_timer >= Seconds::new(2.0) {
          to_remove.push(entity);
        }
      } else {
        to_remove.push(entity);
      }
    }

    // We should trigger a post-death mode before deleting
    for entity in to_remove {
      let _ = scene.remove_entity(entity);
    }
  }
}

impl System for HealthSystem {
  fn get_name(&self) -> &'static str {
    "HealthSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.clear_recently_damaged(scene);
    self.check_for_barrier(scene, backpack);
    self.remove_dead_entities(scene, backpack);
  }
}
