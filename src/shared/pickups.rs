use crate::shared::components::{
  TimeOfDay,
};
use engine::{
  application::{
    components::{TextComponent, LightComponent},
    scene::Scene,
  },
  systems::{
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Seconds, Framerate, Radians},
};
use std::f32::consts::PI;

pub struct TimeOfDaySystem {
}

impl Initializable for TimeOfDaySystem {
  fn initialize(_: &Inventory) -> Self {

    Self { }
  }
}

impl TimeOfDaySystem {
}

impl System for TimeOfDaySystem {
  fn get_name(&self) -> &'static str {
    "TimeOfDaySystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    let mut sun_inclination = Radians::new(0.0);

    // TODO: Should be running 4 times the speed of normal time

    if let Some((_, (time_of_day, text))) = scene.query_one::<(&mut TimeOfDay, &mut TextComponent)>() {
      time_of_day.current_time = (time_of_day.current_time + time_of_day.delta_time * *Seconds::from(Framerate::new(60.0))) % time_of_day.total_time;
      let hour = time_of_day.current_time as u32 / 100;
      let minute = (60.0 * ((time_of_day.current_time % 100.0) / 100.0)) as u32;

      let percent = time_of_day.get_percent();
      sun_inclination = Radians::new((PI * 2.0) * percent + PI);

      text.text = format!("{:02}:{:02} {:}", hour, minute, if time_of_day.current_time > 1200.0 { "pm" } else { "am" });
    }

    for (_, light) in scene.query_mut::<&mut LightComponent>() {
      if let LightComponent::Directional { inclination, .. } = light {
        *inclination = sun_inclination;
      }
    }
  }
}
