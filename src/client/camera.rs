use crate::shared::components::{
  AnimatedCamera, BossCameraSensor, CameraShake, CameraTrack, Health, Noise, Player,
  RequestLevelLoad,
};

use crate::shared::components::LoadingCameraTrack;
use engine::{
  application::{
    components::{CameraComponent, InputComponent, SelfComponent, SocketComponent},
    scene::{Collision, Scene, TransformComponent},
    input::InputsReader, 
  },
  resources::node::Transform,
  systems::{rendering::CameraConfig, Backpack, Initializable, Inventory, Middleware, Subsystem},
  utils::{easing::*, units::Seconds},
  Entity,
  nalgebra::{Unit, Vector3},
  glm,
};

use crate::shared::game_input::{GameInput, InputState};
use engine::application::scene::IdComponent;
use std::collections::HashSet;

use rand::Rng;

pub struct CameraTracking {
  pub tracking: Option<bool>,
}

pub struct CameraMiddleware {
  inputs: InputsReader<GameInput>,

  is_on_track: bool,
  handled_shake: HashSet<Entity>,
  is_using_socket_animation: bool,

  is_tracked: Option<bool>,
}

impl Initializable for CameraMiddleware {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();

    Self {
      inputs,
      is_on_track: false,
      handled_shake: HashSet::new(),
      is_using_socket_animation: false,
      is_tracked: None,
    }
  }
}

impl CameraMiddleware {
  fn handle_animated_camera_trigger(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut activate = false;
    let mut s_transform = Transform::zeros();
    for (_, (sensor, _)) in
      scene.query_mut::<(&mut BossCameraSensor, &Collision<Player, BossCameraSensor>)>()
    {
      sensor.activate = true; //TODO - Ricardo: Unfinished, just pushing it to add in the sensor fade in part and boss planner.
      activate = sensor.activate;
    }
    for (_, (socket, animated)) in scene.query_mut::<(&SocketComponent, &mut AnimatedCamera)>() {
      animated.is_active = activate;
      if activate {
        self.is_using_socket_animation = true;
        s_transform = socket.socket_transform;
      }
    }

    if self.is_using_socket_animation {
      if let Some(camera_config) = backpack.get_mut::<CameraConfig>() {
        camera_config.translation = s_transform.translation;
        camera_config.front = s_transform.get_forward_direction();
        camera_config.up = Unit::new_normalize(Vector3::y());
      }
    }
  }

  pub fn handle_loading_camera_track(
    &mut self,
    scene: &mut Scene,
    backpack: &mut Backpack,
    input: &GameInput,
  ) {
    backpack.insert(CameraTracking {
      tracking: self.is_tracked,
    });

    if let Some(false) = self.is_tracked {
      return;
    }

    let mut camera_track_id = None;

    let mut has_track = false;

    for (_, _camera_track) in scene.query_mut::<&mut LoadingCameraTrack>() {
      has_track = true;
      self.is_tracked = Some(true);
    }

    if !has_track {
      return;
    }

    for (_, (camera_track, id, _transform)) in
      scene.query_mut::<(&mut LoadingCameraTrack, &IdComponent, &TransformComponent)>()
    {
      match self.is_tracked {
        Some(true) => (),
        _ => continue,
      }

      if input.state.contains(InputState::LeftClick) {
        camera_track.enabled = true;
      }

      if camera_track.current_timer >= camera_track.duration {
        camera_track.enabled = false;
        camera_track.current_timer = Seconds::new(0.0);
        self.is_tracked = Some(false);

        backpack.insert(RequestLevelLoad);
        continue;
      }

      camera_track_id = Some(id.clone());

      if camera_track.enabled {
        camera_track.current_timer += *backpack.get::<Seconds>().unwrap();
      }
    }

    match self.is_tracked {
      Some(true) => (),
      _ => return,
    }

    if let Some(camera_track_id) = camera_track_id {
      let mut _child_entity = Entity::DANGLING;
      {
        let mut query = scene.query::<&SocketComponent>();
        _child_entity = query
          .iter()
          .find(|(_, parent)| parent.parent_id == *camera_track_id)
          .unwrap()
          .0;
      }

      if let Some((camera, transform)) =
        scene.get_components_mut::<(&CameraComponent, &TransformComponent)>(_child_entity)
      {
        if let CameraComponent::Perspective { fovy, .. } = camera {
          if let Some(camera) = backpack.get_mut::<CameraConfig>() {
            camera.fovy = *fovy;
            camera.front = transform.world_transform().get_forward_direction();
            camera.translation = transform.world_transform().translation;
            camera.up = Unit::new_normalize(Vector3::y());
          }
        }
      }
    }
  }

