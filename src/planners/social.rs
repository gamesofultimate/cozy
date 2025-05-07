use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Iter, HashMap};
use std::sync::Arc;
use tagged::{Duplicate, Registerable, Schema};

use engine::{
  application::{
    goap::{Action, Blackboard, Goal, Sensor},
    scene::{IdComponent, PrefabId, Scene, TransformComponent},
  },
  nalgebra::{Unit, Vector3},
  resources::navmesh::Navmesh,
  systems::{Backpack, Registry},
  utils::{
    physics,
    units::{Meters, Rps},
  },
  Entity,
};

use crate::shared::components::{Character, Friend, HouseEntrance, Movement, Seat, TimeOfDay};

pub struct Friends {
  data: HashMap<PrefabId, FriendLocation>,
}

pub struct FriendLocation {
  location: Vector3<f32>,
  interacting_with: Option<(PrefabId, Meters)>,
}

impl Friends {
  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
    }
  }

  pub fn insert(&mut self, id: PrefabId, location: Vector3<f32>) {
    self.data.entry(id).or_insert(FriendLocation {
      location,
      interacting_with: None,
    });
  }

  pub fn iter(&self) -> Iter<'_, PrefabId, FriendLocation> {
    self.data.iter()
  }
}

pub struct SocialRegistry {}

impl Registry for SocialRegistry {
  fn register() {
    {
      use engine::application::goap::goal_registry::Access;
      Socialize::register();
    }
    {
      use engine::application::goap::action_registry::Access;
      //GoToSleep::register();
    }
    {
      use engine::application::goap::sensor_registry::Access;
      SenseSocialNeed::register();
      SenseFriends::register();
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct Socialize {}
impl Goal for Socialize {
  fn name(&self) -> &'static str {
    "Socialize"
  }

  fn get_goal(&self, _: Entity, _: &mut Scene, _: &mut Backpack) -> Blackboard {
    let mut blackboard = Blackboard::new();
    blackboard.insert_bool("want-to-socialize", true);
    blackboard
  }
}

/*
#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct GoToSleep {}

impl Action for GoToSleep {
  fn name(&self) -> &'static str {
    "SitDown"
  }

  fn cost(&self, local: &Backpack, blackboard: &Blackboard) -> f32 {
    if blackboard.get_bool("sleepy") {
      0.0
    } else {
      300.0
    }
  }

  fn check_readyness(&mut self, _local: &Backpack, blackboard: &Blackboard) -> bool {
    // We always know where home is for now
    blackboard.get_bool("sleepy")
  }

  fn apply_effect(&mut self, _: &mut Backpack, blackboard: &mut Blackboard) {
    blackboard.insert_bool("tired", false);
    blackboard.insert_bool("rested", true);
    blackboard.insert_bool("sleepy", false);
    blackboard.insert_bool("laying-down", true);
  }

  fn within_range(&mut self, local: &Backpack, _: Option<Arc<Navmesh>>) -> bool {
    if let Some(seat) = local.get::<HomeLocation>() {
      seat.distance < Meters::new(1.6)
    } else {
      false
    }
  }

  fn move_towards(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _backpack: &mut Backpack,
    local: &mut Backpack,
    _navmesh: Option<Arc<Navmesh>>,
  ) -> Option<(Vector3<f32>, Vector3<f32>)> {
    let HomeLocation { translation, .. } = local.get::<HomeLocation>()?;
    let (transform, movement) = scene.get_components_mut::<(&TransformComponent, &Movement)>(entity)?;

    let mut start_direction = transform.get_forward_direction().into_inner();
    start_direction.y = 0.0;
    let start_direction = Unit::new_normalize(start_direction);

    let mut end_direction = translation - transform.translation;
    end_direction.y = 0.0;
    let end_direction = Unit::new_normalize(end_direction);

    let mut linear_velocity = Vector3::y() * -9.8;
    linear_velocity += *start_direction * *movement.walking_speed;
    let mut angular_velocity =
        physics::directions_to_angular_velocity(start_direction, end_direction, Rps::new(6.0));
    angular_velocity.x = 0.0;
    angular_velocity.z = 0.0;

    return Some((linear_velocity, angular_velocity));
  }

  fn execute(&mut self, entity: Entity, scene: &mut Scene, _: &mut Backpack, local: &mut Backpack) {
    if let Some(HomeLocation { .. }) = local.get::<HomeLocation>()
      && let Some(character) = scene.get_components_mut::<&mut Character>(entity)
    {
      //character.rest_level = (character.rest_level + resting_factor).min(character.total_energy);
      character.rest_level = (character.rest_level + 0.01).min(character.total_energy);
    }
  }
}
*/

// NOTE: Should probably be two sensors: SenseSelf and SenseRest
#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct SenseSocialNeed {}

impl Sensor for SenseSocialNeed {
  fn name(&self) -> &'static str {
    "SenseSocialNeed"
  }

  fn sense(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _: &mut Backpack,
    local: &mut Backpack,
    blackboard: &mut Blackboard,
    _: Option<Arc<Navmesh>>,
  ) {
    match scene.get_components_mut::<(&TransformComponent, &Character)>(entity) {
      Some((transform, character)) => {
        let social_need = character.social.percent();
        if social_need < 0.8 {
          blackboard.insert_bool("want-to-socialize", true);
        } else {
          blackboard.insert_bool("want-to-socialize", false);
        }

        local.insert(transform.clone())
      }
      None => {
        blackboard.insert_bool("want-to-socialize", false);
      }
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct SenseFriends {}

impl Sensor for SenseFriends {
  fn name(&self) -> &'static str {
    "SenseFriends"
  }

  fn sense(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    global: &mut Backpack,
    local: &mut Backpack,
    blackboard: &mut Blackboard,
    _: Option<Arc<Navmesh>>,
  ) {
    if let Some(friends) = global.get_mut::<Friends>() {
      let entity_transform = match scene.get_components_mut::<&TransformComponent>(entity) {
        Some(transform) => transform.clone(),
        None => return,
      };

      // TODO: Find interactions
    }
  }
}
