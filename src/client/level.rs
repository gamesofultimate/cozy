use crate::shared::components::{Boss, LevelName, LevelState, Player, RequestLevelLoad};
use engine::application::components::{
  LevelDownloaderVolume, LightComponent, ModelComponent, StateMachineComponent,
};
use engine::application::scene::{TagComponent, TransformComponent};
use engine::systems::level::LevelManager;
use engine::utils::easing::Easing;
use engine::utils::interpolation::Interpolator;
use engine::utils::units::Seconds;
use engine::{
  application::scene::Scene,
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
};

pub struct LevelSystem {
  physics: PhysicsController,
  instantiate_player: bool,
}

impl Initializable for LevelSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();

    Self {
      physics,
      instantiate_player: false,
    }
  }
}

impl LevelSystem {
  fn handle_request_load(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if self.instantiate_player {
      let (entity, _) = scene.query_mut::<&Player>().into_iter().next().unwrap();
      if let Some(_) = scene.spawn_entity_and_prefab_and_children_with(
        entity,
        "PlayerJail",
        |prefab| {
          log::error!("prefab: {:?}", prefab);
        },
        |child_prefab| {
          log::error!("child: {:?}", child_prefab);
        },
      ) {
        self.instantiate_player = false;

        let (_, (_player, player_transform)) = scene
          .query_mut::<(&Player, &TransformComponent)>()
          .into_iter()
          .next()
          .unwrap();
        log::error!("Player: {:?}", player_transform);

        let (_, (_player, enemy_transform)) = scene
          .query_mut::<(&Boss, &TransformComponent)>()
          .into_iter()
          .next()
          .unwrap();

        log::error!("Enemy: {:?}", enemy_transform);

        for (_, (transform, tag)) in scene.query_mut::<(&TransformComponent, &TagComponent)>() {
          if tag.name == "Jail Starter Area" {
            log::error!("Starter: {:?}", transform);
          }
        }
      }
      use engine::application::scene::Prefab;
      let prefab = Prefab::pack(scene, entity);
      //log::info!("resulting new prefab: {:?}", &prefab);
    }

    if backpack.take::<RequestLevelLoad>().is_some() {
      {
        let (entity, _) = scene.query_mut::<&Player>().into_iter().next().unwrap();
        let _ = scene.remove_component::<ModelComponent>(entity);
        let _ = scene.remove_component::<StateMachineComponent>(entity);
      }

      let (_, level_downloader) = scene
        .query_mut::<&LevelDownloaderVolume>()
        .into_iter()
        .next()
        .unwrap();

      let level_name = level_downloader.level.clone();

      let level_manager = backpack.get_mut::<LevelManager>().unwrap();
      level_manager.load_level(scene, level_name);

      self.instantiate_player = true;
    }
  }

  fn handle_level_state(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    if let Some(level_state) = backpack.get_mut::<LevelState>() {
      level_state.current = level_state.next.clone();
    }

    let (entity, new_level_name) = scene.query_mut::<&LevelName>().into_iter().next()?;
    let new_level_name = new_level_name.clone();
    let _ = scene.remove_component::<LevelName>(entity);

    if backpack.get::<LevelState>().is_none() {
      backpack.insert(LevelState::new(new_level_name));
    }

    if let Some(level_state) = backpack.get_mut::<LevelState>() {
      level_state.next = new_level_name;
    }

    Some(())
  }

  fn handle_level_transition(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let delta_time = *backpack.get::<Seconds>().unwrap();
    let level_state = backpack.get_mut::<LevelState>()?;

    match (level_state.current.clone(), level_state.next.clone()) {
      (LevelName::Loading, LevelName::Jail) => {
        let interpolator = Interpolator::new(0.0, 1.0, Easing::SineInOut, 0.0..=1.0);
        level_state.interpolator = Some(interpolator);
      }
      _ => {}
    }

    if let Some(ref mut interpolator) = level_state.interpolator {
      interpolator.accumulate(*delta_time);

      if interpolator.is_finished() {
        level_state.interpolator = None;
      } else {
        for (_entity, light) in scene.query_mut::<&mut LightComponent>() {
          if let LightComponent::Point { intensity, .. } = light {
            *intensity = interpolator.get();
          }
        }
      }
    }

    Some(())
  }
}

impl System for LevelSystem {
  fn get_name(&self) -> &'static str {
    "LevelSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_request_load(scene, backpack);
    self.handle_level_state(scene, backpack);
    self.handle_level_transition(scene, backpack);
  }
}
