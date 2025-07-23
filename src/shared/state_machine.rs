//use crate::shared::audio_components::{AudioGameStart, SoundtrackIntro};
use crate::shared::components::{ActiveCamera, Player};
use crate::shared::game_input::{GameInput, InputState};
use chrono::{DateTime, TimeDelta, Utc};
use engine::{
  application::{
    components::{
      AudioSourceComponent, CameraComponent, NetworkedPlayerComponent, PhysicsComponent,
      SelfComponent, SourceState,
    },
    config::Config,
    input::InputsReader,
    scene::{Scene, TransformComponent},
  },
  nalgebra::{Unit, Vector3},
  systems::{
    physics::PhysicsController, trusty::MultiplayerController, Backpack, Initializable, Inventory,
    System,
  },
  utils::{
    easing::Easing,
    interpolation::Interpolator,
    units::{Degrees, Radians, Seconds},
  },
  ConnectionId,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tagged::registry::Prev;

#[cfg(target_arch = "wasm32")]
use engine::systems::rendering::CameraConfig;

pub struct StateMachineSystem {
  current_time: Seconds,
  physics: PhysicsController,
  inputs: InputsReader<GameInput>,
  multiplayer: MultiplayerController,
}

impl Initializable for StateMachineSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let multiplayer = inventory.get::<MultiplayerController>().clone();
    Self {
      physics,
      inputs,
      multiplayer,
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
    #[cfg(target_arch = "wasm32")]
    {
      self.handle_start(scene, backpack);
      self.handle_receive_from_server(scene, backpack);
      self.handle_camera_dof(scene, backpack);
    }
    self.set_camera(scene, backpack);
    //self.handle_transitions(scene, backpack);

    self.handle_state(scene, backpack);
    #[cfg(not(target_arch = "wasm32"))]
    self.handle_replicate(backpack);
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

  #[cfg(target_arch = "wasm32")]
  fn handle_camera_dof(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut camera_position = Vector3::zeros();
    let mut focus_position = Vector3::zeros();

    if let Some(camera_config) = backpack.get_mut::<CameraConfig>() {
      camera_position = camera_config.translation;
    }

    for (_, (transform, _)) in scene.query_mut::<(&mut TransformComponent, &ActiveCamera)>() {
      focus_position = transform.translation;
    }

    if let Some((config, game)) = backpack.fetch_mut::<(Config, StateMachine)>() {
      if config.dof.focus_scale < 0.1 {
        config.dof.enabled = false;
      } else {
        config.dof.enabled = true;
      }

      log::info!("state: {:?}", game.state);

      config.dof.focus_point = Vector3::metric_distance(&camera_position, &focus_position);

      match game.state {
        GameState::RequestTransition | GameState::TransitionToGame { .. } | GameState::Playing => {
          config.dof.focus_scale = (config.dof.focus_scale - 0.04).clamp(0.0, 3.0);
        }
        _ => {
          config.dof.focus_scale = (config.dof.focus_scale + 0.04).clamp(0.0, 3.0);
        }
      }
    }
  }

  fn set_camera(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if let Some((next, Prev(prev))) = backpack.fetch_mut::<(StateMachine, Prev<StateMachine>)>() {
      match (&prev.state, &next.state) {
        (GameState::Loading, GameState::RequestTransition)
        | (GameState::Loading, GameState::TransitionToGame { .. }) => {
          let mut player = None;
          for (entity, (_, _, _)) in
            scene.query_mut::<(&Player, &NetworkedPlayerComponent, &SelfComponent)>()
          {
            player = Some(entity);
          }

          if let Some(entity) = player {
            scene.add_local_component(entity, ActiveCamera {});
          }

          for (entity, (transform, _)) in
            scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
          {
            let degrees = Radians::from(Degrees::new(45.0));
            transform.translation = Vector3::new(0.0, 6.0, -6.0);
            transform.rotation = Vector3::new(*degrees, 0.0, 0.0);
          }
        }
        (_, GameState::Initializing) => {
          for (entity, (transform, _)) in
            scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
          {
            transform.translation = Vector3::new(0.0, 0.0, -3.0);
            transform.rotation = Vector3::new(0.0, 0.0, 0.0);
          }
        }
        _ => {}
      }
      /*
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
      */
    }
  }

  #[cfg(target_arch = "wasm32")]
  fn handle_start(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let input = self.inputs.read_client();
    if let Some(machine) = backpack.get_mut::<StateMachine>() {
      match &machine.state {
        GameState::Loading if input.state.contains(InputState::LeftClick) => machine.start_game(),
        _ => {}
      }
    }
  }

  #[cfg(target_arch = "wasm32")]
  fn handle_receive_from_server(&mut self, _: &mut Scene, backpack: &mut Backpack) {
    while let Ok(state) = self.multiplayer.try_recv_custom::<StateMachine>() {
      backpack.insert(state);
    }
  }

  fn handle_state(&mut self, _: &mut Scene, backpack: &mut Backpack) {
    if let Some((game, delta_time)) = backpack.fetch_mut::<(StateMachine, Seconds)>() {
      self.current_time += *delta_time;

      if let Some(_) = game.bump_state(self.current_time) {
        self.current_time = Seconds::new(0.0);
      }
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn handle_replicate(&mut self, backpack: &mut Backpack) {
    if let Some((game, Prev(prev))) = backpack.fetch_mut::<(StateMachine, Prev<StateMachine>)>() {
      if game != prev {
        self.multiplayer.broadcast_custom(game.clone());
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
  Pause,
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

  pub fn start_game(&mut self) {
    self.state = GameState::RequestTransition;
  }

  pub fn bump_state(&mut self, current_time: Seconds) -> Option<()> {
    // if no one is online, go back to initializing
    if self.players.len() == 0 {
      self.state = GameState::Initializing;
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
