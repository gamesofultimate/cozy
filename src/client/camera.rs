use crate::shared::components::{ActiveCamera, CameraFollower, Player};

use engine::{
  application::{
    components::{CameraComponent, SelfComponent},
    input::InputsReader,
    scene::{Scene, TransformComponent},
  },
  nalgebra::{Unit, UnitQuaternion, Vector3},
  systems::{rendering::CameraConfig, Backpack, Initializable, Inventory, Middleware, Subsystem},
};

use crate::shared::game_input::GameInput;

pub struct CameraMiddleware {}

impl Initializable for CameraMiddleware {
  fn initialize(inventory: &Inventory) -> Self {
    Self {}
  }
}

impl CameraMiddleware {
  fn update_follow(&mut self, scene: &mut Scene, _: &mut Backpack) {
    let mut camera_transform = None;

    for (_, (_, transform)) in scene.query_mut::<(&ActiveCamera, &mut TransformComponent)>() {
      camera_transform = Some(transform.clone());
    }

    for (_, (follower, transform)) in
      scene.query_mut::<(&mut CameraFollower, &mut TransformComponent)>()
    {
      if let Some(camera_transform) = camera_transform {
        transform.translation = camera_transform.translation;
      }
    }
  }

  fn update_camera_rotation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let follower = match scene.query_one::<&CameraFollower>() {
      Some((_, follower)) => follower.clone(),
      None => return,
    };

    for (_, (transform, camera)) in scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
    {
      let direction = transform.world_transform().get_forward_direction();
      //let direction = transform.get_forward_direction();
      if let CameraComponent::Perspective { .. } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        let up = Vector3::y();
        let q1 = UnitQuaternion::face_towards(&camera_config.front, &up);
        let q2 = UnitQuaternion::face_towards(&direction, &up);
        let transition = q1.slerp(&q2, follower.interpolation_speed);

        camera_config.front = Unit::new_normalize(transition * Vector3::z());
        camera_config.up = Unit::new_normalize(up);
      }
    }
  }

  fn update_camera_translation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let follower = match scene.query_one::<&CameraFollower>() {
      Some((_, follower)) => follower.clone(),
      None => return,
    };

    for (_, (transform, camera)) in scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
    {
      let camera_position = transform.world_transform();

      if let CameraComponent::Perspective {
        fovy, zfar, znear, ..
      } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.fovy = *fovy;
        camera_config.znear = *znear;
        camera_config.zfar = *zfar;
        camera_config.translation = camera_config
          .translation
          .lerp(&camera_position.translation, follower.interpolation_speed);
      }
    }
  }
}

impl Middleware for CameraMiddleware {
  fn get_name(&self) -> &'static str {
    "CameraMiddleware"
  }

  fn post(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.update_follow(scene, backpack);
    self.update_camera_rotation(scene, backpack);
    self.update_camera_translation(scene, backpack);
  }
}

pub struct CameraSubsystem;

impl Subsystem for CameraSubsystem {
  fn get_priority() -> isize {
    0_100_000
  }
}
