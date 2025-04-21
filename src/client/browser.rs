#![cfg(target_arch = "wasm32")]
use crate::client::game_input::GameInput;
use engine::{
  application::scene::Scene,
  systems::{
    browser::{BrowserController, BrowserReceiver},
    input::InputsReader,
    Backpack, Initializable, Inventory, System,
  },
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Serialize, Deserialize, Tsify)]
pub enum Message {
  StartGame,
  StopGame,
  PauseGame,
  TriggerSignup,
  FinishSignup,
  Login,
  TriggerInvitation,
  FinishInvitation,
}

pub struct BrowserSystem {
  inputs: InputsReader<GameInput>,
  receiver: BrowserReceiver<Message>,
  controller: BrowserController<Message>,
}

impl Initializable for BrowserSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let receiver = inventory.get::<BrowserReceiver<Message>>().clone();
    let controller = inventory.get::<BrowserController<Message>>().clone();
    Self {
      inputs,
      receiver,
      controller,
    }
  }
}

impl System for BrowserSystem {
  fn get_name(&self) -> &'static str {
    "BrowserSystem"
  }

  fn run(&mut self, _scene: &mut Scene, _backpack: &mut Backpack) {
    let input = self.inputs.read();
    if input.left_click {
      self.controller.send(Message::StartGame);
    }

    if input.escape {
      self.controller.send(Message::StopGame);
    }
  }
}
