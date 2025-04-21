use crate::shared::components::{Direction, Door, DoorSensor, DoorState, Player};
use engine::{
  application::{
    components::PhysicsComponent,
    scene::{Collision, Scene, TransformComponent},
  },
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};
use nalgebra::Vector3;

pub struct DoorSystem {
  physics: PhysicsController,
}

impl Initializable for DoorSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    Self { physics }
  }
}

impl DoorSystem {
  fn detect_collision(&mut self, scene: &mut Scene, delta_time: Seconds) {
    let mut doors = vec![];
    for (_, (sensor, _)) in scene.query_mut::<(&DoorSensor, &Collision<Player, DoorSensor>)>() {
      doors.push(sensor.door);
    }

    for door in doors {
      let entity = match scene.get_entity(door) {
        Some(data) => data,
        None => continue,
      };
      let (transform, physics, door) =
        match scene
          .get_components_mut::<(&TransformComponent, &PhysicsComponent, &mut Door)>(*entity)
        {
          Some(data) => data,
          None => continue,
        };

      door.door_state = match door {
        Door { door_state, .. } if *door_state == DoorState::Closed => DoorState::Opening {
          origin: transform.translation,
        },
        Door {
          door_state: DoorState::Opening { origin },
          offset: Direction::X { offset },
          opening_speed,
          ..
        } => {
          let movement = Vector3::x() * *offset * **opening_speed * *delta_time;
          let next_position = transform.translation + movement;
          self
            .physics
            .set_kinematic_translation(&physics, next_position);
          if (next_position.x - origin.x).abs() > *offset {
            DoorState::Open
          } else {
            DoorState::Opening { origin: *origin }
          }
        }
        _ => continue,
      }
    }
  }
}

impl System for DoorSystem {
  fn get_name(&self) -> &'static str {
    "DoorSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    self.detect_collision(scene, delta_time);
  }
}
