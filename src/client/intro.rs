use crate::shared::components::{Interactable, IntroTrigger, Player};
use engine::application::{
  renderer3d::Renderer3d,
  scene::{Collision, Scene, TransformComponent},
};
use engine::systems::{Backpack, Initializable, Inventory, RenderingSystem};

pub struct IntroSystem {}

impl Initializable for IntroSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl IntroSystem {
  fn handle_player_destination(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let mut destination = None;
    for (entity, (trigger, interactable)) in scene.query_mut::<(&mut IntroTrigger, &Interactable)>()
    {
      if !interactable.has_activated {
        continue;
      }

      destination = Some((entity, trigger.auto_player_destination));
    }

    let (trigger_entity, destination_prefab_id) = match destination {
      Some(destination) => destination,
      None => return,
    };

    let destination_entity = scene.get_entity(destination_prefab_id).unwrap();

    let destination = scene
      .get_components_mut::<&TransformComponent>(*destination_entity)
      .unwrap()
      .clone();

    for (_, (player, collision)) in
      scene.query_mut::<(&mut Player, &Collision<Player, Interactable>)>()
    {
      if collision.other == trigger_entity {
        player.auto_destination = Some(destination.translation);
      }
    }
  }

  fn handle_blur(
    &mut self,
    scene: &mut Scene,
    _backpack: &mut Backpack,
    renderer: &mut Renderer3d,
  ) {
    let mut config = renderer.get_config();

    for (_, (player, _transform)) in scene.query_mut::<(&mut Player, &TransformComponent)>() {
      if player.auto_destination.is_some() {
        config.dof.enabled = true;
        config.dof.focus_scale = config.dof.radius_scale;
      } else {
        config.dof.enabled = false;
        config.dof.focus_scale = 0.0;
      }
    }

    renderer.set_config(config);
  }
}

impl RenderingSystem for IntroSystem {
  fn get_name(&self) -> &'static str {
    "IntroSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack, renderer: &mut Renderer3d) {
    self.handle_player_destination(scene, backpack);
    self.handle_blur(scene, backpack, renderer);
  }
}
