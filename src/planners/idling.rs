use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tagged::{Duplicate, Registerable, Schema};

use engine::{
  application::{
    goap::{Action, Blackboard, Goal, Sensor},
    scene::{Scene, TransformComponent},
  },
  utils::{physics, units::{Meters, Rps}},
  nalgebra::{Vector3, Unit},
  resources::navmesh::Navmesh,
  systems::{Backpack, Registry},
  Entity,
};

use crate::shared::components::{
  Seat,
  Npc,
  Movement,
};

pub struct Tiredness(f32);
pub struct SeatLocation { translation: Vector3<f32>, distance: Meters, resting_factor: f32 }

pub struct IdleRegistry {}

impl Registry for IdleRegistry {
  fn register() {
    {
      use engine::application::goap::goal_registry::Access;
      Bored::register();
      Rest::register();
    }
    {
      use engine::application::goap::action_registry::Access;
      SitDown::register();
      Nothing::register();
    }
    {
      use engine::application::goap::sensor_registry::Access;
      SenseSeats::register();
      SenseSelf::register();
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct Bored {}
impl Goal for Bored {
  fn name(&self) -> &'static str {
    "Bored"
  }

  fn get_goal(&self, _: Entity, _: &mut Scene, _: &mut Backpack) -> Blackboard {
    let mut blackboard = Blackboard::new();
    // NOTE: An idle goal doesn't make much sense to me. Other things should lead into an idle state
    blackboard.insert_bool("bored", true);
    blackboard
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct Nothing {}

impl Action for Nothing {
  fn name(&self) -> &'static str {
    "Nothing"
  }

  fn cost(&self, local: &Backpack, _: &Blackboard) -> f32 {
    9999.0
  }

  fn check_readyness(&mut self, _local: &Backpack, blackboard: &Blackboard) -> bool {
    true
  }

  fn apply_effect(&mut self, _: &mut Backpack, blackboard: &mut Blackboard) {
    blackboard.insert_bool("bored", true);
  }

  fn within_range(&mut self, local: &Backpack, _: Option<Arc<Navmesh>>) -> bool {
    false
  }

  fn move_towards(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _backpack: &mut Backpack,
    local: &mut Backpack,
    _navmesh: Option<Arc<Navmesh>>,
  ) -> Option<(Vector3<f32>, Vector3<f32>)> {
    let linear_velocity = Vector3::y() * -9.8;
    let angular_velocity = Vector3::zeros();

    return Some((linear_velocity, angular_velocity));
  }

  fn execute(&mut self, entity: Entity, scene: &mut Scene, _: &mut Backpack, local: &mut Backpack) {
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct Rest {}
impl Goal for Rest {
  fn name(&self) -> &'static str {
    "Rest"
  }

  fn get_goal(&self, _: Entity, _: &mut Scene, _: &mut Backpack) -> Blackboard {
    let mut blackboard = Blackboard::new();
    // NOTE: An idle goal doesn't make much sense to me. Other things should lead into an idle state
    blackboard.insert_bool("rested", true);
    blackboard
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct SitDown {}

impl Action for SitDown {
  fn name(&self) -> &'static str {
    "SitDown"
  }

  fn cost(&self, local: &Backpack, _: &Blackboard) -> f32 {
    if let Some(seat) = local.get::<SeatLocation>() {
      *seat.distance
    } else {
      9999.0
    }
  }

  fn check_readyness(&mut self, _local: &Backpack, blackboard: &Blackboard) -> bool {
    blackboard.get_bool("tired")
  }

  fn apply_effect(&mut self, _: &mut Backpack, blackboard: &mut Blackboard) {
    blackboard.insert_bool("tired", false);
    blackboard.insert_bool("rested", true);
    blackboard.insert_bool("bored", true);
    blackboard.insert_bool("sitted", true);
  }

  fn within_range(&mut self, local: &Backpack, _: Option<Arc<Navmesh>>) -> bool {
    if let Some(seat) = local.get::<SeatLocation>() {
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
    let SeatLocation { translation, .. } = local.get::<SeatLocation>()?;
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
    if let Some(SeatLocation { resting_factor, .. }) = local.get::<SeatLocation>()
      && let Some(npc) = scene.get_components_mut::<&mut Npc>(entity)
    {
      npc.rest_level = (npc.rest_level + resting_factor).min(npc.total_energy);
    }
  }
}

// NOTE: Should probably be two sensors: SenseSelf and SenseRest
#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct SenseSelf {
}

impl Sensor for SenseSelf {
  fn name(&self) -> &'static str {
    "SenseSelf"
  }

  fn sense(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _: &mut Backpack,
    local: &mut Backpack,
    blackboard: &mut Blackboard,
  ) {
    match scene.get_components_mut::<(&TransformComponent, &Npc)>(entity) {
      Some((transform, npc)) => {
        let tiredness = npc.rest_level / npc.total_energy;
        if tiredness < 0.3 {
          blackboard.insert_bool("tired", true);
        } else {
          blackboard.insert_bool("tired", false);
        }

        local.insert(transform.clone())
      },
      None => {
        blackboard.insert_bool("tired", false);
      },
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct SenseSeats {
  max_distance: Meters,
}

impl Sensor for SenseSeats {
  fn name(&self) -> &'static str {
    "SenseSeats"
  }

  fn sense(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _: &mut Backpack,
    local: &mut Backpack,
    blackboard: &mut Blackboard,
  ) {
    /*
    if let Some(Some(_)) = scene.get_components_mut::<Option<&Gun>>(entity) {
      local.insert(GunLocation::Inventory);
      blackboard.insert_bool("has_gun", true);
      return;
    }
    */

    let entity_transform = match scene.get_components_mut::<&TransformComponent>(entity) {
      Some(transform) => transform.clone(),
      None => return,
    };

    let mut distance_to_seat = None;
    for (_, (transform, seat)) in scene.query_mut::<(&TransformComponent, &Seat)>() {
      let distance = Vector3::metric_distance(
        &entity_transform.translation,
        &transform.translation,
      );

      if distance > *self.max_distance { continue }

      match distance_to_seat {
        Some((_, current_distance, _)) if distance < current_distance => {
          distance_to_seat = Some((transform.translation, distance, seat.resting_factor))
        }
        None => distance_to_seat = Some((transform.translation, distance, seat.resting_factor)),
        _ => {}
      }
    }

    match distance_to_seat {
      Some((translation, distance, resting_factor)) => {
        local.insert(SeatLocation { translation, distance: Meters::new(distance), resting_factor });
      }
      None => {
        local.take::<SeatLocation>();
      }
    }
  }
}
