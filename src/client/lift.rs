use crate::shared::components::{
  Direction, Generator, GeneratorPowered, Lift, LiftSensor, LiftState, LiftType, Lockdown,
  LockdownAffected, Player,
};
use engine::application::scene::PrefabId;
use engine::{
  application::{
    components::PhysicsComponent,
    scene::{Collision, Scene, TransformComponent},
  },
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};
use nalgebra::Vector3;

pub struct LiftSystem {
  physics: PhysicsController,
}

impl Initializable for LiftSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    Self { physics }
  }
}

impl LiftSystem {
  fn get_generator(&mut self, scene: &mut Scene) -> Option<Generator> {
    let query = scene.query_mut::<&Generator>();

    let mut generator_clone = None;
    for (_, generator) in query {
      generator_clone = Some(generator.clone());
    }

    generator_clone
  }

  fn detect_collision(&mut self, scene: &mut Scene) -> (Vec<PrefabId>, Vec<PrefabId>) {
    let mut lifts = vec![];
    let mut inactive_lifts = vec![];

    for (_, sensor) in scene
      .query_mut::<&LiftSensor>()
      .without::<Collision<Player, LiftSensor>>()
    {
      inactive_lifts.push(sensor.lift);
    }

    for (_, (sensor, _)) in scene.query_mut::<(&LiftSensor, &Collision<Player, LiftSensor>)>() {
      lifts.push(sensor.lift);
    }

    return (lifts, inactive_lifts);
  }

  fn move_lifts(
    &mut self,
    scene: &mut Scene,
    active_lifts: Vec<PrefabId>,
    inactive_lifts: Vec<PrefabId>,
    delta_time: f32,
    backpack: &mut Backpack,
  ) {
    let maybe_lockdown = backpack.get::<Lockdown>();
    let maybe_generator = self.get_generator(scene);

    for lift in inactive_lifts {
      let entity = match scene.get_entity(lift) {
        Some(data) => data,
        None => continue,
      };

      let (transform, physics, lift, maybe_lockdown_affected, maybe_generator_powered) = match scene
        .get_components_mut::<(
          &TransformComponent,
          &PhysicsComponent,
          &mut Lift,
          Option<&LockdownAffected>,
          Option<&GeneratorPowered>,
        )>(*entity)
      {
        Some(data) => data,
        None => continue,
      };

      match (&maybe_generator, maybe_generator_powered) {
        (Some(generator), Some(_generator_powered)) => {
          if generator.active == false {
            continue;
          }
        }
        (_, _) => {}
      }

      if let Some(_lockdown_affected) = maybe_lockdown_affected {
        if let Some(lockdown) = maybe_lockdown
          && lockdown.state
        {
          continue;
        }
      }

      if lift.lift_type == LiftType::OneWayOnly {
        continue;
      }

      if lift.lift_type == LiftType::ReturnOnLeave {
        lift.lift_state = match lift {
          Lift {
            lift_state: LiftState::Stopped1,
            ..
          } => LiftState::Moving1 {
            origin: transform.translation,
          },
          Lift {
            lift_state: LiftState::Moving1 { origin },
            offset: Direction::Y { offset },
            moving_speed,
            ..
          } => {
            let movement = Vector3::y() * *offset * **moving_speed * delta_time;
            let next_position = transform.translation - movement;
            self
              .physics
              .set_kinematic_translation(&physics, next_position);
            if (next_position.y - origin.y).abs() > *offset {
              LiftState::Stopped0
            } else {
              LiftState::Moving1 { origin: *origin }
            }
          }
          _ => continue,
        };
      } else if lift.lift_type == LiftType::MoveOnEnter && lift.deactivate == true {
        lift.deactivate = false;
        lift.lift_state = LiftState::Stopped0;
        lift.direction_val *= -1;
      }
    }

    for lift in active_lifts {
      let entity = match scene.get_entity(lift) {
        Some(data) => data,
        None => continue,
      };

      let (transform, physics, lift, maybe_lockdown_affected, maybe_generator_powered) = match scene
        .get_components_mut::<(
          &TransformComponent,
          &PhysicsComponent,
          &mut Lift,
          Option<&LockdownAffected>,
          Option<&GeneratorPowered>,
        )>(*entity)
      {
        Some(data) => data,
        None => continue,
      };

      match (&maybe_generator, maybe_generator_powered) {
        (Some(generator), Some(_generator_powered)) => {
          if generator.active == false {
            continue;
          }
        }
        (_, _) => {}
      }

      if let Some(_lockdown_affected) = maybe_lockdown_affected {
        if let Some(lockdown) = maybe_lockdown
          && lockdown.state
        {
          continue;
        }
      }

      if lift.lift_type == LiftType::MoveOnEnter {
        if lift.direction_val != 1 && lift.direction_val != -1 {
          lift.direction_val = 1;
        }
      } else {
        lift.direction_val = 1;
        lift.deactivate = false;
      }
      if lift.deactivate == false {
        lift.lift_state = match lift {
          Lift {
            lift_state: LiftState::Stopped0,
            ..
          } => LiftState::Moving0 {
            origin: transform.translation,
          },
          Lift {
            lift_state: LiftState::Moving0 { origin },
            offset: Direction::Y { offset },
            moving_speed,
            ..
          } => {
            let direction = lift.direction_val as f32;
            let movement = direction * Vector3::y() * *offset * **moving_speed * delta_time;
            let next_position = transform.translation + movement;
            self
              .physics
              .set_kinematic_translation(&physics, next_position);
            if (next_position.y - origin.y).abs() > *offset {
              LiftState::Stopped1
            } else {
              LiftState::Moving0 { origin: *origin }
            }
          }
          _ => continue,
        };
      }
      if (lift.lift_state == LiftState::Stopped1 || lift.lift_state == LiftState::Stopped0)
        && lift.deactivate == false
      {
        lift.deactivate = true;
      }
    }
  }
}

impl System for LiftSystem {
  fn get_name(&self) -> &'static str {
    "LiftSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    let (active, inactive) = self.detect_collision(scene);
    self.move_lifts(scene, active, inactive, *delta_time, backpack);
  }
}
