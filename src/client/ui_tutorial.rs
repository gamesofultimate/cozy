use crate::shared::components::{Player, TutorialSensor};
use crate::shared::ui_components::UITutorial;
use engine::{
  application::{
    components::{SpriteComponent, TextComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
};

use engine::application::scene::Collision;

pub struct UITutorialSystem {}

impl Initializable for UITutorialSystem {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {}
  }
}

impl UITutorialSystem {
  fn display_tutorial(&mut self, scene: &mut Scene) {
    let mut activate_tutorial = false;
    for (_, (_, _)) in scene.query_mut::<(&TutorialSensor, &Collision<Player, TutorialSensor>)>() {
      activate_tutorial = true;
    }

    for (_, (maybe_sprite, maybe_text, _)) in scene.query_mut::<(
      Option<&mut SpriteComponent>,
      Option<&mut TextComponent>,
      &UITutorial,
    )>() {
      if maybe_sprite.is_some() {
        let sprite = maybe_sprite.unwrap();
        if activate_tutorial {
          sprite.opacity = 1.0;
        } else {
          sprite.opacity = 0.0;
        };
      }

      if maybe_text.is_some() {
        let text = maybe_text.unwrap();
        if activate_tutorial {
          text.opacity = 1.0;
        } else {
          text.opacity = 0.0;
        };
      }
    }
  }
}

impl System for UITutorialSystem {
  fn get_name(&self) -> &'static str {
    "UITutorialSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    self.display_tutorial(scene);
  }
}
