use engine::{
  application::{
    components::ParentComponent,
    physics3d::{ColliderHandle, CollisionEvent},
    scene::Scene,
  },
  systems::{
    physics::{CollisionsReader, PhysicsController},
    Backpack, Initializable, Inventory, System,
  },
  Entity, Query, QueryItem,
};
use std::collections::VecDeque;

use crate::shared::components::{Action, Harvestable, Pickup, Tile, WaterSource};

pub struct CollisionSystem {
  physics: PhysicsController,
  collisions_reader: CollisionsReader,
}

impl Initializable for CollisionSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    let collisions_reader = inventory.get::<CollisionsReader>().clone();

    Self {
      physics,
      collisions_reader,
    }
  }
}

impl System for CollisionSystem {
  fn get_name(&self) -> &'static str {
    "CollisionSystem"
  }

  fn attach(&mut self, _: &mut Scene, _: &mut Backpack) {
    self
      .physics
      .register_collision_handler(handle_collision_event);
  }

  fn provide(&mut self, _: &Inventory) {}

  fn run(&mut self, _: &mut Scene, _: &mut Backpack) {}
}

fn handle_collision_event(
  physics: &PhysicsController,
  scene: &mut Scene,
  collision_event: &CollisionEvent,
) {
  physics.try_handle_collision::<Action, Pickup>(scene, collision_event);
  physics.try_handle_collision::<Action, Tile>(scene, collision_event);
  physics.try_handle_collision::<Action, WaterSource>(scene, collision_event);
  physics.try_handle_collision::<Action, Harvestable>(scene, collision_event);
}
