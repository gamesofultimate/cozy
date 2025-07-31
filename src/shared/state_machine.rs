//use crate::shared::audio_components::{AudioGameStart, SoundtrackIntro};
use crate::shared::components::{ActionTypes, ActiveCamera, Character, CharacterState, Player};
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
    physics::PhysicsController, trusty::AssetManager, trusty::MultiplayerController, Backpack,
    Initializable, Inventory, System,
  },
  tsify,
  utils::{
    easing::Easing,
    interpolation::Interpolator,
    units::{Degrees, Radians, Seconds},
  },
  ConnectionId, PlayerId,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tagged::registry::Prev;

#[cfg(target_arch = "wasm32")]
use crate::client::browser::Message;
#[cfg(target_arch = "wasm32")]
use engine::systems::browser::BrowserController;
#[cfg(target_arch = "wasm32")]
use engine::systems::rendering::CameraConfig;

pub struct StateMachineSystem {
  current_time: Seconds,
  physics: PhysicsController,
  inputs: InputsReader<GameInput>,
  #[cfg(target_arch = "wasm32")]
  browser: BrowserController<Message>,
  multiplayer: MultiplayerController,
  pending_required: usize,
  pending_priority: usize,
  downloaded_required: usize,
  downloaded_priority: usize,
}

impl Initializable for StateMachineSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let multiplayer = inventory.get::<MultiplayerController>().clone();
    #[cfg(target_arch = "wasm32")]
    let browser = inventory.get::<BrowserController<Message>>().clone();
    Self {
      physics,
      inputs,
      multiplayer,
      #[cfg(target_arch = "wasm32")]
      browser,
      pending_required: 0,
      pending_priority: 0,
      downloaded_required: 0,
      downloaded_priority: 0,
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
      self.handle_sign_up(scene, backpack);
      self.handle_camera_dof(scene, backpack);
      self.handle_game_loading(scene, backpack);
      self.handle_update_ui(scene);
    }

    self.handle_state(scene, backpack);
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

  #[cfg(target_arch = "wasm32")]
  fn handle_sign_up(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let (machine, manager) = backpack.fetch_mut::<(StateMachine, AssetManager)>()?;

    for (_, (network, _)) in scene.query_mut::<(&NetworkedPlayerComponent, &SelfComponent)>() {
      if machine.is_active() && !machine.is_logged_in(&network.connection_id) {
        machine.set_signup();
      }
    }

    Some(())
  }

  #[cfg(target_arch = "wasm32")]
  fn handle_game_loading(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let (machine, manager) = backpack.fetch_mut::<(StateMachine, AssetManager)>()?;

    let pending_required = manager.pending_required_count();
    let pending_priority = manager.pending_priority_count();
    let downloaded_required = manager.downloaded_required_count();
    let downloaded_priority = manager.downloaded_priority_count();

    let total_downloaded = downloaded_required + downloaded_priority;
    let total_necessary =
      pending_required + pending_priority + downloaded_required + downloaded_priority;

    if pending_required != self.pending_required
      || pending_priority != self.pending_priority
      || downloaded_required != self.downloaded_required
      || downloaded_priority != self.downloaded_priority
    {
      // send message
      self.pending_required = pending_required;
      self.pending_priority = pending_priority;
      self.downloaded_required = downloaded_required;
      self.downloaded_priority = downloaded_priority;

      self.browser.send(Message::UpdateDownloadStats {
        pending_required,
        pending_priority,
        downloaded_required,
        downloaded_priority,
      });

      if manager.is_downloading_required() || manager.is_downloading_priority() {
        machine.set_downloading();
      }

      if total_downloaded == total_necessary {
        machine.set_loaded();
      }
    }

    Some(())
  }

  #[cfg(target_arch = "wasm32")]
  fn handle_camera_dof(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    use crate::shared::components::CameraFollower;

    let (delta_time, camera_config) = match backpack.fetch_mut::<(Seconds, CameraConfig)>() {
      Some((delta_time, camera_config)) => Some((delta_time.clone(), camera_config.clone())),
      None => None,
    }?;

    let focus_position = match scene.query_one::<(&mut TransformComponent, &ActiveCamera)>() {
      Some((_, (transform, _))) => transform.translation,
      None => return None,
    };

    let character = match scene.query_one::<(&CharacterState, &SelfComponent)>() {
      Some((_, (character, _))) => character,
      None => return None,
    };

    let camera_position = camera_config.translation;

    if let Some((config, game)) = backpack.fetch_mut::<(Config, StateMachine)>() {
      if config.dof.focus_scale < 0.01 {
        config.dof.enabled = false;
      } else {
        config.dof.enabled = true;
      }

      let distance = Vector3::metric_distance(&camera_position, &focus_position);

      match game.state {
        GameState::Playing if matches!(character, CharacterState::ShowingOff { .. }) => {
          config.dof.focus_point = lerp(config.dof.focus_point, distance - 1.0, 0.9);
          config.dof.focus_scale = (config.dof.focus_scale + 0.001 * *delta_time).clamp(0.0, 3.0);

          for (entity, (transform, _)) in
            scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
          {
            transform.translation = Vector3::new(0.0, 0.0, -3.0);
            transform.rotation = Vector3::new(0.0, 0.0, 0.0);
          }
        }
        GameState::Signup => {
          config.dof.focus_point = lerp(config.dof.focus_point, distance, 0.9);
          config.dof.focus_scale = (config.dof.focus_scale - 0.001 * *delta_time).clamp(0.0, 3.0);
          for (_, follower) in scene.query_mut::<&mut CameraFollower>() {
            follower.interpolation_speed = 0.02;
          }

          let mut camera = None;

          for (entity, (transform, _)) in
            scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
          {
            camera = Some(transform.world_transform().translation);

            transform.translation = Vector3::new(-1.0, 0.0, -3.0);
            transform.rotation = Vector3::new(0.0, 0.0, 0.0);
          }

          if let Some(camera) = backpack.get_mut::<CameraConfig>() {
            for (entity, (transform, _, _)) in
              scene.query_mut::<(&mut TransformComponent, &Player, &SelfComponent)>()
            {
              let character = transform.world_transform().translation;
              let direction = Unit::new_normalize(camera.translation - character);

              let yaw = direction.x.atan2(direction.z);

              transform.rotation = Vector3::new(0.0, yaw, 0.0);
            }
          }
        }
        GameState::Playing => {
          config.dof.focus_point = lerp(config.dof.focus_point, distance, 0.9);
          config.dof.focus_scale = (config.dof.focus_scale - 0.001 * *delta_time).clamp(0.0, 3.0);
          for (_, follower) in scene.query_mut::<&mut CameraFollower>() {
            follower.interpolation_speed = 0.08;
          }

          for (entity, (transform, _)) in
            scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
          {
            let degrees = Radians::from(Degrees::new(45.0));
            transform.translation = Vector3::new(0.0, 6.0, -8.0);
            transform.rotation = Vector3::new(*degrees, 0.0, 0.0);
          }
        }
        GameState::Initializing | GameState::Downloading | GameState::Loaded => {
          for (entity, (transform, _)) in
            scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
          {
            transform.translation = Vector3::new(0.0, 0.0, -3.0);
            transform.rotation = Vector3::new(0.0, 0.0, 0.0);
          }
        }
        GameState::Paused => {
          config.dof.focus_point = lerp(config.dof.focus_point, 0.0, 0.9);
          config.dof.focus_scale = (config.dof.focus_scale + 0.001 * *delta_time).clamp(0.0, 3.0);
        }
        _ => {
          config.dof.focus_point = lerp(config.dof.focus_point, distance, 0.9);
          config.dof.focus_scale = (config.dof.focus_scale + 0.001 * *delta_time).clamp(0.0, 3.0);
          for (_, follower) in scene.query_mut::<&mut CameraFollower>() {
            follower.interpolation_speed = 0.02;
          }
        }
      }
    }

    Some(())
  }

  #[cfg(target_arch = "wasm32")]
  fn handle_start(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let input = self.inputs.read_client();
    if let Some(machine) = backpack.get_mut::<StateMachine>() {
      match &machine.state {
        GameState::Loaded if input.state.contains(InputState::LeftClick) => machine.start_game(),
        GameState::Playing if input.state.contains(InputState::Escape) => machine.pause_game(),
        GameState::Paused if input.state.contains(InputState::LeftClick) => machine.resume_game(),
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

      // if no one is online, go back to initializing
      if game.players.len() == 0 {
        game.state = GameState::Initializing;
        self.current_time = Seconds::new(0.0);
      }

      match game.state {
        GameState::Initializing => {
          game.state = GameState::Downloading;
          self.current_time = Seconds::new(0.0);
        }
        GameState::RequestTransition => {
          game.state = GameState::TransitionToGame {
            timeout: Seconds::new(2.0),
          };
          self.current_time = Seconds::new(0.0);
        }
        GameState::TransitionToGame { timeout } if self.current_time > timeout => {
          game.state = GameState::Playing;
          self.current_time = Seconds::new(0.0);
        }
        _ => {}
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

  #[cfg(target_arch = "wasm32")]
  fn handle_replicate(&mut self, backpack: &mut Backpack) {
    if let Some((state, Prev(prev))) = backpack.fetch_mut::<(StateMachine, Prev<StateMachine>)>() {
      if state != prev {
        self.browser.send(Message::UpdateStateMachine {
          state: state.clone(),
        });
      }
    }
  }

  #[cfg(target_arch = "wasm32")]
  fn handle_update_ui(&mut self, scene: &mut Scene) {
    for (_, (character, state, Prev(prev_character), Prev(prev_state), _)) in scene.query_mut::<(
      &Character,
      &CharacterState,
      &Prev<Character>,
      &Prev<CharacterState>,
      &SelfComponent,
    )>() {
      if character != prev_character || state != prev_state {
        self.browser.send(Message::UpdateCharacter {
          character: character.clone(),
          state: match (state, &character.action) {
            (CharacterState::CollectingWater, _) => String::from("Collecting Water.."),
            (CharacterState::WorkingTile(_), _) => String::from("Watering Soil.."),
            (CharacterState::ThrowingSeed(_, _), _) => String::from("Planting Seed.."),
            (CharacterState::Harvesting(_, _), _) => String::from("Harvesting..."),
            (_, ActionTypes::WaterTile) => String::from("Water Soil"),
            (_, ActionTypes::ThrowSeed) => String::from("Plant Seed"),
            (_, ActionTypes::Harvest) => String::from("Harvest"),
          },
        });
      }
    }

    for (_, (curr, Prev(prev))) in scene.query_mut::<(&Character, &mut Prev<Character>)>() {
      *prev = curr.clone();
    }

    for (_, (curr, Prev(prev))) in scene.query_mut::<(&CharacterState, &mut Prev<CharacterState>)>()
    {
      *prev = curr.clone();
    }

    let mut force_update = false;
    {
      let mut new_entities = vec![];
      for (entity, character) in scene.query_mut::<&Character>().without::<Prev<Character>>() {
        new_entities.push((entity.clone(), character.clone()));
      }

      for (entity, character) in new_entities {
        scene.add_local_component(entity, Prev(character));
        force_update = true;
      }
    }

    {
      let mut new_entities = vec![];
      for (entity, state) in scene
        .query_mut::<&CharacterState>()
        .without::<Prev<CharacterState>>()
      {
        new_entities.push((entity.clone(), state.clone()));
      }

      for (entity, state) in new_entities {
        scene.add_local_component(entity, Prev(state));
        force_update = true;
      }
    }

    if force_update {
      for (_, (character, state, _)) in
        scene.query_mut::<(&Character, &CharacterState, &SelfComponent)>()
      {
        self.browser.send(Message::UpdateCharacter {
          character: character.clone(),
          state: match (state, &character.action) {
            (CharacterState::CollectingWater, _) => String::from("Collecting Water.."),
            (CharacterState::WorkingTile(_), _) => String::from("Watering Soil.."),
            (CharacterState::ThrowingSeed(_, _), _) => String::from("Planting Seed.."),
            (CharacterState::Harvesting(_, _), _) => String::from("Harvesting..."),
            (_, ActionTypes::WaterTile) => String::from("Water Soil"),
            (_, ActionTypes::ThrowSeed) => String::from("Plant Seed"),
            (_, ActionTypes::Harvest) => String::from("Harvest"),
          },
        });
      }
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, tsify::Tsify)]
pub enum GameState {
  Initializing,
  Downloading,
  Loaded,
  RequestTransition,
  Signup,
  TransitionToGame { timeout: Seconds },
  Playing,
  Paused,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, tsify::Tsify)]
pub struct StateMachine {
  pub players: Vec<(ConnectionId, String, Option<PlayerId>)>,
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

  pub fn is_logged_in(&self, search: &ConnectionId) -> bool {
    if let Some((_, _, Some(_))) = self.players.iter().find(|(id, _, _)| id == search) {
      true
    } else {
      false
    }
  }

  pub fn signup(&mut self, search: &ConnectionId, id: PlayerId) {
    if let Some((_, _, player)) = self.players.iter_mut().find(|(id, _, _)| id == search) {
      *player = Some(id);
    }
  }

  pub fn is_playing(&self) -> bool {
    if let GameState::Playing = &self.state {
      true
    } else {
      false
    }
  }

  pub fn is_active(&self) -> bool {
    if let GameState::Playing | GameState::Paused = &self.state {
      true
    } else {
      false
    }
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
  pub fn connect(&mut self, id: ConnectionId, username: &str, player: Option<PlayerId>) {
    self.players.push((id, String::from(username), player));

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
    //self.state = GameState::RequestTransition;
    self.state = GameState::Playing;
  }

  pub fn resume_game(&mut self) {
    self.state = GameState::Playing;
  }

  pub fn set_downloading(&mut self) {
    self.state = GameState::Downloading;
  }

  pub fn set_signup(&mut self) {
    self.state = GameState::Signup;
  }

  pub fn set_loaded(&mut self) {
    self.state = GameState::Loaded;
  }

  pub fn pause_game(&mut self) {
    self.state = GameState::Paused;
  }
}

fn lerp(a: f32, b: f32, percent: f32) -> f32 {
  a * percent + b * (1.0 - percent)
}
