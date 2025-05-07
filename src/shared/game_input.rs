use engine::systems::Backpack;
use engine::{
  application::devices::{
    ButtonState, DeviceEvent, GamepadEvent, KeyboardEvent, KeyboardKey, MouseButton, MouseEvent,
    WindowEvent, GamepadButton,
  },
  application::input::Input,
  utils::units::Seconds,
  nalgebra::Vector2,
};

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use bitflags::bitflags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInput {
  pub up: f32,
  pub right: f32,
  pub forward: f32,

  pub delta: Vector2<f32>,
  pub mouse: Vector2<f32>,

  #[serde(skip)]
  horizontal: HashSet<KeyboardKey>,
  #[serde(skip)]
  vertical: HashSet<KeyboardKey>,

  pub state: InputState,
}

bitflags! {
  #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
  pub struct InputState: u32 {
    const Empty                  = 0b00000000000000000000000000000000;
    const IsFocused              = 0b00000000000000000000000000000001;
    const IsMouseLocked          = 0b00000000000000000000000000000010;
    const IsFullscreen           = 0b00000000000000000000000000000100;
    const HasMouse               = 0b00000000000000000000000000001000;
    const HasJoystick            = 0b00000000000000000000000000010000;
    const LeftClick              = 0b00000000000000000000000000100000;
    const Escape                 = 0b00000000000000000000000001000000;
    const Action                 = 0b00000000000000000000000010000000;
    const IsRunning              = 0b00000000000000000000000100000000;
    const ToggleDebugPerformance = 0b00000000000000000000001000000000;
  }
}

impl GameInput {
  pub fn new() -> Self {
    Self {
      up: 0.0,
      right: 0.0,
      forward: 0.0,
      state: InputState::Empty,
      mouse: Vector2::zeros(),
      delta: Vector2::zeros(),
      horizontal: HashSet::new(),
      vertical: HashSet::new(),
    }
  }

  pub fn check(&self, state: InputState) -> bool {
    self.state.contains(state)
  }

  fn handle_joystick(&mut self, event: DeviceEvent) {
    // Enable joystick
    match event {
      DeviceEvent::Gamepad(_, GamepadEvent::Button(..)) => {
        self.state |= InputState::HasJoystick;
      }
      _ => {}
    }

    match event {
      DeviceEvent::Gamepad(
        _,
        GamepadEvent::Button(ButtonState::Down, GamepadButton::RightShoulder),
      ) => {
        self.state |= InputState::IsRunning;
      }
      DeviceEvent::Gamepad(
        _,
        GamepadEvent::Button(ButtonState::Up, GamepadButton::RightShoulder),
      ) => {
        self.state -= InputState::IsRunning;
      }
      _ => {}
    }

    match event {
      DeviceEvent::Gamepad(_, GamepadEvent::Joystick { left, .. }) => {
        const MIN_EPSILON: f32 = 0.0 - 0.02;
        const MAX_EPSILON: f32 = 0.0 + 0.02;

        if left.x > MAX_EPSILON || left.x < MIN_EPSILON {
          self.right += -left.x;
          self.state |= InputState::HasJoystick;
        }
        if left.y > MAX_EPSILON || left.y < MIN_EPSILON {
          self.forward += -left.y;
          self.state |= InputState::HasJoystick;
        }
      }
      _ => {}
    }
  }

