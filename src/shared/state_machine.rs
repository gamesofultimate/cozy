//use crate::shared::audio_components::{AudioGameStart, SoundtrackIntro};
use crate::shared::components::{ActiveCamera, Player};
use chrono::{DateTime, TimeDelta, Utc};
use engine::{
  application::{
    components::{
      AudioSourceComponent, NetworkedPlayerComponent, PhysicsComponent, SelfComponent, SourceState,
    },
    scene::Scene,
  },
  nalgebra::{Unit, Vector3},
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
  utils::{easing::Easing, interpolation::Interpolator, units::Seconds},
  ConnectionId,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tagged::registry::Prev;

pub struct StateMachineSystem {
  current_time: Seconds,
  physics: PhysicsController,
}

impl Initializable for StateMachineSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    Self {
      physics,
      current_time: Seconds::new(0.0),
    }
  }
}

impl System for StateMachineSystem {
  fn get_name(&self) -> &'static str {
    "StateMachineSystem"
  }

  fn attach(&mut self, _scene: &mut Scene, backpack: &mut Backpack) {
    let machine = StateMachine::new();
    backpack.insert(machine.clone());
    backpack.insert(Prev(machine));
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_turns(scene, backpack);
    self.set_camera(scene, backpack);
    self.handle_transitions(scene, backpack);
    self.handle_prev(backpack);
  }
}

impl StateMachineSystem {
  pub fn handle_prev(&mut self, backpack: &mut Backpack) {
    if let Some((curr, Prev(prev))) = backpack.fetch_mut::<(StateMachine, Prev<StateMachine>)>() {
      *prev = curr.clone();
    }
  }

  fn handle_transitions(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    /*
    use engine::application::{
      components::{LightComponent, SkyLightComponent},
      scene::TransformComponent,
    };

    let delta_time = *backpack.get::<Seconds>().clone().unwrap();
    let mut done = None;

    for (entity, (sky, directional, (directional_intensity, sky_lighting, sky_background))) in scene
      .query_mut::<(
        &mut SkyLightComponent,
        &mut LightComponent,
        &mut (Interpolator, Interpolator, Interpolator),
      )>()
    {
      directional_intensity.accumulate(*delta_time);
      sky_lighting.accumulate(*delta_time);
      sky_background.accumulate(*delta_time);

      if let LightComponent::Directional { intensity, .. } = directional {
        *intensity = directional_intensity.get();
      }

      // handle both cases, so that we can change without breaking
      match sky {
        SkyLightComponent::Dynamic {
          lighting_intensity,
          background_intensity,
          ..
        } => {
          *lighting_intensity = sky_lighting.get();
          *background_intensity = sky_background.get();
        }
        SkyLightComponent::Image {
          lighting_intensity,
          background_intensity,
          ..
        } => {
          *lighting_intensity = sky_lighting.get();
          *background_intensity = sky_background.get();
        }
      }

      /*
      log::info!("
        *intensity = directional_intensity.get();
        *lighting_intensity = sky_lighting.get();
        *background_intensity = sky_background.get();
      */

      if directional_intensity.is_finished()
        && sky_lighting.is_finished()
        && sky_background.is_finished()
      {
        done = Some(entity);
      }
    }

    if let Some(entity) = done {
      let _ = scene.remove_local_component::<(Interpolator, Interpolator, Interpolator)>(entity);
    }

