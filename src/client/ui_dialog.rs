use crate::shared::components::Conversation;
use crate::shared::ui_components::UIDialogText;
use engine::{
  application::{components::TextComponent, scene::Scene},
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
  Entity,
};

pub struct UIDialogSystem {}

impl Initializable for UIDialogSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl UIDialogSystem {
  fn get_current_conversation(&mut self, scene: &mut Scene) -> Option<(Entity, Conversation)> {
    for (entity, conversation) in scene.query_mut::<&Conversation>() {
      if conversation.active {
        return Some((entity, conversation.clone()));
      }
    }

    return None;
  }
}

impl UIDialogSystem {
  fn hide_dialog(&mut self, scene: &mut Scene) {
    let query = scene.query_mut::<(&UIDialogText, &mut TextComponent)>();
    for (_, (_, text)) in query {
      text.opacity = 0.0;
    }
  }

  fn display_dialog(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = *backpack.get::<Seconds>().unwrap();

    let conversation = self.get_current_conversation(scene);
    let (conversation_entity, conversation) = match conversation {
      Some((entity, conversation)) => (entity, conversation),
      None => return,
    };

    if conversation.index >= conversation.dialog.len() {
      let conversation = scene
        .get_components_mut::<&mut Conversation>(conversation_entity)
        .unwrap();
      conversation.active = false;
      return;
    }

    let current_dialog = conversation.dialog.get(conversation.index).unwrap();
    let query = scene.query_mut::<(&UIDialogText, &mut TextComponent)>();

    for (_, (_dialog, text)) in query {
      text.opacity = 1.0;
      text.text = current_dialog.text.clone();
    }

    let conversation = scene
      .get_components_mut::<&mut Conversation>(conversation_entity)
      .unwrap();

    conversation.current_duration_timer += delta_time;
    if conversation.current_duration_timer >= current_dialog.duration {
      conversation.current_duration_timer = Seconds::zero();
      conversation.index += 1;
    }
  }
}

impl System for UIDialogSystem {
  fn get_name(&self) -> &'static str {
    "UIDialogSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.hide_dialog(scene);
    self.display_dialog(scene, backpack);
  }
}
