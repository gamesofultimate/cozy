use crate::shared::components::{BossSensor, Spawner};
use engine::utils::units::Seconds;
use engine::{
  application::scene::{Scene, TransformComponent},
  systems::{Backpack, Initializable, Inventory, System},
};
use nalgebra::Vector3;

pub struct SpawnSystem {}

impl Initializable for SpawnSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl SpawnSystem {
  pub fn spawn_enemies(&mut self, scene: &mut Scene, backpack: &Backpack) {
    let mut to_spawn = vec![];
    let mut is_active = false;
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    for (_, sensor) in scene.query_mut::<&mut BossSensor>() {
      is_active = sensor.activate;
    }

    for (_entity, (spawner, transform)) in scene.query_mut::<(&mut Spawner, &TransformComponent)>()
    {
      spawner.current_timer += delta_time;
      if is_active
        && spawner.current_entities_spawned < spawner.max_entities_to_spawn
        && spawner.current_timer >= spawner.time_between_spawns
      {
        to_spawn.push((
          spawner.entity_prefab,
          transform.translation,
          spawner.max_entities_to_spawn,
        ));
        spawner.current_entities_spawned += 1;
        spawner.current_timer = Seconds::new(0.0);
      }
    }

    for spawner in to_spawn {
      let prefab_to_spawn = spawner.0;
      scene.spawn_prefab_id_with(prefab_to_spawn, |prefab| {
        let position = Vector3::new(spawner.1.x, 0.0, spawner.1.z);
        prefab.transform.translation = position;
      });
    }
  }
}

impl System for SpawnSystem {
  fn get_name(&self) -> &'static str {
    "SpawnSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.spawn_enemies(scene, backpack);
  }
}