    if let Some((next, Prev(prev))) = backpack.fetch_mut::<(StateMachine, Prev<StateMachine>)>() {
      match (&prev.state, &next.state) {
        (GameState::TeamSelection { .. }, GameState::Starting { locations, .. }) => {
          /*
          for (_, (audio, _)) in scene.query_mut::<(&mut AudioSourceComponent, &SoundtrackIntro)>()
          {
            audio.state = SourceState::Stopped;
          }
          for (_, (audio, _)) in scene.query_mut::<(&mut AudioSourceComponent, &AudioGameStart)>() {
            audio.state = SourceState::Playing;
          }
          */

          let mut entities = vec![];

          for (entity, (sky, directional)) in
            scene.query_mut::<(&mut SkyLightComponent, &mut LightComponent)>()
          {
            let directional = if let LightComponent::Directional { intensity, .. } = directional {
              let cached = *intensity;
              *intensity = 0.0;
              cached
            } else {
              continue;
            };
            // handle both cases, so that we can change without breaking
            let (sky_lighting, sky_background) = match sky {
              SkyLightComponent::Dynamic {
                lighting_intensity,
                background_intensity,
                ..
              } => {
                let cached_lighting = *lighting_intensity;
                let cached_background = *background_intensity;
                *lighting_intensity = 0.0;
                *background_intensity = 0.0;
                (cached_lighting, cached_background)
              }
              SkyLightComponent::Image {
                lighting_intensity,
                background_intensity,
                ..
              } => {
                let cached_lighting = *lighting_intensity;
                let cached_background = *background_intensity;
                *lighting_intensity = 0.0;
                *background_intensity = 0.0;
                (cached_lighting, cached_background)
              }
            };

            entities.push((entity, directional, sky_lighting, sky_background));
          }

          for (entity, directional, sky_lighting, sky_background) in entities {
            scene.add_local_component(
              entity,
              (
                Interpolator::new(0.0, directional, Easing::Linear, 0.0..=0.600),
                Interpolator::new(0.0, sky_lighting, Easing::Linear, 0.0..=0.600),
                Interpolator::new(0.0, sky_background, Easing::Linear, 0.0..=0.600),
              ),
            );
          }

          let mut player_location = None;

          for (_, (_, transform, network, maybe_self)) in scene.query_mut::<(
            &Player,
            &mut TransformComponent,
            &NetworkedPlayerComponent,
            Option<&SelfComponent>,
          )>() {
            if let Some(spawn_position) = locations.get(&network.connection_id) {
              // It doesn't have a physics body yet, so it can't be moved by physics
              transform.translation = *spawn_position;

              if let Some(_) = maybe_self {
                player_location = Some(*spawn_position);
              }
            }
          }

          #[cfg(target_arch = "wasm32")]
          {
            use engine::systems::rendering::CameraConfig;

            if let Some(player_location) = player_location
              && let Some(camera) = backpack.get_mut::<CameraConfig>()
            {
              camera.translation = player_location;
            }
          }

          log::info!("Game is starting!");
        }
        // Normally do nothing
        _ => {}
      }
    }
    */
  }

  fn set_camera(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    /*
    if let Some((game,)) = backpack.fetch_mut::<(StateMachine,)>() {
      scene.clear_component::<ActiveCamera>();
      if let Some((current_player, _)) = game.get_current() {
        let mut next_camera = None;

        for (entity, (_, network)) in scene.query_mut::<(&Player, &NetworkedPlayerComponent)>() {
          if network.connection_id == *current_player {
            next_camera = Some(entity);
            break;
          }
        }
        if let Some(entity) = next_camera {
          scene.add_local_component(entity, ActiveCamera {});
        }
      } else if let GameState::TeamSelection { .. } = game.state {
        let mut next_camera = None;

        for (entity, (_, _, _)) in
          scene.query_mut::<(&Player, &SelfComponent, &NetworkedPlayerComponent)>()
        {
          next_camera = Some(entity);
          break;
        }
        if let Some(entity) = next_camera {
          scene.add_local_component(entity, ActiveCamera {});
        }
      } else if let GameState::Starting { .. } = game.state {
        // Show my new location when the game starts
        let mut next_camera = None;

        for (entity, (_, _, _)) in
          scene.query_mut::<(&Player, &SelfComponent, &NetworkedPlayerComponent)>()
        {
          next_camera = Some(entity);
          break;
        }
        if let Some(entity) = next_camera {
          scene.add_local_component(entity, ActiveCamera {});
        }
      }
    }
    */
  }

  fn handle_turns(&mut self, _: &mut Scene, backpack: &mut Backpack) {
    if let Some((game, delta_time)) = backpack.fetch_mut::<(StateMachine, Seconds)>() {
      self.current_time += *delta_time;

      if let Some(_) = game.bump_state(self.current_time) {
        self.current_time = Seconds::new(0.0);
      }
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GameState {
  Initializing,
  Loading,
  RequestTransition,
  TransitionToGame { timeout: Seconds },
  Playing,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StateMachine {
  pub players: Vec<(ConnectionId, String)>,
  pub state: GameState,
  pub ready: HashSet<ConnectionId>,
  pub admin: Option<ConnectionId>,
}

#[allow(unused)]
impl StateMachine {
  pub fn new() -> Self {
    Self {
      players: vec![],
      state: GameState::Initializing,
      ready: HashSet::new(),
      admin: None,
    }
  }

  pub fn mark_ready(&mut self, id: ConnectionId) {
    self.ready.insert(id);
  }

  pub fn mark_unready(&mut self, id: ConnectionId) {
    let _ = self.ready.remove(&id);
  }

  pub fn check_readiness(&mut self, id: ConnectionId) -> bool {
    self.ready.contains(&id)
  }

  pub fn check_admin(&mut self, id: ConnectionId) -> bool {
    if let Some(admin) = self.admin
      && admin == id
    {
      true
    } else {
      false
    }
  }

  pub fn ready_count(&mut self) -> (usize, usize) {
    (self.ready.len(), self.players.len())
  }

  pub fn can_current_move(&self) -> bool {
    unimplemented!()
    /*
    if let GameState::Turn { current_player, .. } = self.state
      && let Some((_current_player, _)) = self.players.get(current_player)
    {
      true
    } else {
      false
    }
    */
  }

  pub fn can_move(&self, id: ConnectionId) -> bool {
    unimplemented!()
    /*
    if let GameState::Turn { current_player, .. } = self.state
      && let Some((current_player, _)) = self.players.get(current_player)
    {
      *current_player == id
    } else {
      false
    }
    */
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn connect(&mut self, id: ConnectionId, username: &str) {
    self.players.push((id, String::from(username)));

    if let None = self.admin {
      self.admin = Some(id);
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn disconnect(&mut self, id: ConnectionId) {
    self.players.retain(|player| id != player.0);

    if let Some(admin) = self.admin
      && admin == id
      && self.players.len() > 0
    {
      let id = self.players[0].0;
      self.admin = Some(id);
    }

    if self.players.len() == 0 {
      self.admin = None;
    }
  }

  pub fn bump_state(&mut self, current_time: Seconds) -> Option<()> {
    // if no one is online, go back to initializing
    if self.players.len() == 0 {
      let state = GameState::Initializing;
      return Some(());
    }

    match self.state {
      GameState::Initializing => {
        self.state = GameState::Loading;
        Some(())
      }
      GameState::RequestTransition => {
        self.state = GameState::TransitionToGame {
          timeout: Seconds::new(2.0),
        };
        Some(())
      }
      GameState::TransitionToGame { timeout } if current_time > timeout => {
        self.state = GameState::Playing;
        Some(())
      }
      _ => None,
    }
  }
}
