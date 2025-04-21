use crate::client::game_input::GameInput;
use crate::shared::components::{Checkpoint, Player, RestoreState, SaveState};
use engine::application::{
  components::PhysicsComponent,
  scene::{Collision, IdComponent, Prefab, Scene, TransformComponent},
};
use engine::systems::{
  input::InputsReader, physics::PhysicsController, Backpack, Initializable, Inventory, System,
};
use engine::Entity;
use std::collections::HashMap;

pub struct CheckpointSystem {
  current_checkpoint: Checkpoint,
  prefabs: HashMap<Entity, Prefab>,
  inputs: InputsReader<GameInput>,
  physics_controller: PhysicsController,
}

impl Initializable for CheckpointSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let physics_controller = inventory.get::<PhysicsController>().clone();
    Self {
      current_checkpoint: Checkpoint::new(),
      prefabs: HashMap::new(),
      inputs,
      physics_controller,
    }
  }
}

impl CheckpointSystem {
  fn handle_checkpoints(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let mut checkpoint = None;
    for (entity, (_check, _)) in
      scene.query_mut::<(&mut Checkpoint, &Collision<Player, Checkpoint>)>()
    {
      checkpoint = Some(entity);
    }

    let checkpoint = checkpoint?;

    let mut entities = vec![];

    if let Some(current_checkpoint) = scene.get_components_mut::<&mut Checkpoint>(checkpoint) {
      if current_checkpoint.saved == false {
        backpack.insert(SaveState {});
        self.current_checkpoint = current_checkpoint.clone();
        current_checkpoint.saved = true;
        for (entity, _) in scene.query_mut::<&TransformComponent>() {
          entities.push(entity);
        }
        self.prefabs.clear();
      }
    }

    for ent in &entities {
      let prefab_to_save = Prefab::pack(scene, *ent);
      self.prefabs.insert(*ent, prefab_to_save.clone().unwrap());
    }

    Some(())
  }

  fn reload_checkpoint(&mut self, scene: &mut Scene, input: &GameInput, backpack: &mut Backpack) {
    if !input.respawn || self.prefabs.is_empty() {
      return;
    }

    let mut entity_vec = vec![];
    for (ent, _id) in scene.query_mut::<&IdComponent>() {
      if !self.prefabs.contains_key(&ent) {
        entity_vec.push(ent);
      }
    }

    for ent in entity_vec {
      let _ = scene.remove_entity(ent);
    }

    for (stored_entity, prefab) in self.prefabs.clone() {
      let mut entity = match scene.get_components_mut::<&IdComponent>(stored_entity) {
        Some(_) => Some(stored_entity.clone()),
        None => None,
      };

      if entity.is_none() {
        self.prefabs.remove(&stored_entity);
        entity = Some(scene.create_raw_entity(&prefab.tag.name));
        self.prefabs.insert(entity.unwrap(), prefab.clone());
      }

      let entity = entity.unwrap();
      scene.create_with_prefab(entity, prefab.clone());

      let physics = match scene.get_components_mut::<&PhysicsComponent>(entity) {
        Some(physics) => physics,
        None => continue,
      };

      self.physics_controller.despawn(physics);
      self
        .physics_controller
        .insert(entity, &physics, prefab.transform.world_transform());
    }
    backpack.insert(RestoreState {});
  }
}

impl System for CheckpointSystem {
  fn get_name(&self) -> &'static str {
    "CheckpointSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let input = self.inputs.read();

    self.handle_checkpoints(scene, backpack);
    self.reload_checkpoint(scene, &input, backpack);
  }
}
