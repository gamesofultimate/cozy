#![cfg(target_arch = "wasm32")]
use engine::{
  application::components::PhysicsComponent,
  application::scene::{Collision, Scene},
  systems::{Backpack, Initializable, Inventory, System},
};

use crate::shared::components::{
  AudioFade, Boss, BossSensor, Enemy, FadeSensor, Health, Interactable, Lifecycle, Movement, Player,
};

use engine::application::components::AudioSourceComponent;
use engine::application::components::SourceState;
use engine::utils::units::{Decibels, Seconds};
use nalgebra::Vector3;

pub struct AudioControlSystem {}

impl Initializable for AudioControlSystem {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {}
  }
}

impl AudioControlSystem {
  fn handle_enemy_footsteps(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    for (_, (_, physics, _, maybe_audio, lifecycle)) in scene.query_mut::<(
      &Enemy,
      &PhysicsComponent,
      &Movement,
      Option<&mut AudioSourceComponent>,
      &Lifecycle,
    )>() {
      if let Some(audio) = maybe_audio {
        let linear_velocity = physics.get_linear_velocity();
        let velocity = Vector3::new(linear_velocity.x, 0.0, linear_velocity.z);
        if velocity.magnitude() > 1.0 && !lifecycle.is_dead {
          match audio.state {
            SourceState::Stopped => audio.state = SourceState::Playing,
            _ => {}
          }
        } else {
          audio.state = SourceState::Stopped;
        }
      }
    }
  }

  fn handle_audio_fade_in(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut entities_fade_in = vec![];
    let delta_time = *backpack.get::<Seconds>().unwrap();

    for (entity, (_audio_source, fade)) in
      scene.query_mut::<(&mut AudioSourceComponent, &mut AudioFade)>()
    {
      if fade.fade_in {
        if fade.fade_out_finished {
          entities_fade_in.push((entity, fade.clone()));
        }
      }
    }

    for (_, ref mut fade) in &mut entities_fade_in {
      let entity = match scene.get_entity_mut(fade.fade_in_prefab) {
        Some(data) => data.clone(),
        None => continue,
      };
      let audio_source = scene.get_components_mut::<&mut AudioSourceComponent>(entity);
      let in_audio_source = audio_source.unwrap();

      fade.fade_in_timer += delta_time;
      in_audio_source.state = SourceState::Playing;

      if fade.fade_in_timer < Seconds::new(0.2) {
        in_audio_source.volume = fade.fade_in_start_volume;
      }

      if fade.fade_in_timer <= fade.fade_time {
        in_audio_source.volume += Decibels(fade.fade_volume.to_linear() * *delta_time);
      } else {
        fade.fade_out_finished = false;
      }
    }

    for (ent, fade) in entities_fade_in {
      let audiofade = scene.get_components_mut::<&mut AudioFade>(ent).unwrap();
      *audiofade = fade;
    }
  }

  fn handle_boss_death_fade(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = *backpack.get::<Seconds>().unwrap();
    let mut boss_dead = false;
    for (_, (_, health)) in scene.query_mut::<(&Boss, &mut Health)>() {
      if health.current_health <= 0.1 {
        boss_dead = true;
      }
    }
    for (_, (audio_source, interactable, _, fade)) in scene.query_mut::<(
      &mut AudioSourceComponent,
      &mut Interactable,
      &BossSensor,
      &mut AudioFade,
    )>() {
      if boss_dead {
        fade.activate_timer = true;
      }

      if fade.activate_timer {
        fade.fade_timer += delta_time;
        if fade.fade_timer >= fade.fade_time {
          audio_source.state = SourceState::Stopped;
          interactable.ignore = true;
          fade.fade_out_finished = true;
          fade.activate_timer = false;
        } else {
          audio_source.volume -= Decibels(fade.fade_volume.to_linear() * *delta_time);
          fade.fade_in_start_volume = audio_source.volume;
        }
      }
    }
  }

  fn handle_sensor_audio_fade(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = *backpack.get::<Seconds>().unwrap();
    let mut entities_fade_out = vec![];
    let mut entities_fade_in = vec![];

    for (entity, (_sensor, fade, _collision)) in scene.query_mut::<(
      &mut FadeSensor,
      &mut AudioFade,
      &Collision<Player, FadeSensor>,
    )>() {
      if fade.fade_in == false {
        entities_fade_out.push((entity, fade.clone()));
      } else {
        entities_fade_in.push((entity, fade.clone()));
      }
    }

    for (_, ref mut fade) in &mut entities_fade_out {
      let entity = match scene.get_entity_mut(fade.fade_in_prefab) {
        Some(data) => data.clone(),
        None => continue,
      };
      let audio_source = scene.get_components_mut::<&mut AudioSourceComponent>(entity);
      let in_audio_source = audio_source.unwrap();

      fade.fade_timer += delta_time;

      if fade.fade_timer <= fade.fade_time {
        in_audio_source.volume -= Decibels(fade.fade_volume.to_linear() * *delta_time);
      } else {
        in_audio_source.state = SourceState::Stopped;
      }
    }

    for (ent, fade) in entities_fade_out {
      let audiofade = scene.get_components_mut::<&mut AudioFade>(ent).unwrap();
      *audiofade = fade;
    }

    for (_, ref mut fade) in &mut entities_fade_in {
      let entity = match scene.get_entity_mut(fade.fade_in_prefab) {
        Some(data) => data.clone(),
        None => continue,
      };
      let audio_source = scene.get_components_mut::<&mut AudioSourceComponent>(entity);
      let in_audio_source = audio_source.unwrap();
      in_audio_source.state = SourceState::Playing;

      fade.fade_in_timer += delta_time;

      if fade.fade_in_timer < Seconds::new(0.2) {
        in_audio_source.volume = fade.fade_in_sensor_start_volume;
      }

      if fade.fade_in_timer <= fade.fade_time {
        in_audio_source.volume += Decibels(fade.fade_volume.to_linear() * *delta_time);
      } else {
      }
    }

    for (ent, fade) in entities_fade_in {
      let audiofade = scene.get_components_mut::<&mut AudioFade>(ent).unwrap();
      *audiofade = fade;
    }
  }
}

impl System for AudioControlSystem {
  fn get_name(&self) -> &'static str {
    "AudioControlSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_enemy_footsteps(scene, backpack);
    self.handle_boss_death_fade(scene, backpack);
    self.handle_audio_fade_in(scene, backpack);
    self.handle_sensor_audio_fade(scene, backpack);
  }
}