  fn handle_camera_track(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.is_on_track = false;

    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    let mut current_camera_track = None;
    let query = scene.query_mut::<&mut CameraTrack>();
    for (_, camera_track) in query {
      if !camera_track.is_running {
        continue;
      }

      camera_track.current_index = 0;
      camera_track.current_timer += delta_time;

      let mut track_duration = Seconds::new(0.0);
      let node_durations = camera_track.nodes.iter().map(|node| node.duration);

      for duration in node_durations {
        track_duration += duration;

        if camera_track.current_timer > track_duration {
          camera_track.current_index += 1;
        }
      }

      if camera_track.current_timer > track_duration {
        camera_track.current_timer = Seconds::new(0.0);
        camera_track.is_running = false;
        continue;
      }

      let mut camera_track_clone = camera_track.clone();

      let mut current_duration = Seconds::new(0.0);
      let node_durations = camera_track
        .nodes
        .iter()
        .take(camera_track.current_index)
        .map(|node| node.duration);

      for duration in node_durations {
        current_duration += duration;
      }

      camera_track_clone.current_timer -= current_duration;
      current_camera_track = Some(camera_track_clone);
    }

    let camera_track = match current_camera_track {
      Some(current_camera_track) => current_camera_track.clone(),
      None => return,
    };

    self.is_on_track = true;

    let current_node = camera_track.nodes[camera_track.current_index];

    let mut next_node = camera_track.nodes[camera_track.current_index];
    if camera_track.current_index < camera_track.nodes.len() - 1 {
      next_node = camera_track.nodes[camera_track.current_index + 1];
    }

    let current_entity = match scene.get_entity(current_node.prefab) {
      Some(data) => data.clone(),
      None => return,
    };

    let next_entity = match scene.get_entity(next_node.prefab) {
      Some(data) => data.clone(),
      None => return,
    };

    let current_transform = scene
      .get_components_mut::<&TransformComponent>(current_entity)
      .unwrap();

    let current_world_transform = current_transform.world_transform();

    let next_transform = scene
      .get_components_mut::<&TransformComponent>(next_entity)
      .unwrap();

    let next_world_transform = next_transform.world_transform();

    let easing_time = *camera_track.current_timer / *current_node.duration;
    let easing_factor = ease(current_node.easing, easing_time);

    let camera_position = glm::lerp(
      &current_world_transform.translation,
      &next_world_transform.translation,
      easing_factor,
    );

    let camera_direction_quaternion = glm::quat_slerp(
      &current_world_transform.rotation,
      &next_world_transform.rotation,
      easing_factor,
    );

    let camera_direction = Unit::new_normalize(camera_direction_quaternion) * Vector3::z();

    if let Some(camera_config) = backpack.get_mut::<CameraConfig>() {
      camera_config.translation = camera_position;
      camera_config.front = Unit::new_normalize(camera_direction);
      camera_config.up = Unit::new_normalize(Vector3::y());
    }
  }

  fn handle_damage_shake(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    let mut recently_damaged = false;
    let query = scene.query_mut::<(&SelfComponent, &Health, &Player)>();
    for (_, (_, health, _)) in query {
      recently_damaged = health.recently_damaged;
    }

    let query = scene.query_mut::<(
      &SelfComponent,
      &CameraComponent,
      &InputComponent,
      &mut CameraShake,
    )>();
    for (_, (_, _, input, camera_shake)) in query {
      if recently_damaged {
        camera_shake.current_damage_timer = Seconds::new(0.0);
      }

      camera_shake.current_damage_timer += delta_time;

      if camera_shake.current_damage_timer > camera_shake.damage_shake_duration {
        camera_shake.current_damage_shake_direction = Vector3::zeros();
        continue;
      }

      let shake_timer_percentage =
        *camera_shake.current_damage_timer / *camera_shake.damage_shake_duration;
      let shake_ease_factor =
        ease_mirrored(camera_shake.damage_shake_easing, shake_timer_percentage);
      let shake_factor = camera_shake.damage_shake_strength * shake_ease_factor;
      camera_shake.current_damage_shake_strength = shake_factor;

      if !recently_damaged {
        return;
      }

      camera_shake.current_damage_timer = Seconds::new(0.0);

      let camera_direction = input.get_front();
      let right = camera_direction.cross(&Vector3::y()).normalize();
      let up = right.cross(&camera_direction).normalize();

      let rand = rand::thread_rng().gen_range(-1.0..1.0);
      let mut shake_direction = right * rand;

      let rand = rand::thread_rng().gen_range(-1.0..1.0);
      shake_direction += up * rand;

      camera_shake.current_damage_shake_direction = shake_direction;
    }
  }

