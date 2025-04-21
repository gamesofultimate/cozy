use crate::shared::components::{Interactable, Interaction, Objective, Player};
use crate::shared::ui_components::{UIInteractable, UIObjective, UIObjectiveType};
use engine::{
  application::{
    components::{SelfComponent, SpriteComponent, TextComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
};

use engine::application::scene::Collision;

pub struct UIInteractableSystem {
  first_run: bool,
}

impl Initializable for UIInteractableSystem {
  fn initialize(_: &Inventory) -> Self {
    Self { first_run: true }
  }
}

impl UIInteractableSystem {
  fn cache_interactable(&mut self, scene: &mut Scene) {
    for (_, (maybe_sprite, maybe_text, interactable)) in scene.query_mut::<(
      Option<&SpriteComponent>,
      Option<&TextComponent>,
      &mut UIInteractable,
    )>() {
      if let Some(sprite) = maybe_sprite {
        interactable.cached_opacity = sprite.opacity;
      }

      if let Some(text) = maybe_text {
        interactable.cached_opacity = text.opacity;
      }

      self.first_run = false;
    }
  }

  fn handle_objective(&mut self, scene: &mut Scene) {
    let mut current_objective = None;

    for (_, (objective, interactable, _collision)) in
      scene.query_mut::<(&Objective, &Interactable, &Collision<Player, Interactable>)>()
    {
      if interactable.ignore {
        continue;
      }

      current_objective = Some(objective.clone());
    }

    let objective = match current_objective {
      Some(objective) => objective,
      None => return,
    };

    for (_, (text, ui_objective)) in scene.query_mut::<(&mut TextComponent, &UIObjective)>() {
      if ui_objective.section != UIObjectiveType::Description {
        continue;
      }

      text.text = objective.text.clone();
    }
  }

  fn display_interactable(&mut self, scene: &mut Scene) {
    let mut is_interacting = false;
    for (_, (_, interactable)) in
      scene.query_mut::<(&Collision<Player, Interactable>, &Interactable)>()
    {
      if interactable.ignore {
        continue;
      }

      is_interacting = interactable.action == Interaction::ActionKey;
    }

    let mut player_is_interacting = false;
    for (_, (_, _, _)) in
      scene.query_mut::<(&Collision<Player, Interactable>, &Player, &SelfComponent)>()
    {
      player_is_interacting = is_interacting;
    }

    for (_, (maybe_sprite, maybe_text, interactable)) in scene.query_mut::<(
      Option<&mut SpriteComponent>,
      Option<&mut TextComponent>,
      &UIInteractable,
    )>() {
      if let Some(sprite) = maybe_sprite {
        sprite.opacity = if player_is_interacting {
          interactable.cached_opacity
        } else {
          0.0
        };
      }

      if let Some(text) = maybe_text {
        text.opacity = if player_is_interacting {
          interactable.cached_opacity
        } else {
          0.0
        };
      }
    }
  }
}

impl System for UIInteractableSystem {
  fn get_name(&self) -> &'static str {
    "UIInteractableSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    if self.first_run {
      self.cache_interactable(scene);
    }

    self.handle_objective(scene);
    self.display_interactable(scene);
  }
}
