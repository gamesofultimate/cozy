use crate::shared::components::{
  TimeOfDay,
  Pickup,
  Action,
  Log,
};
use engine::{
  application::{
    components::{TextComponent, LightComponent},
    scene::{Scene, Collision},
  },
  systems::{
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Seconds, Framerate, Radians},
};
use std::f32::consts::PI;

pub struct PickupsSystem {
}

impl Initializable for PickupsSystem {
  fn initialize(_: &Inventory) -> Self {

    Self { }
  }
}

impl PickupsSystem {
}

impl System for PickupsSystem {
  fn get_name(&self) -> &'static str {
    "PickupsSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    let mut sun_inclination = Radians::new(0.0);

    // TODO: Should be running 4 times the speed of normal time

    //for (_, (log, _)) in scene.query_mut::<(&mut Log, &Collision<Action, Pickup>)>() {
    for (_, _) in scene.query_mut::<&Collision<Action, Pickup>>() {
      log::info!("log");
    }
  }
}
