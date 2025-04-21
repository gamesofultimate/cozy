use engine::{
  application::{animation::AnimationTransition, components::PhysicsComponent, scene::Scene},
  systems::{Backpack, Registry},
  utils::units::Kph,
};
use hecs::Entity;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tagged::{Duplicate, Registerable, Schema};

use super::components::{AnimatedCamera, Enemy, Gun, Health, Player};

use crate::shared::components::LoadingCameraTrack;

pub struct AnimationTransitions {}

impl Registry for AnimationTransitions {
  fn register() {
  }
}

