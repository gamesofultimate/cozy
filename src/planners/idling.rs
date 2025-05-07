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
  Character,
  Movement,
};

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

  fn cost(&self, _: &Backpack, _: &Blackboard) -> f32 {
    9999.0
  }

  fn check_readyness(&mut self, _: &Backpack, _: &Blackboard) -> bool {
    true
  }

  fn apply_effect(&mut self, _: &mut Backpack, blackboard: &mut Blackboard) {
    blackboard.insert_bool("bored", true);
  }

  fn within_range(&mut self, _: &Backpack, _: Option<Arc<Navmesh>>) -> bool {
    false
  }

  fn move_towards(
    &mut self,
    _: Entity,
    _: &mut Scene,
    _: &mut Backpack,
    _: &mut Backpack,
    _: Option<Arc<Navmesh>>,
  ) -> Option<(Vector3<f32>, Vector3<f32>)> {
    let linear_velocity = Vector3::y() * -9.8;
    let angular_velocity = Vector3::zeros();

    return Some((linear_velocity, angular_velocity));
  }

  fn execute(&mut self, _: Entity, _: &mut Scene, _: &mut Backpack, _: &mut Backpack) {
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
    blackboard.get_bool("tired") && blackboard.get_bool("found_resting_place")
  }

  fn apply_effect(&mut self, _: &mut Backpack, blackboard: &mut Blackboard) {
    blackboard.insert_bool("tired", false);
    blackboard.insert_bool("rested", true);
    blackboard.insert_bool("bored", true);
    blackboard.insert_bool("sitting", true);
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
      && let Some(character) = scene.get_components_mut::<&mut Character>(entity)
    {
      character.rest.add(*resting_factor);
    }
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
    _: Option<Arc<Navmesh>>,
  ) {
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
        blackboard.insert_bool("found_resting_place", true);
      }
      None => {
        blackboard.insert_bool("found_resting_place", false);
        local.take::<SeatLocation>();
      }
    }
  }
}
