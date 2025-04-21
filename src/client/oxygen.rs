use crate::shared::components::Health;
use crate::shared::components::Oxygen;
use crate::shared::ui_components::UIOxygen;
use crate::shared::ui_components::UIType;
use engine::application::scene::TransformComponent;
use engine::utils::units::Seconds;
use engine::{
  application::{
    components::{SelfComponent, SpriteComponent, TextComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
};
use nalgebra::Vector4;

pub struct OxygenSystem {}

impl Initializable for OxygenSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl OxygenSystem {
  fn display_oxygen(&mut self, scene: &mut Scene) {
    let mut player_oxygen = None;
    for (_, (oxygen, _)) in scene.query_mut::<(&Oxygen, &SelfComponent)>() {
      player_oxygen = Some(oxygen.clone());
    }

    if let Some(player_oxygen) = player_oxygen {
      for (_, (text, ui_oxygen)) in scene.query_mut::<(&mut TextComponent, &UIOxygen)>() {
        if ui_oxygen.ui_type == UIType::Permanent {
          text.opacity = if player_oxygen.active {
            ui_oxygen.target_opacity
          } else {
            0.0
          };
          continue;
        }

        let player_percent = (player_oxygen.current_oxygen / player_oxygen.total_oxygen) * 100.0;
        if player_percent <= player_oxygen.low_percentage && ui_oxygen.use_colored_text {
          text.color = Vector4::new(1.0, 0.0, 0.0, 1.0);
        } else {
          text.color = Vector4::new(1.0, 1.0, 1.0, 1.0);
        }
        text.text = format!("{:.1}%", player_percent);
      }
      for (_, (sprite, transform, ui_oxygen)) in
        scene.query_mut::<(&mut SpriteComponent, &mut TransformComponent, &UIOxygen)>()
      {
        let player_percent = player_oxygen.current_oxygen / player_oxygen.total_oxygen;
        if ui_oxygen.ui_type == UIType::Permanent {
          if !player_oxygen.active {
            sprite.opacity = 0.0;
            continue;
          }

          transform.scale.x = player_percent * 0.5;
          sprite.opacity = ui_oxygen.target_opacity;
        } else if ui_oxygen.ui_type == UIType::LowMarker {
          if player_percent < 0.20 {
            sprite.opacity = ui_oxygen.target_opacity;
          } else {
            sprite.opacity = 0.0;
          }
        } else {
          if player_percent < ui_oxygen.show_below_oxygen {
            sprite.opacity = ui_oxygen.target_opacity - player_percent;
            transform.scale.x = ui_oxygen.max_scale_multiplier + (sprite.opacity);
            transform.scale.y = ui_oxygen.max_scale_multiplier + (sprite.opacity);
          } else {
            sprite.opacity = 0.0;
          }
        }
      }
    }
  }

  fn oxygen_spend(&mut self, scene: &mut Scene, delta_time: Seconds) {
    for (_, (health, oxygen, _)) in scene.query_mut::<(&mut Health, &mut Oxygen, &SelfComponent)>()
    {
      if !oxygen.active {
        continue;
      }

      oxygen.current_oxygen -= oxygen.oxygen_loss_per_second * *delta_time;

      if oxygen.current_oxygen <= 0.0 {
        oxygen.current_oxygen = 0.0;
        health.current_health -= oxygen.health_loss_per_second * *delta_time;
      }
    }
  }
}

impl System for OxygenSystem {
  fn get_name(&self) -> &'static str {
    "OxygenSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    self.oxygen_spend(scene, delta_time);
    self.display_oxygen(scene);
  }
}
