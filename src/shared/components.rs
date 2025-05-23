use engine::{
  application::scene::{PrefabId, ProvideAssets},
  nalgebra::{Unit, Vector3},
  resources::model::ModelId,
  systems::Registry,
  utils::units::{Framerate, Kph, Seconds},
  Entity,
};
use tagged::{Duplicate, Registerable, Schema};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

pub struct GameComponents;

impl Registry for GameComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;
    Movement::register();
    Pickup::register();
    Player::register();
    CameraFollow::register();
    Character::register();
    WaterCan::register();
    WaterSource::register();
    Rock::register();
    Durability::register();
    Crop::register();
    Seeds::register();
    Seat::register();
    TimeOfDay::register();
    Action::register();
    Log::register();
    PickupSpace::register();
    HouseEntrance::register();
    Tile::register();
    Preloader::register();
  }
}

fn default_direction() -> Unit<Vector3<f32>> {
  Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0))
}
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Movement {
  pub walking_speed: Kph,
  pub running_speed: Kph,

  #[serde(skip, default = "default_direction")]
  pub direction: Unit<Vector3<f32>>,
}

impl ProvideAssets for Movement {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct CameraFollow {
  pub following: PrefabId,
}

impl ProvideAssets for CameraFollow {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Pickup {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
enum ChangeDirection {
  Add { want: f32, rate: f32 },
  Remove { want: f32, rate: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Level {
  pub current: f32,
  pub min: f32,
  pub max: f32,

  #[serde(skip)]
  want: Option<ChangeDirection>,
}

impl Level {
  pub fn to_max(max: f32, duration: Seconds) -> Self {
    let diff = max;
    let frames = *duration / *Seconds::from(Framerate::new(16.0));
    let rate = diff / frames;
    let want = Some(ChangeDirection::Add { want: max, rate });

    Self {
      current: 0.0,
      min: 0.0,
      max,
      want,
    }
  }

  pub fn percent(&self) -> f32 {
    self.current / self.max
  }

  pub fn set_want(&mut self, want: f32, duration: Seconds) {
    let diff = want - self.current;
    let frames = *duration / *Seconds::from(Framerate::new(16.0));
    let rate = diff / frames;
    self.want = match diff {
      v if v > 0.0 => Some(ChangeDirection::Add { want, rate }),
      v if v < 0.0 => Some(ChangeDirection::Remove { want, rate }),
      _ => None,
    };
  }

  pub fn change_by(&mut self, diff: f32, duration: Seconds) {
    let frames = *duration / *Seconds::from(Framerate::new(16.0));
    let rate = diff / frames;
    self.want = match diff {
      v if v > 0.0 => Some(ChangeDirection::Add {
        want: self.current + diff,
        rate,
      }),
      v if v < 0.0 => Some(ChangeDirection::Remove {
        want: self.current + diff,
        rate,
      }),
      _ => None,
    };
  }

  pub fn maximize(&mut self, duration: Seconds) {
    let diff = self.max - self.current;
    let frames = *duration / *Seconds::from(Framerate::new(16.0));
    let rate = diff / frames;
    self.want = match diff {
      v if v > 0.0 => Some(ChangeDirection::Add {
        want: self.max,
        rate,
      }),
      v if v < 0.0 => Some(ChangeDirection::Remove {
        want: self.max,
        rate,
      }),
      _ => None,
    };
  }

  pub fn maximize_with_rate(&mut self, rate: f32) {
    let diff = self.max - self.current;
    self.want = match diff {
      v if v > 0.0 => Some(ChangeDirection::Add {
        want: self.max,
        rate,
      }),
      v if v < 0.0 => Some(ChangeDirection::Remove {
        want: self.max,
        rate: -rate,
      }),
      _ => None,
    };
  }

  pub fn minimize(&mut self, duration: Seconds) {
    let diff = self.min - self.current;
    let frames = *duration / *Seconds::from(Framerate::new(16.0));
    let rate = diff / frames;
    self.want = match diff {
      v if v > 0.0 => Some(ChangeDirection::Add {
        want: self.min,
        rate,
      }),
      v if v < 0.0 => Some(ChangeDirection::Remove {
        want: self.min,
        rate,
      }),
      _ => None,
    };
  }

  pub fn add(&mut self, value: f32) {
    self.current = (self.current + value).min(self.max);
  }

  pub fn remove(&mut self, value: f32) {
    self.current = (self.current - value).max(self.min);
  }

  pub fn tick(&mut self) -> Option<()> {
    let (current, want) = match self.want {
      Some(ChangeDirection::Add { want, rate }) => ((self.current + rate).min(want), want),
      // We calculate rate automatically, and if it's a remove, the rate will be negative
      Some(ChangeDirection::Remove { want, rate }) => ((self.current + rate).max(want), want),
      None => return Some(()),
    };
    self.current = current;

    if current == want {
      Some(())
    } else {
      None
    }
  }
}

impl ProvideAssets for Pickup {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Player {}

impl ProvideAssets for Player {}

pub enum CharacterState {
  Normal,
  Running,
  CollectingWater,
  WorkingTile(Entity),
  ThrowingSeed(Entity, Level),
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Character {
  pub cash: u64,
  pub rest: Level,
  pub social: Level,
  pub hunger: Level,
  pub action: ActionTypes,
}

impl ProvideAssets for Character {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum Stage {
  Seeds,
  Seedling,
  Flowering,
  //Sprout,
  Mature,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct WaterCan {
  pub level: Level,
}

impl ProvideAssets for WaterCan {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Crop {
  pub stage: Stage,
  pub season_start: u32,
  pub season_end: u32,
  pub growth_phases: u32,
}

impl ProvideAssets for Crop {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Rock {
  pub health: Level,
}

impl ProvideAssets for Rock {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Durability {}

impl ProvideAssets for Durability {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Seat {
  pub resting_factor: f32,
}

impl ProvideAssets for Seat {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct TimeOfDay {
  pub current_time: f32,
  pub total_time: f32,
  pub delta_time: f32,
}

impl TimeOfDay {
  pub fn get_percent(&self) -> f32 {
    self.current_time / self.total_time
  }

  pub fn get_hours(&self) -> u32 {
    (self.current_time as u32 / 6000) % 12
  }

  pub fn get_minutes(&self) -> u32 {
    (60.0 * ((self.current_time % 6000.0) / 6000.0)) as u32
  }
}

impl ProvideAssets for TimeOfDay {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum SeedTypes {
  Pumpkin,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum ActionTypes {
  WaterTile,
  ThrowSeed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Action {
}

impl ProvideAssets for Action {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Log {}

impl ProvideAssets for Log {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct PickupSpace {}

impl ProvideAssets for PickupSpace {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct HouseEntrance {
  pub owner: PrefabId,
}
impl ProvideAssets for HouseEntrance {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Friend {}
impl ProvideAssets for Friend {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Tile {}

impl ProvideAssets for Tile {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum WaterType {
  Salty,
  Fresh,
}
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct WaterSource {
  water_type: WaterType,
  fill_rate: f32,
}

impl ProvideAssets for WaterSource {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct WateredTile {}

impl ProvideAssets for WateredTile {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct CropTile {}

impl ProvideAssets for CropTile {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Seeds {
  pub pumpkins: usize,
}

impl ProvideAssets for Seeds {}

// NOTE: Anti-pattern. We should revisit this.
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Preloader {
  models: Vec<ModelId>,
}

impl ProvideAssets for Preloader {
  fn provide_assets(&self, ids: &mut Vec<Uuid>) {
    for model in &self.models {
      ids.push(**model);
    }
  }
}
