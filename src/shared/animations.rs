use engine::{
  application::{animation::AnimationTransition, components::PhysicsComponent, scene::Scene},
  systems::{Backpack, Registry},
  utils::units::Kph,
  Entity,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tagged::{Duplicate, Registerable, Schema};

use super::components::CharacterState;

pub struct AnimationTransitions {}

impl Registry for AnimationTransitions {
  fn register() {
    use engine::application::animation::animation_transition_registry::Access;

    IsCelebrating::register();
    IsDoneCelebrating::register();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct IsCelebrating {}

impl AnimationTransition for IsCelebrating {
  fn should_transition(&self, entity: Entity, scene: &mut Scene, _: &Backpack) -> bool {
    if let Some(character) = scene.get_components_mut::<&mut CharacterState>(entity)
      && let CharacterState::ShowingOff { .. } = character
    {
      true
    } else {
      false
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct IsDoneCelebrating {}

impl AnimationTransition for IsDoneCelebrating {
  fn should_transition(&self, entity: Entity, scene: &mut Scene, _: &Backpack) -> bool {
    if let Some(character) = scene.get_components_mut::<&mut CharacterState>(entity)
      && let CharacterState::Normal = character
    {
      true
    } else {
      false
    }
  }
}
