use crate::shared::components::{
  Player,
};

use engine::{
  application::{
    components::{CameraComponent, InputComponent, SelfComponent, SocketComponent},
    scene::{Collision, Scene, TransformComponent},
    input::InputsReader, 
  },
  resources::node::Transform,
  systems::{rendering::CameraConfig, Backpack, Initializable, Inventory, System, Subsystem},
  utils::{easing::*, units::Seconds},
  Entity,
  nalgebra::{Unit, Vector3},
  glm,
};

use crate::shared::game_input::{GameInput, InputState};
use engine::application::scene::IdComponent;
use std::collections::HashSet;

use rand::Rng;

pub struct CameraSystem {
  inputs: InputsReader<GameInput>,
}

impl Initializable for CameraSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();

    Self {
      inputs,
    }
  }
}

impl CameraSystem {
  fn update_camera_rotation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (_, transform, camera)) in scene.query_mut::<(
      &SelfComponent,
      &mut TransformComponent,
      &CameraComponent,
    )>() {
      let direction = transform.world_transform().get_forward_direction();
      if let CameraComponent::Perspective { .. } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.front = direction;
        camera_config.up = Unit::new_normalize(Vector3::y());
      }
    }
  }

  fn update_camera_translation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (_, transform, camera)) in scene.query_mut::<(
      &SelfComponent,
      &mut TransformComponent,
      &CameraComponent,
    )>() {
      let camera_position = transform.world_transform();

      if let CameraComponent::Perspective {
        fovy, zfar, znear, ..
      } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.fovy = *fovy;
        camera_config.znear = *znear;
        camera_config.zfar = *zfar;
        camera_config.translation = camera_position.translation;
      }
    }
  }
}

impl System for CameraSystem {
  fn get_name(&self) -> &'static str {
    "CameraSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let inputs = self.inputs.read();

    self.update_camera_rotation(scene, backpack);
    self.update_camera_translation(scene, backpack);
  }
}
