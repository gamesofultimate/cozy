use engine::{
  application::scene::{PrefabId, ProvideAssets},
  systems::Registry,
  utils::{
    units::Kph,
  },
  nalgebra::{Unit, Vector3},
};
use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};

pub struct GameComponents;

impl Registry for GameComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;
    Movement::register();
    Pickup::register();
    Player::register();
    CameraFollow::register();
    Npc::register();
    Flower::register();
    Seat::register();
    TimeOfDay::register();
    Action::register();
    Log::register();
    PickupSpace::register();
    HouseEntrance::register();
  }
}

fn default_direction() -> Unit<Vector3<f32>> { Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0)) }
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

impl ProvideAssets for Pickup {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Player {
}

impl ProvideAssets for Player {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Npc {
  pub rest_level: f32,
  pub total_energy: f32,
  pub social_level: f32,
  pub total_social: f32,
}

impl ProvideAssets for Npc {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Flower {
}

impl ProvideAssets for Flower {}

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
pub struct Action {
}

impl ProvideAssets for Action {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Log {
}

impl ProvideAssets for Log {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct PickupSpace {
}

impl ProvideAssets for PickupSpace {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct HouseEntrance {
  pub owner: PrefabId,
}

impl ProvideAssets for HouseEntrance {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Friend {
}

impl ProvideAssets for Friend {}
