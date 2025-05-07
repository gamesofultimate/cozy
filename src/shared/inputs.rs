use crate::{
  shared::game_input::{GameInput, InputState},
  shared::components::{Movement, Character, CharacterState},
};
use engine::application::scene::TransformComponent;
use engine::{
  application::{
    components::{
      AudioSourceComponent, InputComponent, PhysicsComponent,
      SourceState,
    },
    scene::Scene,
  },
  systems::{
    world::WorldConfig,
    physics::PhysicsController,
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Seconds, Kph},
  nalgebra::{Vector3, Unit},
};

#[cfg(target_arch = "wasm32")]
use engine::{
  application::input::InputsReader,
  systems::input::CanvasController,
};


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
  fn capture_mouse(&mut self, backpack: &mut Backpack) {
    let input = self.inputs.read_client();
    if input.state.contains(InputState::LeftClick) && !input.state.contains(InputState::IsMouseLocked) {
      self.canvas.capture_mouse(true);
    } else if input.state.contains(InputState::Escape | InputState::IsMouseLocked) {
      self.canvas.capture_mouse(false);
    }

    if let Some(world) = backpack.get_mut::<WorldConfig>() {
      if input.state.contains(InputState::ToggleDebugPerformance) {
        world.debug_performance = !world.debug_performance;
      }
    }
  }

  fn handle_move(&mut self, scene: &mut Scene, delta_time: Seconds) {
    for (_, (component, input, physics, character, movement, _, maybe_audio)) in scene.query_mut::<(
      &mut InputComponent,
      &GameInput,
      &PhysicsComponent,
      &mut Character,
      &mut Movement,
      &mut TransformComponent,
      Option<&mut AudioSourceComponent>,
    )>() {

      if input.state.contains(InputState::IsRunning) && let CharacterState::Normal = character.state {
        character.state = CharacterState::Running;
      }
      if !input.state.contains(InputState::IsRunning) && let CharacterState::Running = character.state {
        character.state = CharacterState::Normal;
      }

      let speed = match character.state {
        CharacterState::Normal => movement.walking_speed,
        CharacterState::Running => movement.running_speed,
        _ => Kph::new(0.0),
      };

      let mut velocity = Vector3::new(0.0, -9.8, 0.0);

      if input.check(InputState::IsMouseLocked) || input.check(InputState::HasJoystick) {
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
      direction.y = 0.0;

      if direction != Vector3::zeros() {
        let direction = Unit::new_normalize(direction);
        movement.direction = direction;
      }

      self
        .physics
        .set_direction(&physics, movement.direction);
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
    self.capture_mouse(backpack);

    self.handle_move(scene, delta_time);
  }
}
