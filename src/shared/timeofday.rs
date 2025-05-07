use crate::planners::social::{FriendLocation, Friends};
use crate::shared::components::{Friend, TimeOfDay};
use engine::{
  application::{
    components::{LightComponent, TextComponent},
    scene::{IdComponent, Scene, TransformComponent},
  },
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::{Framerate, Radians, Seconds},
};
use std::f32::consts::PI;

pub struct TimeOfDaySystem {}

impl Initializable for TimeOfDaySystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl TimeOfDaySystem {
  pub fn position_sun(&mut self, scene: &mut Scene) {
    let mut sun_inclination = Radians::new(0.0);

    // TODO: Should be running 4 times the speed of normal time

    if let Some((_, (time_of_day, text))) =
      scene.query_one::<(&mut TimeOfDay, &mut TextComponent)>()
    {
      time_of_day.current_time = (time_of_day.current_time
        + time_of_day.delta_time * *Seconds::from(Framerate::new(60.0)))
        % time_of_day.total_time;

      let hour = time_of_day.get_hours();
      let minute = time_of_day.get_minutes();
      let percent = time_of_day.get_percent();

      sun_inclination = Radians::new((PI * 2.0) * percent + PI);

      text.text = format!(
        "{:02}:{:02} {:}",
        hour,
        minute,
        if time_of_day.current_time > time_of_day.total_time / 2.0 {
          "pm"
        } else {
          "am"
        }
      );
    }

    for (_, light) in scene.query_mut::<&mut LightComponent>() {
      if let LightComponent::Directional { inclination, .. } = light {
        *inclination = sun_inclination;
      }
    }
  }

  pub fn friends_map(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut friends = backpack
      .entry::<Friends>()
      .or_insert_with(|| Friends::new());

    for (_, (id, transform, home)) in
      scene.query_mut::<(&IdComponent, &TransformComponent, &Friend)>()
    {
      friends.insert(**id, transform.translation);
    }
  }
}

impl System for TimeOfDaySystem {
  fn get_name(&self) -> &'static str {
    "TimeOfDaySystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.position_sun(scene);
    self.friends_map(scene, backpack);
  }
}
