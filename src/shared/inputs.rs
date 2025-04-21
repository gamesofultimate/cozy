use std::collections::HashMap;
use crate::{
  shared::game_input::{GameInput, InputState},
  shared::components::{Movement, Player},
};
use engine::application::components::ColliderType;
use engine::application::scene::TransformComponent;
use engine::utils::easing::Easing;
use engine::utils::interpolation::Interpolator;
use engine::utils::units::Kph;
use engine::{
  rapier3d::prelude::{
    QueryFilter,
    Ray,
  },
  application::{
    components::{
      AudioSourceComponent, InputComponent, ParentComponent, PhysicsComponent, SocketComponent,
      SourceState, NetworkedPlayerComponent,
    },
    input::InputsReader, 
    scene::Scene,
  },
  systems::{
    physics::PhysicsController, world::WorldConfig,
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Mps, Seconds},
  nalgebra::{Vector2, Vector3, Unit},
  ConnectionId,
};

#[cfg(target_arch = "wasm32")]
use engine::systems::input::CanvasController;


pub struct InputsSystem {
  inputs: InputsReader<GameInput>,
  physics: PhysicsController,
  #[cfg(target_arch = "wasm32")]
  canvas: CanvasController,
  crouch_timer: Seconds,
  uncrouch_timer: Seconds,
  is_crouching: bool,
}

impl Initializable for InputsSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let physics = inventory.get::<PhysicsController>().clone();
    #[cfg(target_arch = "wasm32")]
    let canvas = inventory.get::<CanvasController>().clone();
    Self {
      inputs,
      physics,
      #[cfg(target_arch = "wasm32")]
      canvas,
      crouch_timer: Seconds::new(0.0),
      uncrouch_timer: Seconds::new(0.0),
      is_crouching: false,
    }
  }
}

impl InputsSystem {
  #[cfg(target_arch = "wasm32")]
  fn capture_mouse(&mut self, input: &GameInput) {
    if input.state.contains(InputState::LeftClick) && !input.state.contains(InputState::IsMouseLocked) {
      self.canvas.capture_mouse(true);
    } else if input.state.contains(InputState::Escape | InputState::IsMouseLocked) {
      self.canvas.capture_mouse(false);
    }
  }

  fn ground_check(
    &mut self,
    physics: &PhysicsComponent,
    movement: &mut Movement,
    transform: &mut TransformComponent,
  ) {
    let center = transform.translation;
    let max_distance = movement.distance_to_ground_check;
    let rigid_body_handle = match self.physics.get_rigid_body(&physics.joint.body.id) {
      Some(handle) => handle,
      None => return,
    };
    let filter = QueryFilter::default().exclude_rigid_body(rigid_body_handle);
    let solid = false;

    let direction = Unit::new_normalize(Vector3::new(0.0, -1.0, 0.0));

    let ray = Ray::new(center.into(), direction.into_inner());

    if let Some(_) = self.physics.raycast(&ray, *max_distance, solid, filter) {
      movement.is_grounded = true;
    } else {
      movement.is_grounded = false;
    }
  }

  fn handle_jump(&mut self, input: &GameInput, delta_time: Seconds, movement: &mut Movement) {
    let jump_gravity = (2.0 * *movement.jump_height) / (movement.jump_peak_time.powf(2.0));
    let fall_gravity = (2.0 * *movement.jump_height) / (movement.jump_fall_time.powf(2.0));
    let jump_velocity = jump_gravity * *movement.jump_peak_time;

    if movement.y_velocity > Mps::zero() {
      movement.y_velocity -= Mps::new(jump_gravity * *delta_time);
    } else if movement.is_grounded {
      movement.y_velocity = Mps::new(-9.8); //keep normal gravity if we are grounded so that if we fall off a ledge, gravity isn't compounded
      movement.jump_count = 0;
    } else {
      movement.y_velocity -= Mps::new(fall_gravity * *delta_time);
    }

    movement.extra_jump_timer += delta_time;

    if input.state.contains(InputState::Jump) && movement.is_grounded && movement.jump_count < movement.max_num_jumps {
      movement.y_velocity = Mps::new(jump_velocity);
      movement.extra_jump_timer = Seconds::zero();
      movement.jump_count = 1;
    } else if input.state.contains(InputState::Jump)
      && (!movement.is_grounded
        && (movement.extra_jump_timer >= movement.time_between_jumps
          && movement.jump_count < movement.max_num_jumps))
    {
      movement.y_velocity = Mps::new(jump_velocity);
      movement.extra_jump_timer = Seconds::zero();
      movement.jump_count += 1;
    }
  }

  fn handle_dash(
    &mut self,
    input: &GameInput,
    delta_time: Seconds,
    movement: &mut Movement,
    speed: &mut Kph,
  ) -> Kph {
    if input.state.contains(InputState::Dash) && movement.is_dashing == false {
      if movement.dash_cooldown_timer >= movement.dash_cooldown {
        movement.is_dashing = true;
        movement.dash_timer = Seconds::zero();
      }
    }

    if movement.is_dashing == false {
      movement.dash_cooldown_timer += delta_time;
    } else {
      *speed = movement.dashing_speed;
      movement.dash_timer += delta_time;

      if movement.dash_timer >= movement.dash_time {
        movement.is_dashing = false;
        movement.dash_timer = Seconds::zero();
        movement.dash_cooldown_timer = Seconds::zero();
      }
    }

    return *speed;
  }