  fn handle_keyboard(&mut self, event: DeviceEvent) {
    match event {
      // Start: Keyboard down
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::D | KeyboardKey::Right,
      )) => {
        self.horizontal.insert(KeyboardKey::Right);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::A | KeyboardKey::Left,
      )) => {
        self.horizontal.insert(KeyboardKey::Left);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::W | KeyboardKey::Up,
      )) => {
        self.vertical.insert(KeyboardKey::Up);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::S | KeyboardKey::Down,
      )) => {
        self.vertical.insert(KeyboardKey::Down);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Down, KeyboardKey::LShift)) => {
        self.state |= InputState::IsRunning;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Down, KeyboardKey::Escape)) => {
        self.state |= InputState::Escape;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Down, KeyboardKey::Space)) => {
        self.state |= InputState::Action;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Down, KeyboardKey::P)) => {
        self.state |= InputState::ToggleDebugPerformance;
      }
      // End: Keyboard down

      // Start: Keyboard up
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::D | KeyboardKey::Right,
      )) => {
        self.horizontal.remove(&KeyboardKey::Right);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::A | KeyboardKey::Left,
      )) => {
        self.horizontal.remove(&KeyboardKey::Left);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::W | KeyboardKey::Up,
      )) => {
        self.vertical.remove(&KeyboardKey::Up);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::S | KeyboardKey::Down,
      )) => {
        self.vertical.remove(&KeyboardKey::Down);
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Up, KeyboardKey::LShift)) => {
        self.state -= InputState::IsRunning;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Up, KeyboardKey::Escape)) => {
        self.state -= InputState::Escape;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Up, KeyboardKey::Space)) => {
        self.state -= InputState::Action;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Up, KeyboardKey::P)) => {
        self.state -= InputState::ToggleDebugPerformance;
      }

      _ => {}
    }
  }

  fn handle_mouse(&mut self, event: DeviceEvent) {
    match event {
      // Start: Mouse buttons
      DeviceEvent::Mouse(MouseEvent::Button(ButtonState::Down, MouseButton::Primary)) => {
        self.state |= InputState::LeftClick;
      }
      DeviceEvent::Mouse(MouseEvent::Button(ButtonState::Up, MouseButton::Primary)) => {
        self.state -= InputState::LeftClick;
      }
      // End: Mouse buttons

      // Start: Mouse motion
      DeviceEvent::Mouse(MouseEvent::Motion(_, delta)) => {
        self.delta += delta;
      }
      // End: Mouse motion
      _ => {}
    }
  }

  fn handle_window(&mut self, event: DeviceEvent) {
    match event {
      DeviceEvent::Window(WindowEvent::Focus) => {
        self.state |= InputState::IsFocused;
      },
      DeviceEvent::Window(WindowEvent::Blur) => {
        self.state -= InputState::IsFocused;
      },
      DeviceEvent::Window(WindowEvent::CaptureMouse) => {
        self.state |= InputState::IsMouseLocked;
      },
      DeviceEvent::Window(WindowEvent::ReleaseMouse) => {
        self.state -= InputState::IsMouseLocked;
      },
      DeviceEvent::Window(WindowEvent::RequestFullscreen) => {
        self.state |= InputState::IsFullscreen;
      },
      DeviceEvent::Window(WindowEvent::ReleaseFullscreen) => {
        self.state -= InputState::IsFullscreen;
      },
      _ => {}
    }
  }
}

impl Default for GameInput {
  fn default() -> Self {
    Self::new()
  }
}

impl Input for GameInput {
  fn from_backpack(&mut self, _: &mut Backpack) {}

  fn reset(&mut self) {
    self.delta.x = 0.0;
    self.delta.y = 0.0;
  }

  fn has_mouse_lock(&self) -> bool {
    self.state.contains(InputState::IsMouseLocked)
  }

  fn from_devices(&mut self, event: DeviceEvent, _: Seconds) {

    self.handle_keyboard(event);
    if self.horizontal.len() == 1 {
      if self.horizontal.contains(&KeyboardKey::Right) {
        self.right = -1.0;
      } else if self.horizontal.contains(&KeyboardKey::Left) {
        self.right = 1.0;
      }
    } else {
      self.right = 0.0;
    }

    if self.vertical.len() == 1 {
      if self.vertical.contains(&KeyboardKey::Up) {
        self.forward = 1.0;
      } else if self.vertical.contains(&KeyboardKey::Down) {
        self.forward = -1.0;
      }
    } else {
      self.forward = 0.0;
    }

    self.handle_joystick(event);
    self.handle_mouse(event);
    self.handle_window(event);
  }
}
