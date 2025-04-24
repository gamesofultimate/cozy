use crate::shared::components::{
  CameraFollow,
  Player,
};

use engine::{
  application::{
    components::{CameraComponent, SelfComponent},
    scene::{Scene, TransformComponent},
    input::InputsReader, 
  },
  systems::{rendering::CameraConfig, Backpack, Initializable, Inventory, System},
  nalgebra::{Unit, Vector3},
};

use crate::shared::game_input::GameInput;

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
  fn update_follow(&mut self, scene: &mut Scene, _: &mut Backpack) {
    let mut camera_transform = None;
    for (_, (_, _, transform)) in scene.query_mut::<(
      &SelfComponent,
      &Player,
      &mut TransformComponent,
    )>() {
      camera_transform = Some(transform.clone());
    }
    for (_, (_, transform)) in scene.query_mut::<(&CameraFollow, &mut TransformComponent)>() {
      if let Some(camera_transform) = camera_transform {
        transform.translation = camera_transform.translation;
      }
    }
  }

  fn update_camera_rotation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (transform, camera)) in scene.query_mut::<(
      &mut TransformComponent,
      &CameraComponent,
    )>() {
      let direction = transform.world_transform().get_forward_direction();
      //let direction = transform.get_forward_direction();
      if let CameraComponent::Perspective { .. } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.front = direction;
        camera_config.up = Unit::new_normalize(Vector3::y());
      }
    }
  }

  fn update_camera_translation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (transform, camera)) in scene.query_mut::<(
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
    self.update_follow(scene, backpack);
    self.update_camera_rotation(scene, backpack);
    self.update_camera_translation(scene, backpack);
  }
}