  fn handle_move(&mut self, scene: &mut Scene, inputs: &HashMap<ConnectionId, GameInput>, delta_time: Seconds) {
    /*
    use engine::application::scene::Collision;
    use crate::shared::components::Floor;

    for (entity, (player, _)) in scene.query_mut::<(&Player, &Collision<Player, Floor>)>() {
      log::info!("player {:?}", &player);
    }
    */

    let mut parents = vec![];

    for (_, (transform, player, movement, physics)) in scene.query_mut::<(
      &TransformComponent,
      &mut Player,
      &Movement,
      &PhysicsComponent,
    )>() {

      let destination = match player.auto_destination {
        Some(destination) => destination,
        None => continue,
      };

      let distance = (transform.translation - destination).magnitude();
      if distance < 1.0 {
        player.auto_destination = None;
        continue;
      }

      let direction = (destination - transform.translation).normalize();

      let mut velocity = Vector3::new(0.0, *movement.y_velocity, 0.0);
      velocity += direction * *movement.walking_speed;

      self
        .physics
        .move_controller_velocity(&physics, velocity, delta_time);

      //return;
    }

    for (_, (maybe_parent, maybe_socket, component, network)) in scene.query_mut::<(
      Option<&ParentComponent>,
      Option<&SocketComponent>,
      &mut InputComponent,
      &NetworkedPlayerComponent,
    )>() {
      let parent_id = match (maybe_parent, maybe_socket) {
        (Some(parent), _) => parent.parent_id,
        (_, Some(socket)) => socket.parent_id,
        _ => continue,
      };

      let input = match inputs.get(&network.connection_id) {
        Some(data) => data,
        None => continue,
      };

      if input.check(InputState::IsMouseLocked) {
        component.receive_keyboard_rotation(input.delta.component_mul(&Vector2::new(1.0, -1.0)));
      }
      parents.push((parent_id, network.clone(), component.clone()));
    }

    for (parent_id, network, component) in parents.drain(..) {
      let parent_entity = match scene.get_entity(parent_id) {
        Some(data) => data,
        None => continue,
      };

      let (physics, movement, transform, maybe_audio) = match scene.get_components_mut::<(
        &PhysicsComponent,
        &mut Movement,
        &mut TransformComponent,
        Option<&mut AudioSourceComponent>,
      )>(*parent_entity)
      {
        Some(data) => data,
        None => continue,
      };

      let input = match inputs.get(&network.connection_id) {
        Some(data) => data,
        None => continue,
      };

      //log::info!("input: {:?}", &input);

      let mut speed;
      if self.is_crouching {
        speed = movement.crouching_speed;
      } else {
        if input.state.contains(InputState::IsRunning) {
          speed = movement.running_speed;
        } else {
          speed = movement.walking_speed;
        }
      }

      speed = self.handle_dash(input, delta_time, movement, &mut speed);

      self.ground_check(physics, movement, transform);
      self.handle_jump(input, delta_time, movement);

      let mut velocity = Vector3::new(0.0, *movement.y_velocity, 0.0);
      //let mut velocity = Vector3::new(0.0, 0.0, 0.0);

      if input.state.contains(InputState::IsMouseLocked) {
        if input.forward.abs() > component.deadzone {
          velocity += *component.get_front() * input.forward * *speed;
        }

        if input.right.abs() > component.deadzone {
          velocity += component.get_front().cross(&Vector3::y()) * input.right * *speed;
        }
      }
      if let Some(audio) = maybe_audio {
        if movement.is_grounded {
          let velocity = Vector3::new(velocity.x, 0.0, velocity.z);
          if velocity.magnitude() > 0.1 {
            match audio.state {
              SourceState::Stopped => audio.state = SourceState::Playing,
              _ => {}
            }
          } else {
            audio.state = SourceState::Stopped;
          }
        } else {
          audio.state = SourceState::Stopped;
        }
      }

      // log::info!("transform: {:?} vel: {:?}", &transform.world_transform().translation, &velocity);

      self
        .physics
        .move_controller_velocity(&physics, velocity, delta_time);
    }
  }

  fn handle_crouch(&mut self, scene: &mut Scene, inputs: &HashMap<ConnectionId, GameInput>, delta_time: Seconds) {
    for (_, (_transform, _player, _movement, physics_component, network)) in scene.query_mut::<(
      &mut TransformComponent,
      &mut Player,
      &Movement,
      &mut PhysicsComponent,
      &NetworkedPlayerComponent,
    )>() {
      let input = match inputs.get(&network.connection_id) {
        Some(data) => data,
        None => continue,
      };

      let mut interpolator = Interpolator::new(0.2, 0.5, Easing::Linear, 0.0..=0.3);
      interpolator.accumulate(*delta_time);

      if input.state.contains(InputState::Crouch) {
        physics_component.joint.body.collider_type =
          ColliderType::new_capsule_y(interpolator.get(), interpolator.get());
        self.is_crouching = true;
      } else {
        interpolator = Interpolator::new(0.5, 0.25, Easing::Linear, 0.0..=0.3);
        interpolator.accumulate(*delta_time);
        physics_component.joint.body.collider_type =
          ColliderType::new_capsule_y(interpolator.get(), interpolator.get());
        self.is_crouching = false;
      }
    }
  }
}

impl System for InputsSystem {
  fn get_name(&self) -> &'static str {
    "InputsSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    let inputs = self.inputs.read();

    for (id, input) in &inputs {
      #[cfg(target_arch = "wasm32")]
      self.capture_mouse(&input);
    }

    self.handle_move(scene, &inputs, delta_time);
    self.handle_crouch(scene, &inputs, delta_time);
  }
}
