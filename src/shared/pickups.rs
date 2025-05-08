use crate::shared::components::{
  Action, Character, CharacterState, Log, Pickup, PickupSpace, Tile, TimeOfDay, WaterCan,
  WaterSource, WateredTile,
  Seeds, ActionTypes,
};
use crate::shared::game_input::{GameInput, InputState};
use crate::shared::ui_components::InventoryDisplay;
use engine::{
  application::{
    components::{
      LightComponent, ModelComponent, NetworkedPlayerComponent, ParentComponent, SelfComponent,
      TextComponent,
    },
    scene::{Collision, IdComponent, Scene, TransformComponent},
  },
  nalgebra::Vector3,
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::{Framerate, Radians, Seconds},
};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct PickupsSystem {}

impl Initializable for PickupsSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl PickupsSystem {
  pub fn handle_select_action(&self, scene: &mut Scene) {
    for (_, (input, character)) in
      scene.query_mut::<(&GameInput, &mut Character)>()
    {
      if input.check(InputState::ChangeActionUp) {
        character.action = match character.action {
          ActionTypes::WaterTile => ActionTypes::PlantSeed,
          ActionTypes::PlantSeed => ActionTypes::WaterTile,
        };
      }
      if input.check(InputState::ChangeActionDown) {
        character.action = match character.action {
          ActionTypes::WaterTile => ActionTypes::PlantSeed,
          ActionTypes::PlantSeed => ActionTypes::WaterTile,
        };
      }
    }
  }

  pub fn handle_pickup(&self, scene: &mut Scene) {
    let mut spaces = HashMap::new();
    for (_, (id, network, _)) in
      scene.query_mut::<(&IdComponent, &NetworkedPlayerComponent, &PickupSpace)>()
    {
      spaces.insert(*network.connection_id, *id);
    }

    let mut insertions = vec![];
    //for (_, (log, _)) in scene.query_mut::<(&mut Log, &Collision<Action, Pickup>)>() {
    for (_, (id, input, network, collision)) in scene.query_mut::<(
      &IdComponent,
      &GameInput,
      &NetworkedPlayerComponent,
      &Collision<Action, Pickup>,
    )>() {
      if input.check(InputState::Action)
        && let Some(id) = spaces.get(&network.connection_id)
      {
        //pickups.insert(network.connection_id, *id, collision.other);
        insertions.push((collision.other, *id));
      }
    }

    for (entity, parent_id) in insertions {
      scene.add_component(entity, ParentComponent::new(*parent_id));
      scene.add_component(entity, TransformComponent::default());
    }

    for (_, (model, _, maybe_collision)) in
      scene.query_mut::<(&mut ModelComponent, &Tile, Option<&Collision<Action, Tile>>)>()
    {
      if let Some(_) = maybe_collision {
        model.color = Vector3::new(1.0, 1.0, 0.0);
        model.color_intensity = 0.1;
      } else {
        model.color_intensity = 0.0;
      }
    }
  }

  pub fn handle_water_sources(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap();

    for (_, (input, character, can, collision)) in scene.query_mut::<(
      &GameInput,
      &mut CharacterState,
      &mut WaterCan,
      &Collision<Action, WaterSource>,
    )>() {
      if input.check(InputState::Action)
        && let CharacterState::Normal | CharacterState::Running = character
      {
        *character = CharacterState::CollectingWater;
        can.level.maximize_with_rate(0.125);
      }
    }

    for (_, (input, character, can)) in
      scene.query_mut::<(&GameInput, &mut CharacterState, &mut WaterCan)>()
    {
      if let CharacterState::CollectingWater = character {
        if let Some(_) = can.level.tick() {
          *character = CharacterState::Normal;
        }
      }
    }

    for (_, (model, _, maybe_collision)) in scene.query_mut::<(
      &mut ModelComponent,
      &WaterSource,
      Option<&Collision<Action, WaterSource>>,
    )>() {
      if let Some(_) = maybe_collision {
        model.color = Vector3::new(0.0, 0.0, 1.0);
        model.color_intensity = 0.1;
      } else {
        model.color_intensity = 0.0;
      }
    }
  }

  pub fn handle_watering_tiles(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap();

    for (_, (input, character, state, can, collision)) in scene
      .query_mut::<(
        &GameInput,
        &mut Character,
        &mut CharacterState,
        &mut WaterCan,
        &Collision<Action, Tile>,
      )>()
      .without::<WateredTile>()
    {
      if input.check(InputState::Action)
        && let ActionTypes::WaterTile = character.action
        && can.level.current >= 1.0
        && let CharacterState::Normal | CharacterState::Running = state
      {
        *state = CharacterState::WorkingTile(collision.other);
        can.level.change_by(-1.0, Seconds::new(4.0));
      }
    }

    let mut working_tile = None;
    for (_, (input, character, can)) in
      scene.query_mut::<(&GameInput, &mut CharacterState, &mut WaterCan)>()
    {
      if let CharacterState::WorkingTile(entity) = character {
        if let Some(_) = can.level.tick() {
          working_tile = Some(*entity);
          *character = CharacterState::Normal;
        }
      }
    }

    {
      if let Some(tile_entity) = working_tile
        && let Some(prefab) = scene.get_parent_prefab_owned("Prefab::Wet Dirt")
        && let Some(model) = prefab.get::<ModelComponent>()
      {
        scene.add_component(tile_entity, WateredTile {});
        scene.add_component(tile_entity, model.clone());
      }
    }

    for (_, (model, _, maybe_collision)) in scene
      .query_mut::<(&mut ModelComponent, &Tile, Option<&Collision<Action, Tile>>)>()
      .without::<WateredTile>()
    {
      if let Some(_) = maybe_collision {
        model.color = Vector3::new(1.0, 1.0, 0.0);
        model.color_intensity = 0.1;
      } else {
        model.color_intensity = 0.0;
      }
    }
  }

  pub fn handle_add_state(&self, scene: &mut Scene) {
    let mut entities = vec![];
    for (entity, _) in scene.query_mut::<&Character>().without::<CharacterState>() {
      entities.push(entity);
    }

    for entity in entities {
      scene.add_local_component(entity, CharacterState::Normal);
    }
  }

  pub fn handle_update_ui(&self, scene: &mut Scene) {
    let (character, seeds, maybe_water) =
      match scene.query_one::<(&SelfComponent, &Character, &Seeds, Option<&WaterCan>)>() {
        Some((entity, (_, character, seeds, water))) => (character.clone(), seeds.clone(), water.cloned()),
        None => return,
      };

    if let Some((_, (text, _))) = scene.query_one::<(&mut TextComponent, &InventoryDisplay)>() {
      let mut texts = vec![];

      if let Some(water) = maybe_water {
        texts.push(format!("Water: {:03.2}%", water.level.percent() * 100.0));
      }

      texts.push(format!("---"));
      texts.push(format!("Seeds"));
      texts.push(format!("Pumpking: {:}", seeds.pumpkins));
      texts.push(format!("---"));
      texts.push(format!("Action"));
      texts.push(match character.action {
        ActionTypes::WaterTile => format!("Water tile"),
        ActionTypes::PlantSeed => format!("Plant pumpkin"),
      });

      texts.push(format!("---"));

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
    self.handle_select_action(scene);
    self.handle_add_state(scene);
    self.handle_pickup(scene);
    self.handle_water_sources(scene, backpack);
    self.handle_watering_tiles(scene, backpack);
    self.handle_update_ui(scene);
  }
}
