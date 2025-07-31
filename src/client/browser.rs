#![cfg(target_arch = "wasm32")]
use engine::{
  application::{
    components::{NetworkedPlayerComponent, SelfComponent},
    input::InputsReader,
    scene::{Collision, Scene},
  },
  systems::{
    browser::{BrowserController, BrowserReceiver},
    Backpack, Initializable, Inventory, System,
  },
  tsify,
  utils::units::Seconds,
  PlayerId,
};
use serde::{Deserialize, Serialize};

use crate::shared::components::{
  Action, ActionTypes, Character, CharacterState, Harvestable, Item, Quantity,
};
use crate::shared::game_input::{GameInput, InputState};
use crate::shared::state_machine::{GameState, StateMachine};

#[derive(Debug, Serialize, Deserialize, tsify::Tsify)]
pub enum Message {
  StartGame,
  StopGame,
  PauseGame,
  TriggerSignup,
  Login,
  TriggerInvitation,
  FinishInvitation,

  StartSale,
  FinishSignup,
  UpdateStateMachine {
    state: StateMachine,
  },
  UpdateCharacter {
    character: Character,
    state: String,
  },
  UpdateDownloadStats {
    pending_required: usize,
    pending_priority: usize,
    downloaded_required: usize,
    downloaded_priority: usize,
  },
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

impl BrowserSystem {
  fn get_name(&self) -> &'static str {
    "BrowserSystem"
  }

  pub fn handle_browser_messages(&self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let machine = backpack.get_mut::<StateMachine>()?;
    for message in self.receiver.receive() {
      match message {
        Message::StartGame => {
          machine.start_game();
        }
        Message::FinishSignup => {
          for (_, (network, _)) in scene.query_mut::<(&NetworkedPlayerComponent, &SelfComponent)>()
          {
            machine.signup(&network.connection_id, PlayerId::new());
            machine.start_game();
          }
        }
        _ => {}
      }
    }

    Some(())
  }

  pub fn handle_game_start(&self) {
    let input = self.inputs.read_client();

    if input.check(InputState::LeftClick) {
      self.controller.send(Message::StartGame);
    }

    if input.check(InputState::Escape) {
      self.controller.send(Message::StopGame);
    }
  }

  pub fn handle_sales(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap();

    for (_, (input, character, state, collision)) in scene.query_mut::<(
      &GameInput,
      &mut Character,
      &mut CharacterState,
      &Collision<Action, Harvestable>,
    )>() {
      if input.check(InputState::Action)
        && let ActionTypes::Harvest = character.action
        && let CharacterState::Normal | CharacterState::Running = state
      {
        //self.controller.send(Message::StartSale);
      }
    }
  }
}

impl System for BrowserSystem {
  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_browser_messages(scene, backpack);
    self.handle_game_start();
    self.handle_sales(scene, backpack);
  }
}
