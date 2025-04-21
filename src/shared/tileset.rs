use crate::shared::components::{
  Tileset,
};
use engine::application::components::{
  ModelComponent,
  TextComponent,
};
use engine::{
  application::{
    components::{CameraComponent, InputComponent, PhysicsComponent, SelfComponent},
    physics3d::RigidBodyHandle,
    scene::{Collision, IdComponent, Scene, TransformComponent},
    input::InputsReader, 
  },
  resources::{node::Transform, particles::ParticleId},
  systems::{
    controller::AudioController, physics::PhysicsController,
    controller::ParticleController, Backpack, Initializable, Inventory, System,
  },
  utils::physics,
  utils::units::{Decibels, Degrees, Meters, Mps, Seconds},
  Entity,
  nalgebra::{Point3, Rotation3, Unit, Vector3},
};
use rand::Rng;
use std::f32::consts::PI;
use std::mem::variant_count;
use uuid::Uuid;

pub struct TilesetSystem {
  physics: PhysicsController,
}

impl Initializable for TilesetSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();

    Self {
      physics,
    }
  }
}

impl TilesetSystem {
}

impl System for TilesetSystem {
  fn get_name(&self) -> &'static str {
    "TilesetSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut entities = vec![];

    for (entity, (transform, tileset)) in scene.query_mut::<(&TransformComponent, &Tileset)>() {
      entities.push((entity, transform.clone(), tileset.clone()));
    }

    for (entity, transform, tileset) in entities {
      let half_x = tileset.width / 2;
      let half_z = tileset.length / 2;
      for x in 0..tileset.width {
        for z in 0..tileset.length {
          scene.spawn_prefab_with("Tile::Grass", |prefab| {
            let mut transform = transform.clone();
            transform.translation.x -= half_x as f32;
            transform.translation.x += x as f32;
            transform.translation.z -= half_z as f32;
            transform.translation.z += z as f32;

            prefab.transform = transform.into();
          });
        }
      }
    }
    scene.clear_component::<Tileset>();
  }
}
