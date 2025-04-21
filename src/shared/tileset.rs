use crate::shared::components::{
  Tileset,
};
use engine::application::components::ModelComponent;
use engine::application::components::TextComponent;
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

  fn attach(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut entities = vec![];

    for (entity, (transform, tileset)) in scene.query_mut::<(&Transform, &Tileset)>() {
      entities.push((entity, transform.clone(), tileset.clone()));
    }

    for (entity, transform, _tileset) in entities {
      scene.spawn_prefab_with("Tile::Grass", |prefab| {
        prefab.transform = transform.into();
      });
      //scene.add_component(entity, Prev(set));
    }
    scene.clear_component::<Tileset>();
  }
}
