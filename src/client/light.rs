use crate::shared::components::{LightInterpolation, Lockdown, LockdownAffected};

use engine::{
  application::{components::LightComponent, scene::Scene},
  systems::{Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
  glm,
};

pub struct LightSystem {}

impl Initializable for LightSystem {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {}
  }
}

impl LightSystem {
  fn interpolate_lights(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().unwrap().clone();

    let query = scene.query_mut::<(&mut LightComponent, &mut LightInterpolation)>();

    for (_, (light, interpolation)) in query {
      let mut resolved_radiance = interpolation.start_radiance;

      if interpolation.enabled {
        interpolation.current_timer += delta_time;

        if interpolation.current_timer > interpolation.frequency {
          interpolation.current_timer -= interpolation.frequency;
        }

        let current_timer = *interpolation.current_timer;
        let frequency = *interpolation.frequency;

        let mut lerp_timer = (current_timer * 2.0) / frequency;

        if lerp_timer <= 1.0 {
          resolved_radiance = glm::lerp(
            &interpolation.start_radiance,
            &interpolation.end_radiance,
            lerp_timer,
          );
        } else {
          lerp_timer -= 1.0;
          resolved_radiance = glm::lerp(
            &interpolation.end_radiance,
            &interpolation.start_radiance,
            lerp_timer,
          );
        }
      }

      if let LightComponent::Point { radiance, .. } = light {
        *radiance = resolved_radiance;
      }

      if let LightComponent::Spot { radiance, .. } = light {
        *radiance = resolved_radiance;
      }
    }
  }

  fn lockdown_lights(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let lockdown = match backpack.get::<Lockdown>().cloned() {
      Some(lockdown) => lockdown,
      None => return,
    };

    if lockdown.state {
      return;
    }

    let query = scene.query_mut::<(&mut LightComponent, &mut LockdownAffected)>();

    for (_, (light, _)) in query {
      if let LightComponent::Point { intensity, .. } = light {
        *intensity = 0.0;
      }

      if let LightComponent::Spot { intensity, .. } = light {
        *intensity = 0.0;
      }
    }
  }
}

impl System for LightSystem {
  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.interpolate_lights(scene, backpack);
    self.lockdown_lights(scene, backpack);
  }
}
