use crate::shared::components::{
  Ball,
  Goal,
  GoalId,
};
use engine::application::components::ModelComponent;
use engine::application::components::ParentComponent;
use engine::application::components::TextComponent;
use engine::{
  rapier3d::prelude::{QueryFilter, Ray},
  application::{
    components::{CameraComponent, InputComponent, PhysicsComponent, SelfComponent},
    physics3d::RigidBodyHandle,
    scene::{Collision, IdComponent, Scene, TransformComponent},
    input::InputsReader, 
  },
  resources::{node::Transform, particles::ParticleId},
  systems::{
    controller::AudioController, physics::PhysicsController,
    controller::ParticleController, Backpack, Initializable, Inventory, System,
  },
  utils::physics,
  utils::units::{Decibels, Degrees, Meters, Mps, Seconds},
  Entity,
  nalgebra::{Point3, Rotation3, Unit, Vector3},
};
use rand::Rng;
use std::f32::consts::PI;
use std::mem::variant_count;
use uuid::Uuid;

pub struct ScoringSystem {
  physics: PhysicsController,
  team1_score: usize,
  team2_score: usize,
}

impl Initializable for ScoringSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();

    Self {
      physics,
      team1_score: 0,
      team2_score: 0,
    }
  }
}

impl ScoringSystem {
}

impl System for ScoringSystem {
  fn get_name(&self) -> &'static str {
    "ScoringSystem"
  }

  fn attach(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (entity, (goal, _)) in scene.query_mut::<(&Goal, &Collision<Ball, Goal>)>() {
      match goal.team {
        GoalId::Team1 => {
          self.team2_score += 1;
        }
        GoalId::Team2 => {
          self.team1_score += 1;
        }
      }
    }

    for (entity, score) in scene.query_mut::<&mut TextComponent>() {
      score.text = format!("{:} v {:}", self.team1_score, self.team2_score);
    }
  }
}
