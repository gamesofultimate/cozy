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
    }
  }
}

impl InputsSystem {
  #[cfg(target_arch = "wasm32")]
  fn capture_mouse(&mut self) {
    let input = self.inputs.read_client();
    if input.state.contains(InputState::LeftClick) && !input.state.contains(InputState::IsMouseLocked) {
      self.canvas.capture_mouse(true);
    } else if input.state.contains(InputState::Escape | InputState::IsMouseLocked) {
      self.canvas.capture_mouse(false);
    }
  }

  fn handle_move(&mut self, scene: &mut Scene, delta_time: Seconds) {
    for (_, (component, input, physics, movement, transform, maybe_audio)) in scene.query_mut::<(
      &mut InputComponent,
      &GameInput,
      &PhysicsComponent,
      &mut Movement,
      &mut TransformComponent,
      Option<&mut AudioSourceComponent>,
    )>() {
      let speed = if input.state.contains(InputState::IsRunning) {
        movement.running_speed
      } else {
        movement.walking_speed
      };

      let mut velocity = Vector3::new(0.0, -9.8, 0.0);
      let mut direction = Vector3::new(0.0, 0.0, -1.0);

      if input.state.contains(InputState::IsMouseLocked) {
        velocity.z += input.forward * *speed;
        velocity.x += input.right * *speed;
      } else if input.check(InputState::HasJoystick) {
        if input.forward.abs() > component.deadzone {
          velocity.z += input.forward * *speed;
        }

        if input.right.abs() > component.deadzone {
          velocity.x += input.right * *speed;
        }
      }

      if let Some(audio) = maybe_audio && audio.state == SourceState::Stopped {
        let velocity = Vector3::new(velocity.x, 0.0, velocity.z);
        if velocity.magnitude() > 0.1 {
          audio.state = SourceState::Playing;
        } else {
          audio.state = SourceState::Stopped;
        }
      }

      let mut direction = velocity.clone();
      //log::info!("direction: {:?}", &direction);
      direction.y = 0.0;

      if direction != Vector3::zeros() {
        let direction = Unit::new_normalize(direction);
        movement.direction = direction;
      }

      log::info!("direction: {:?}", &movement.direction);

      self
        .physics
        .set_direction(&physics, -movement.direction);
      self
        .physics
        .move_controller_velocity(&physics, velocity, delta_time);
    }
  }
}

impl System for InputsSystem {
  fn get_name(&self) -> &'static str {
    "InputsSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    #[cfg(target_arch = "wasm32")]
    self.capture_mouse();

    self.handle_move(scene, delta_time);
  }
}
