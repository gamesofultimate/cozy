use crate::shared::components::{
  TimeOfDay,
};
use engine::{
  application::{
    components::{ModelComponent, CameraComponent, InputComponent, PhysicsComponent, SelfComponent, TextComponent, LightComponent},
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
  utils::units::{Decibels, Degrees, Meters, Mps, Seconds, Framerate, Radians},
  Entity,
  nalgebra::{Point3, Rotation3, Unit, Vector3},
};
use std::fmt::Display;
use std::f32::consts::PI;
use std::mem::variant_count;
use uuid::Uuid;

pub struct TimeOfDaySystem {
}

impl Initializable for TimeOfDaySystem {
  fn initialize(inventory: &Inventory) -> Self {

    Self {
    }
  }
}

impl TimeOfDaySystem {
}

impl System for TimeOfDaySystem {
  fn get_name(&self) -> &'static str {
    "TimeOfDaySystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut sun_inclination = Radians::new(0.0);

    // TODO: Should be running 4 times the speed of normal time

    if let Some((entity, (time_of_day, text))) = scene.query_one::<(&mut TimeOfDay, &mut TextComponent)>() {
      time_of_day.current_time = (time_of_day.current_time + time_of_day.delta_time * *Seconds::from(Framerate::new(60.0))) % time_of_day.total_time;
      let hour = time_of_day.current_time as u32 / 100;
      let minute = (60.0 * ((time_of_day.current_time % 100.0) / 100.0)) as u32;

      let percent = time_of_day.get_percent();
      sun_inclination = Radians::new((PI * 2.0) * percent + PI);

      text.text = format!("{:02}:{:02} {:}", hour, minute, if time_of_day.current_time > 1200.0 { "pm" } else { "am" });
    }

    for (entity, light) in scene.query_mut::<&mut LightComponent>() {
      if let LightComponent::Directional { inclination, .. } = light {
        *inclination = sun_inclination;
      }
    }
  }
}
