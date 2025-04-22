use engine::{
  application::scene::{PrefabId, ProvideAssets},
  resources::{audio::AudioId, model::ModelId, particles::ParticleId, sprite::SpriteId},
  systems::Registry,
  utils::{
    easing::Easing,
    interpolation::Interpolator,
    units::{Decibels, Degrees, Kph, Meters, Mps, Rps, Seconds},
  },
  nalgebra::{Unit, Vector3},
};
use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct GameComponents;

impl Registry for GameComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;
    Movement::register();
    Pickup::register();
    Player::register();
    //Tileset::register();
    CameraFollow::register();
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

/*
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Tileset {
  pub width: u32,
  pub length: u32,
}

impl ProvideAssets for Tileset {
    fn provide_assets(&self, ids: &mut Vec<Uuid>) {
        //ids.push(*self.grenadier_throw_grenade);
        //ids.push(*self.grenadier_grenade_explosion);
    }
}
*/
