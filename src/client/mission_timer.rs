use crate::shared::components::MissionTimer;
use crate::shared::ui_components::UIMissionTimer;
use engine::utils::units::Seconds;
use engine::{
  application::{
    components::{SelfComponent, TextComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
};

pub struct MissionTimerSystem {}

impl Initializable for MissionTimerSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl MissionTimerSystem {
  fn display_timer(&mut self, scene: &mut Scene) {
    let mut player_mission_timer = None;
    for (_, (timer, _)) in scene.query_mut::<(&MissionTimer, &SelfComponent)>() {
      player_mission_timer = Some(timer.clone());
    }

    if let Some(player_mission_timer) = player_mission_timer {
      for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UIMissionTimer)>() {
        let player_percent_minutes = (*player_mission_timer.current_time / 60.0) as u32;
        let player_percent_seconds =
          (*player_mission_timer.current_time - ((player_percent_minutes * 60) as f32)) as u32;
        text.text = format!("{:}:{:}", player_percent_minutes, player_percent_seconds);
      }
    }
  }

  fn timer_spend(&mut self, scene: &mut Scene, delta_time: Seconds) {
    for (_, (timer, _)) in scene.query_mut::<(&mut MissionTimer, &SelfComponent)>() {
      timer.current_time -= delta_time;
    }
  }

  fn hide_ui(&mut self, scene: &mut Scene) {
    let mut hide_ui = false;
    for (_, timer) in scene.query_mut::<&MissionTimer>() {
      if *timer.current_time < 0.0 {
        hide_ui = true;
      }
    }
    for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UIMissionTimer)>() {
      if hide_ui {
        text.opacity = 0.0;
      }
    }
  }
}

impl System for MissionTimerSystem {
  fn get_name(&self) -> &'static str {
    "MissionTimerSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    self.timer_spend(scene, delta_time);
    self.display_timer(scene);
    self.hide_ui(scene);
  }
}
