use crate::shared::components::{
  Action, ActionTypes, Character, CharacterState, Crop, CropTile, CropType, Harvestable,
  Inventory as GameInventory, Item, Level, Log, Pickup, PickupSpace, Quantity, SalesBin, Seeds,
  Stage, Tile, TimeOfDay, WaterCan, WaterSource, WateredTile,
};
use crate::shared::game_input::{GameInput, InputState};
use crate::shared::state_machine::{GameState, StateMachine};
use crate::shared::ui_components::InventoryDisplay;
use engine::{
  application::{
    components::{
      LightComponent, ModelComponent, NetworkedPlayerComponent, ParentComponent, SelfComponent,
      TextComponent,
    },
    scene::{Collision, CollisionEnter, CollisionExit, IdComponent, Scene, TransformComponent},
  },
  nalgebra::Vector3,
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::{Framerate, Radians, Seconds},
};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct PickupsSystem {}

impl Initializable for PickupsSystem {
  fn initialize(inventory: &Inventory) -> Self {
    Self {}
  }
}

impl PickupsSystem {
  pub fn handle_select_action(&self, scene: &mut Scene) {
    for (_, (input, character)) in scene.query_mut::<(&GameInput, &mut Character)>() {
      if input.check(InputState::ChangeActionUp) {
        character.action = match character.action {
          ActionTypes::WaterTile => ActionTypes::ThrowSeed,
          ActionTypes::ThrowSeed => ActionTypes::Harvest,
          ActionTypes::Harvest => ActionTypes::WaterTile,
        };
      }
      if input.check(InputState::ChangeActionDown) {
        character.action = match character.action {
          ActionTypes::WaterTile => ActionTypes::Harvest,
          ActionTypes::Harvest => ActionTypes::ThrowSeed,
          ActionTypes::ThrowSeed => ActionTypes::WaterTile,
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

    for (_, (model, _, _)) in scene.query_mut::<(
      &mut ModelComponent,
      &WaterSource,
      &CollisionEnter<Action, WaterSource>,
    )>() {
      model.color = Vector3::new(0.0, 0.0, 1.0);
      model.color_intensity = 0.1;
    }

    for (_, (model, _, _)) in scene.query_mut::<(
      &mut ModelComponent,
      &WaterSource,
      &CollisionExit<Action, WaterSource>,
    )>() {
      model.color_intensity = 0.0;
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

    for (_, (model, _, _)) in scene
      .query_mut::<(&mut ModelComponent, &Tile, &CollisionEnter<Action, Tile>)>()
      .without::<WateredTile>()
    {
      model.color = Vector3::new(1.0, 1.0, 0.0);
      model.color_intensity = 0.1;
    }

    for (_, (model, _, _)) in scene
      .query_mut::<(&mut ModelComponent, &Tile, &Collision<Action, Tile>)>()
      .without::<WateredTile>()
    {
      model.color_intensity = 0.0;
    }
  }

  pub fn handle_throw_seeds(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap();

    for (_, (input, character, state, seeds, collision)) in scene
      .query_mut::<(
        &GameInput,
        &mut Character,
        &mut CharacterState,
        &mut Seeds,
        &Collision<Action, Tile>,
      )>()
      .without::<CropTile>()
    {
      if input.check(InputState::Action)
        && let ActionTypes::ThrowSeed = character.action
        && seeds.pumpkins >= 1
        && let CharacterState::Normal | CharacterState::Running = state
      {
        *state =
          CharacterState::ThrowingSeed(collision.other, Level::to_max(1.0, Seconds::new(4.0)));
        seeds.pumpkins -= 1;
      }
    }

    let mut working_tile = None;
    for (_, (input, character)) in scene.query_mut::<(&GameInput, &mut CharacterState)>() {
      if let CharacterState::ThrowingSeed(entity, timing) = character {
        if let Some(_) = timing.tick() {
          working_tile = Some(*entity);
          *character = CharacterState::Normal;
        }
      }
    }

    {
      if let Some(tile_entity) = working_tile
        && let Some(prefabs) = scene.get_prefab_owned(CropType::Pumpkin.get_prefab())
        && let Some((mut parent, _)) = prefabs
          .iter()
          .cloned()
          .find(|(prefab, _)| prefab.tag.name == CropType::Pumpkin.get_prefab())
        && let Some((mut prefab, _)) = prefabs
          .iter()
          .cloned()
          .find(|(prefab, _)| prefab.tag.name == "Seeds")
        && let Some(transform) = scene
          .get_components_mut::<&TransformComponent>(tile_entity)
          .cloned()
      {
        scene.add_component(tile_entity, CropTile {});
        let crop_entity = scene.create_raw_entity("Pumpkin Crop");
        prefab.transform = transform;
        prefab.remove::<ParentComponent>();
        scene.create_with_prefab(crop_entity, parent);
        scene.create_with_prefab(crop_entity, prefab);
      }
    }

    for (_, (model, _, _)) in scene
      .query_mut::<(&mut ModelComponent, &Tile, &CollisionEnter<Action, Tile>)>()
      .without::<CropTile>()
    {
      model.color = Vector3::new(0.0, 1.0, 1.0);
      model.color_intensity = 0.1;
    }
    for (_, (model, _, _)) in scene
      .query_mut::<(&mut ModelComponent, &Tile, &CollisionExit<Action, Tile>)>()
      .without::<CropTile>()
    {
      model.color_intensity = 0.0;
    }
  }

  pub fn handle_harvest(&self, scene: &mut Scene, backpack: &mut Backpack) {
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
        *state = CharacterState::Harvesting(collision.other, Level::to_max(1.0, Seconds::new(4.0)));
      }
    }

    let mut harvesting_entities = vec![];
    for (player_entity, (transform, input, character)) in
      scene.query_mut::<(&TransformComponent, &GameInput, &mut CharacterState)>()
    {
      if let CharacterState::Harvesting(harvesting_entity, timing) = character
        && let Some(_) = timing.tick()
      {
        harvesting_entities.push((player_entity, *harvesting_entity, transform.clone()));
        *character = CharacterState::Normal;
      }
    }

    for (player_entity, harvesting_entity, player_transform) in harvesting_entities {
      let mut is_showoff = false;

      let crop = match scene.get_components_mut::<&Crop>(harvesting_entity) {
        Some(data) => data.clone(),
        None => continue,
      };

      if let Some((inventory, character)) =
        scene.get_components_mut::<(&mut GameInventory, &mut CharacterState)>(player_entity)
      {
        is_showoff = inventory.award(Item::Crop(crop.crop), crop.award);
        if is_showoff {
          *character = CharacterState::ShowingOff {
            item: Item::Crop(crop.crop),
          };
        }
      }

      let _ = scene.despawn(harvesting_entity);

      if is_showoff
        && let Some(prefabs) = scene.get_prefab_owned(crop.crop.get_prefab())
        && let Some((mut parent, _)) = prefabs
          .iter()
          .cloned()
          .find(|(prefab, _)| prefab.tag.name == crop.crop.get_prefab())
        && let Some((mut prefab, _)) = prefabs
          .iter()
          .cloned()
          .find(|(prefab, _)| prefab.tag.name == Stage::Display.get_prefab())
      {
        let crop_entity = scene.create_raw_entity("Display");

        prefab.transform = player_transform;
        prefab.transform.translation -= Vector3::z();
        prefab.transform.translation -= Vector3::y() * 0.25;

        prefab.remove::<ParentComponent>();
        if let Some(crop) = parent.get_mut::<Crop>() {
          crop.stage = Stage::Display;
        }
        scene.create_with_prefab(crop_entity, parent);
        scene.create_with_prefab(crop_entity, prefab);
      }
    }

    for (_, (model, _, _)) in scene.query_mut::<(
      &mut ModelComponent,
      &Harvestable,
      &CollisionEnter<Action, Harvestable>,
    )>() {
      model.color = Vector3::new(1.0, 0.0, 1.0);
      model.color_intensity = 0.1;
    }
    for (_, (model, _, _)) in scene.query_mut::<(
      &mut ModelComponent,
      &Harvestable,
      &CollisionExit<Action, Harvestable>,
    )>() {
      model.color_intensity = 0.0;
    }
  }

  pub fn handle_sales(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap();

    /*
      let mut harvesting_entities = vec![];
      for (player_entity, (transform, input, character)) in
        scene.query_mut::<(&TransformComponent, &GameInput, &mut CharacterState)>()
      {
        if let CharacterState::Harvesting(harvesting_entity, timing) = character
          && let Some(_) = timing.tick()
        {
          harvesting_entities.push((player_entity, *harvesting_entity, transform.clone()));
          *character = CharacterState::Normal;
        }
      }

      for (player_entity, harvesting_entity, player_transform) in harvesting_entities {
        let mut is_showoff = false;

        let crop = match scene.get_components_mut::<&Crop>(harvesting_entity) {
          Some(data) => data.clone(),
          None => continue,
        };

        if let Some((inventory, character)) =
          scene.get_components_mut::<(&mut GameInventory, &mut CharacterState)>(player_entity)
        {
          is_showoff = inventory.award(Item::Crop(crop.crop), crop.award);
          if is_showoff {
            *character = CharacterState::ShowingOff {
              item: Item::Crop(crop.crop),
            };
          }
        }

        let _ = scene.despawn(harvesting_entity);

        if is_showoff
          && let Some(prefabs) = scene.get_prefab_owned(crop.crop.get_prefab())
          && let Some((mut parent, _)) = prefabs
            .iter()
            .cloned()
            .find(|(prefab, _)| prefab.tag.name == crop.crop.get_prefab())
          && let Some((mut prefab, _)) = prefabs
            .iter()
            .cloned()
            .find(|(prefab, _)| prefab.tag.name == Stage::Display.get_prefab())
        {
          let crop_entity = scene.create_raw_entity("Display");

          prefab.transform = player_transform;
          prefab.transform.translation -= Vector3::z();
          prefab.transform.translation -= Vector3::y() * 0.25;

          prefab.remove::<ParentComponent>();
          if let Some(crop) = parent.get_mut::<Crop>() {
            crop.stage = Stage::Display;
          }
          scene.create_with_prefab(crop_entity, parent);
          scene.create_with_prefab(crop_entity, prefab);
        }
      }
    */

    for (_, (model, _, _)) in scene.query_mut::<(
      &mut ModelComponent,
      &SalesBin,
      &CollisionEnter<Action, SalesBin>,
    )>() {
      model.color = Vector3::new(0.0, 0.0, 1.0);
      model.color_intensity = 0.1;
    }
    for (_, (model, _, _)) in scene.query_mut::<(
      &mut ModelComponent,
      &SalesBin,
      &CollisionExit<Action, SalesBin>,
    )>() {
      model.color_intensity = 0.0;
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

  pub fn handle_update_ui(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let (character, seeds, inventory, maybe_water) = match scene.query_one::<(
      &SelfComponent,
      &Character,
      &Seeds,
      &GameInventory,
      Option<&WaterCan>,
    )>() {
      Some((entity, (_, character, seeds, inventory, water))) => (
        character.clone(),
        seeds.clone(),
        inventory.clone(),
        water.cloned(),
      ),
      None => return,
    };

    if let Some((_, (text, _))) = scene.query_one::<(&mut TextComponent, &InventoryDisplay)>() {
      if let Some(machine) = backpack.get_mut::<StateMachine>() {
        if machine.is_active() {
          text.opacity = lerp(text.opacity, 1.0, 0.9);
        } else {
          text.opacity = lerp(text.opacity, 0.0, 0.9);
        }
      }
      let mut texts = vec![];

      if let Some(water) = maybe_water {
        texts.push(format!("Water: {:03.2}%", water.level.percent() * 100.0));
      }

      texts.push(format!("---"));
      for (item, quantity) in inventory.items {
        match quantity {
          Quantity::Infinite => texts.push(format!("{:}", &item)),
          Quantity::Empty => texts.push(format!("{:}", &item)),
          Quantity::Finite(quantity) => texts.push(format!("{:} ({:})", &item, quantity)),
        }
      }
      texts.push(format!("---"));
      texts.push(format!("Action"));
      texts.push(match character.action {
        ActionTypes::WaterTile => format!("Water tile"),
        ActionTypes::ThrowSeed => format!("Plant pumpkin"),
        ActionTypes::Harvest => format!("Harvest"),
      });

      texts.push(format!("---"));

      texts.push(format!("Cash: {:}", character.cash));
      text.text = texts.join("\n");
    }
  }

  pub fn handle_plant_growth(&self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap().clone();
    //panic!("Continue here");
    let mut growing_crops = vec![];
    for (entity, (transform, crop)) in scene.query_mut::<(&TransformComponent, &mut Crop)>() {
      crop.phase_timing += delta_time;
      match crop.stage {
        Stage::Seeds => {
          if crop.phase_timing > crop.seed_timeout {
            growing_crops.push((entity.clone(), transform.clone(), crop.crop, crop.stage));
            crop.phase_timing = Seconds::new(0.0);
          }
        }
        Stage::Seedling => {
          if crop.phase_timing > crop.seedling_timeout {
            growing_crops.push((entity.clone(), transform.clone(), crop.crop, crop.stage));
            crop.phase_timing = Seconds::new(0.0);
          }
        }
        Stage::Flowering => {
          if crop.phase_timing > crop.flowering_timeout {
            growing_crops.push((entity.clone(), transform.clone(), crop.crop, crop.stage));
            crop.phase_timing = Seconds::new(0.0);
          }
        }
        Stage::Mature => {
          // NOTE: Implement rotten crop
          /*
          if crop.phase_timing > crop.mature_timeout {
            growing_crops.push((entity.clone(), transform.clone(), crop.crop, crop.stage));
          }
          */
        }
        Stage::Display => {}
      };
    }

    for (entity, transform, crop, stage) in growing_crops {
      if let Some(prefabs) = scene.get_prefab_owned(crop.get_prefab())
        && let Some((mut parent, _)) = prefabs
          .iter()
          .cloned()
          .find(|(prefab, _)| prefab.tag.name == crop.get_prefab())
        && let Some((mut prefab, _)) = prefabs
          .iter()
          .cloned()
          .find(|(prefab, _)| prefab.tag.name == stage.get_next_stage().get_prefab())
      {
        let _ = scene.despawn(entity);
        let crop_entity = scene.create_raw_entity("Crop");
        prefab.transform = transform;
        prefab.remove::<ParentComponent>();
        if let Some(crop) = parent.get_mut::<Crop>() {
          crop.stage = stage.get_next_stage();
        }
        scene.create_with_prefab(crop_entity, parent);
        scene.create_with_prefab(crop_entity, prefab);
      }
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
    self.handle_throw_seeds(scene, backpack);
    self.handle_harvest(scene, backpack);
    self.handle_plant_growth(scene, backpack);
    self.handle_update_ui(scene, backpack);
  }
}

fn lerp(a: f32, b: f32, percent: f32) -> f32 {
  a * percent + b * (1.0 - percent)
}
