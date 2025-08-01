use crate::{
  shared::components::{Character, CharacterState, Movement},
  shared::game_input::{GameInput, InputState},
};
use engine::application::scene::TransformComponent;
use engine::{
  application::{
    components::{AudioSourceComponent, InputComponent, PhysicsComponent, SourceState},
    scene::Scene,
  },
  nalgebra::{Unit, Vector3},
  systems::{
    physics::PhysicsController, world::WorldConfig, Backpack, Initializable, Inventory, System,
  },
  utils::units::{Kph, Seconds},
};

use crate::shared::state_machine::{GameState, StateMachine};

#[cfg(target_arch = "wasm32")]
use engine::{application::input::InputsReader, systems::input::CanvasController};

pub struct InputsSystem {
  physics: PhysicsController,
  #[cfg(target_arch = "wasm32")]
  inputs: InputsReader<GameInput>,
  #[cfg(target_arch = "wasm32")]
  canvas: CanvasController,
}

impl Initializable for InputsSystem {
  fn initialize(inventory: &Inventory) -> Self {
    #[cfg(target_arch = "wasm32")]
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let physics = inventory.get::<PhysicsController>().clone();
    #[cfg(target_arch = "wasm32")]
    let canvas = inventory.get::<CanvasController>().clone();
    Self {
      #[cfg(target_arch = "wasm32")]
      inputs,
      physics,
      #[cfg(target_arch = "wasm32")]
      canvas,
    }
  }
}

impl InputsSystem {
  #[cfg(target_arch = "wasm32")]
  fn debug(&mut self, backpack: &mut Backpack) {
    let input = self.inputs.read_client();
    if let Some(world) = backpack.get_mut::<WorldConfig>() {
      if input.state.contains(InputState::ToggleDebugPerformance) {
        world.debug_performance = !world.debug_performance;
      }
      if input.state.contains(InputState::ToggleDebugPhysics) {
        world.debug_physics = !world.debug_physics;
      }
    }
  }

  fn handle_move(&mut self, scene: &mut Scene, backpack: &mut Backpack, delta_time: Seconds) {
    for (_, (component, input, physics, character, movement, _, maybe_audio)) in scene.query_mut::<(
      &mut InputComponent,
      &GameInput,
      &PhysicsComponent,
      &mut CharacterState,
      &mut Movement,
      &mut TransformComponent,
      Option<&mut AudioSourceComponent>,
    )>() {
      if input.state.contains(InputState::IsRunning)
        && let CharacterState::Normal = character
      {
        *character = CharacterState::Running;
      }
      if !input.state.contains(InputState::IsRunning)
        && let CharacterState::Running = character
      {
        *character = CharacterState::Normal;
      }

      let speed = match character {
        CharacterState::Normal => movement.walking_speed,
        CharacterState::Running => movement.running_speed,
        _ => Kph::new(0.0),
      };

      let mut velocity = Vector3::new(0.0, -9.8, 0.0);

      if input.forward.abs() > component.deadzone {
        velocity.z += input.forward * *speed;
      }

      if input.right.abs() > component.deadzone {
        velocity.x += input.right * *speed;
      }
      // NOTE: This needs to be enabled, but needs to be done
      // in the server as well, so we can make the game multiplayer
      /*
        if let Some(machine) = backpack.get_mut::<StateMachine>() {
          match &machine.state {
            GameState::Playing => {
            }
            _ => {}
          }
        };
      */

      if let Some(audio) = maybe_audio
        && audio.state == SourceState::Stopped
      {
        let velocity = Vector3::new(velocity.x, 0.0, velocity.z);
        if velocity.magnitude() > 0.1 {
          audio.state = SourceState::Playing;
        } else {
          audio.state = SourceState::Stopped;
        }
      }

      let mut direction = velocity.clone();
      direction.y = 0.0;

      if direction != Vector3::zeros() {
        let direction = Unit::new_normalize(direction);
        movement.direction = direction;
      }

      self.physics.set_direction(&physics, movement.direction);
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
    self.debug(backpack);
    self.handle_move(scene, backpack, delta_time);
  }
}