  fn handle_grenade_noise_shake(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    let mut recently_exploded = None;
    let query = scene.query_mut::<(&TransformComponent, &Noise)>();
    for (entity, (transform, _)) in query {
      if self.handled_shake.contains(&entity) {
        continue;
      }

      self.handled_shake.insert(entity);
      recently_exploded = Some(transform.translation);
    }

    let query = scene.query_mut::<(
      &SelfComponent,
      &CameraComponent,
      &InputComponent,
      &TransformComponent,
      &mut CameraShake,
    )>();
    for (_, (_, _, input, transform, camera_shake)) in query {
      if let Some(_) = recently_exploded {
        camera_shake.current_grenade_timer = Seconds::new(0.0);
      }

      camera_shake.current_grenade_timer += delta_time;

      if camera_shake.current_grenade_timer > camera_shake.grenade_shake_duration {
        camera_shake.current_grenade_shake_direction = Vector3::zeros();
        continue;
      }

      let shake_timer_percentage =
        *camera_shake.current_grenade_timer / *camera_shake.grenade_shake_duration;
      let shake_ease_factor =
        ease_mirrored(camera_shake.grenade_shake_easing, shake_timer_percentage);
      let shake_factor = camera_shake.grenade_shake_strength * shake_ease_factor;
      camera_shake.current_grenade_shake_strength = shake_factor;

      if let None = recently_exploded {
        continue;
      }

      let camera_world_transform = transform.world_transform();

      let distance_to_explosion = Vector3::metric_distance(
        &camera_world_transform.translation,
        &recently_exploded.unwrap(),
      );

      if distance_to_explosion > *camera_shake.grenade_shake_radius {
        continue;
      }

      camera_shake.current_grenade_timer = Seconds::new(0.0);

      let camera_direction = input.get_front();
      let right = camera_direction.cross(&Vector3::y()).normalize();
      let up = right.cross(&camera_direction).normalize();

      let rand = rand::thread_rng().gen_range(-1.0..1.0);
      let mut shake_direction = right * rand;

      let rand = rand::thread_rng().gen_range(-1.0..1.0);
      shake_direction += up * rand;

      camera_shake.current_grenade_shake_direction = shake_direction;
    }
  }

  fn update_camera_rotation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (_, transform, camera, input, _maybe_camera_shake)) in scene.query_mut::<(
      &SelfComponent,
      &mut TransformComponent,
      &CameraComponent,
      &InputComponent,
      Option<&CameraShake>,
    )>() {
      let camera_direction = input.get_front();
      let pitch = input.pitch();
      let yaw = input.yaw();

      let rotated_angle = Vector3::new(-1.0 * *pitch, *yaw, 0.0);

      transform.rotation = rotated_angle;

      if let CameraComponent::Perspective { .. } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.front = camera_direction;
        camera_config.up = Unit::new_normalize(Vector3::y());
      }
    }
  }

  fn update_camera_translation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (_, transform, camera, _input, maybe_camera_shake)) in scene.query_mut::<(
      &SelfComponent,
      &mut TransformComponent,
      &CameraComponent,
      &InputComponent,
      Option<&CameraShake>,
    )>() {
      let mut camera_position = transform.world_transform();

      if let Some(camera_shake) = maybe_camera_shake {
        camera_position.translation += (camera_shake.current_damage_shake_direction
          * camera_shake.current_damage_shake_strength)
          + (camera_shake.current_grenade_shake_direction
            * camera_shake.current_grenade_shake_strength);
      }

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

impl Middleware for CameraMiddleware {
  fn get_name(&self) -> &'static str {
    "CameraMiddleware"
  }

  fn pre(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if !self.is_on_track {
      self.update_camera_rotation(scene, backpack);
    }
  }

  fn post(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let inputs = self.inputs.read();

    self.handle_camera_track(scene, backpack);

    for (id, input) in inputs {
      self.handle_loading_camera_track(scene, backpack, &input);
    }

    if !self.is_on_track {
      self.handle_animated_camera_trigger(scene, backpack);
      self.handle_damage_shake(scene, backpack);
      self.handle_grenade_noise_shake(scene, backpack);
      self.update_camera_translation(scene, backpack);
    }
  }
}

pub struct CameraSubsystem;

impl Subsystem for CameraSubsystem {
  fn get_priority() -> isize {
    0_100_000
  }
}
