use std::f32::consts::PI;
use std::collections::HashMap;
use crate::shared::game_input::{GameInput, InputState};
use crate::shared::ui_components::InventoryDisplay;
use crate::shared::components::{
  TimeOfDay,
  Pickup,
  Action,
  Log,
  PickupSpace,
  Character,
  Tile,
  WaterCan,
  WaterSource,
  CharacterState,
};
use engine::{
  application::{
    components::{TextComponent, LightComponent, SelfComponent, NetworkedPlayerComponent, ParentComponent, ModelComponent},
    scene::{Scene, Collision, IdComponent, TransformComponent},
  },
  systems::{
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Seconds, Framerate, Radians},
  nalgebra::Vector3,
};

pub struct PickupsSystem {
}

impl Initializable for PickupsSystem {
  fn initialize(_: &Inventory) -> Self {

    Self {
    }
  }
}

impl PickupsSystem {
  pub fn handle_pickup(&self, scene: &mut Scene) {
    let mut spaces = HashMap::new();
    for (_, (id, network, _)) in scene.query_mut::<(&IdComponent, &NetworkedPlayerComponent, &PickupSpace)>() {
      spaces.insert(*network.connection_id, *id);
    }

    let mut insertions = vec![];
    //for (_, (log, _)) in scene.query_mut::<(&mut Log, &Collision<Action, Pickup>)>() {
    for (_, (id, input, network, collision)) in scene.query_mut::<(&IdComponent, &GameInput, &NetworkedPlayerComponent, &Collision<Action, Pickup>)>() {
      if input.check(InputState::Action) && let Some(id) = spaces.get(&network.connection_id) {
        //pickups.insert(network.connection_id, *id, collision.other);
        insertions.push((collision.other, *id));
      }
    }

    for (entity, parent_id) in insertions {
      scene.add_component(entity, ParentComponent::new(*parent_id));
      scene.add_component(entity, TransformComponent::default());
    }


    /*
    for (_, (model, _, maybe_collision)) in scene.query_mut::<(&mut ModelComponent, &Tile, Option<&Collision<Action, Tile>>)>() {
      if let Some(_) = maybe_collision {
        model.color = Vector3::new(1.0, 1.0, 1.0);
        model.color_intensity = 0.1;
      } else {
        model.color_intensity = 0.0;
      }
    }
    */
  }

  pub fn handle_water_sources(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap();

    for (_, (input, character, can, collision)) in scene.query_mut::<(&GameInput, &mut Character, &mut WaterCan, &Collision<Action, WaterSource>)>() {
      if input.check(InputState::Action) && let CharacterState::Normal | CharacterState::Running = character.state {
        character.state = CharacterState::CollectingWater;
      }
    }

    for (_, (input, character, can)) in scene.query_mut::<(&GameInput, &mut Character, &mut WaterCan)>() {
      if let CharacterState::CollectingWater = character.state {
        can.level.add(1.0 * **delta_time);
      }

      if can.level.percent() >= 1.0 {
        character.state = CharacterState::Normal;
      }
    }

    for (_, (model, _, maybe_collision)) in scene.query_mut::<(&mut ModelComponent, &WaterSource, Option<&Collision<Action, WaterSource>>)>() {
      if let Some(_) = maybe_collision {
        model.color = Vector3::new(0.0, 0.0, 1.0);
        model.color_intensity = 0.1;
      } else {
        model.color_intensity = 0.0;
      }
    }
  }

  pub fn handle_update_ui(&self, scene: &mut Scene) {
    let (character, maybe_water) = match scene.query_one::<(&SelfComponent, &Character, Option<&WaterCan>)>() {
      None => return,
      Some((entity, (_, character, water))) => (character.clone(), water.cloned()),
    };

    if let Some((_, (text, _))) = scene.query_one::<(&mut TextComponent, &InventoryDisplay)>() {
      let mut texts = vec![];

      if let Some(water) = maybe_water {
        texts.push(format!("Water: {:03.2}%", water.level.percent() * 100.0));
      }

      texts.push(format!("Cash: {:}", character.cash));
      text.text = texts.join("\n");
    }
  }
}

impl System for PickupsSystem {
  fn get_name(&self) -> &'static str {
    "PickupsSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_pickup(scene);
    self.handle_water_sources(scene, backpack);
    self.handle_update_ui(scene);
  }
}
