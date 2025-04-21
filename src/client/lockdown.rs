use crate::shared::components::{
  Interactable, Interaction, Lockdown, LockdownDisabler, LockdownEnabler, Player,
};

use crate::client::GameInput;

use engine::{
  application::scene::{Collision, Scene},
  systems::{input::InputsReader, Backpack, Initializable, Inventory, System},
};

pub struct LockdownSystem {
  inputs: InputsReader<GameInput>,
  lockdown_active: bool,
}

impl Initializable for LockdownSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    Self {
      inputs,
      lockdown_active: true,
    }
  }
}

impl LockdownSystem {
  fn handle_lockdown_disabler_collision(&mut self, scene: &mut Scene) {
    if !self.lockdown_active {
      return;
    }

    let input = self.inputs.read();
    for (_, (lockdown, interactable, _collision)) in scene.query_mut::<(
      &mut LockdownDisabler,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if lockdown.has_activated {
        continue;
      }

      if interactable.ignore {
        continue;
      }

      let switch = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !switch {
        continue;
      }

      interactable.has_activated = true;

      lockdown.has_activated = true;
      self.lockdown_active = false;
    }
  }

  fn handle_lockdown_enabler_collision(&mut self, scene: &mut Scene) {
    if self.lockdown_active {
      return;
    }

    let input = self.inputs.read();
    for (_, (lockdown, interactable, _collision)) in scene.query_mut::<(
      &mut LockdownEnabler,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if lockdown.has_activated {
        continue;
      }

      if interactable.ignore {
        continue;
      }

      let switch = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !switch {
        continue;
      }

      interactable.has_activated = true;

      lockdown.has_activated = true;
      self.lockdown_active = true;
    }
  }
}

impl System for LockdownSystem {
  fn get_name(&self) -> &'static str {
    "LockdownSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_lockdown_disabler_collision(scene);
    self.handle_lockdown_enabler_collision(scene);

    backpack.insert(Lockdown {
      state: self.lockdown_active,
    });
  }
}
