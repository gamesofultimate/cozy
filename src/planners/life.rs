use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tagged::{Duplicate, Registerable, Schema};

use engine::{
  application::{
    goap::{Action, Blackboard, Goal, Sensor},
    scene::{Scene, IdComponent, TransformComponent},
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
  HouseEntrance,
  TimeOfDay,
};

pub struct HomeLocation {
  translation: Vector3<f32>,
  distance: Meters,
}

pub struct LifeRegistry {}

impl Registry for LifeRegistry {
  fn register() {
    {
      use engine::application::goap::goal_registry::Access;
      Sleep::register();
    }
    {
      use engine::application::goap::action_registry::Access;
      GoToSleep::register();
    }
    {
      use engine::application::goap::sensor_registry::Access;
      SenseTimeOfDay::register();
      SenseSelf::register();
      SenseHome::register();
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

/*
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
*/

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct Sleep {}
impl Goal for Sleep {
  fn name(&self) -> &'static str {
    "Sleep"
  }

  fn get_goal(&self, _: Entity, _: &mut Scene, _: &mut Backpack) -> Blackboard {
    let mut blackboard = Blackboard::new();
    // NOTE: An idle goal doesn't make much sense to me. Other things should lead into an idle state
    blackboard.insert_bool("sleepy", false);
    blackboard
  }
}

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
      && let Some(npc) = scene.get_components_mut::<&mut Npc>(entity)
    {
      //npc.rest_level = (npc.rest_level + resting_factor).min(npc.total_energy);
      npc.rest_level = (npc.rest_level + 0.01).min(npc.total_energy);
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
pub struct SenseTimeOfDay {
}

impl Sensor for SenseTimeOfDay {
  fn name(&self) -> &'static str {
    "SenseTimeOfDay"
  }

  fn sense(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _: &mut Backpack,
    local: &mut Backpack,
    blackboard: &mut Blackboard,
  ) {
    if let Some((_, time_of_day)) = scene.query_one::<&mut TimeOfDay>() {
      let hour = time_of_day.get_hours();

      if hour > 22 || hour < 6 { 
        blackboard.insert_bool("sleepy", true);
      } else if hour > 6 || hour < 8 {
        blackboard.insert_bool("get-ready", true);
        blackboard.insert_bool("socialize", true);
      } else if hour > 8 || hour < 16 {
        blackboard.insert_bool("work", true);
      } else if hour > 16 || hour < 18 {
        blackboard.insert_bool("wind-down", true);
        blackboard.insert_bool("socialize", true);
      } else if hour > 18 || hour < 23 {
      } else {
        blackboard.insert_bool("sleepy", false);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Registerable, Duplicate)]
pub struct SenseHome {
}

impl Sensor for SenseHome {
  fn name(&self) -> &'static str {
    "SenseHome"
  }

  fn sense(
    &mut self,
    entity: Entity,
    scene: &mut Scene,
    _: &mut Backpack,
    local: &mut Backpack,
    blackboard: &mut Blackboard,
  ) {
    let (id, entity_transform) = match scene.get_components_mut::<(&IdComponent, &TransformComponent)>(entity) {
      Some((id, transform)) => (*id, transform.clone()),
      None => return,
    };

    for (_, (transform, home)) in scene.query_mut::<(&TransformComponent, &HouseEntrance)>() {
      if *id == home.owner {
        let distance = Vector3::metric_distance(
          &entity_transform.translation,
          &transform.translation,
        );

        local.insert(HomeLocation { translation: transform.translation, distance: Meters::new(distance) });
      }
    }
  }
}
