use crate::client::game_input::GameInput;
use crate::shared::components::{Health, Player};
use crate::shared::ui_components::UIRespawn;
use engine::{
  application::{
    components::{SelfComponent, SpriteComponent, TextComponent},
    scene::Scene,
  },
  systems::{input::InputsReader, Backpack, Initializable, Inventory, System},
};

pub struct UIRespawnSystem {
  is_dead: bool,
  inputs: InputsReader<GameInput>,
}

impl Initializable for UIRespawnSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();

    Self {
      is_dead: false,
      inputs,
    }
  }
}

impl UIRespawnSystem {
  fn display_respawn(&mut self, scene: &mut Scene, input: GameInput) {
    let mut timer = 0.0;
    for (_, (player, health, _)) in scene.query_mut::<(&Player, &Health, &SelfComponent)>() {
      self.is_dead = health.current_health <= 0.0;
      timer = *player.death_transition_timer / 2.0;
    }

    for (_, (maybe_sprite, maybe_text, _respawn)) in scene.query_mut::<(
      Option<&mut SpriteComponent>,
      Option<&mut TextComponent>,
      &UIRespawn,
    )>() {
      if input.mouse_lock {
        if let Some(sprite) = maybe_sprite {
          sprite.opacity = timer; //if self.is_dead { timer /*1.0*/ } else { 0.0 };
        }

        if let Some(text) = maybe_text {
          text.opacity = timer; // if self.is_dead { timer /*1.0*/ } else { 0.0 };
        }
      }
    }

    for (_, (maybe_sprite, maybe_text)) in scene
      .query_mut::<(Option<&mut SpriteComponent>, Option<&mut TextComponent>)>()
      .without::<UIRespawn>()
    {
      if input.mouse_lock {
        if let Some(sprite) = maybe_sprite {
          if timer > 0.0 {
            sprite.opacity = 0.0;
          }
        }

        if let Some(text) = maybe_text {
          if timer > 0.0 {
            text.opacity = 0.0;
          }
        }
      }
    }
  }
}

impl System for UIRespawnSystem {
  fn get_name(&self) -> &'static str {
    "UIRespawnSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    let input = self.inputs.read();

    self.display_respawn(scene, input);
  }
}
