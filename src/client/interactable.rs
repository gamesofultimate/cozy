use crate::shared::components::{
  Boss, BossSensor, CameraTrack, CameraTrackActivator, Conversation, Health, HealthRecovery,
  Interactable, Interaction, IntroTrigger, LevelLoaderTrigger, Movement, Oxygen, Platform,
  PlatformTrigger, Player, PlayerMechanicModifier, Throwable,
};

use crate::shared::ui_components::UIIntro;

use crate::client::GameInput;

use engine::{
  application::{
    components::{AudioSourceComponent, SourceState},
    input::TrustedInput,
    scene::{Collision, Scene},
  },
  systems::{
    input::InputsReader, network::ServerSender, Backpack, Initializable, Inventory, System,
  },
  utils::units::Seconds,
};

pub struct InteractableSystem {
  inputs: InputsReader<GameInput>,
  server_sender: ServerSender<TrustedInput>,
}

impl Initializable for InteractableSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let server_sender = inventory.get::<ServerSender<TrustedInput>>().clone();

    Self {
      inputs,
      server_sender,
    }
  }
}

impl InteractableSystem {
  fn handle_conversation_collision(&mut self, scene: &mut Scene) {
    let input = self.inputs.read();
    for (_, (conversation, interactable, _collision)) in scene.query_mut::<(
      &mut Conversation,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if conversation.active {
        return;
      }

      if interactable.ignore {
        continue;
      }

      let switch = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !switch {
        return;
      }

      interactable.has_activated = true;

      conversation.active = true;
      conversation.current_duration_timer = Seconds::zero();
    }
  }

  fn handle_audio_collision(&mut self, scene: &mut Scene) {
    let input = self.inputs.read();
    for (_, (audio_source, interactable, _collision)) in scene.query_mut::<(
      &mut AudioSourceComponent,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      match audio_source.state {
        SourceState::Playing => return,
        _ => {}
      }

      if interactable.ignore {
        continue;
      }

      let switch = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !switch {
        return;
      }

      interactable.has_activated = true;

      audio_source.state = SourceState::Playing;
    }
  }

  fn handle_audio_collision_boss(&mut self, scene: &mut Scene) {
    for (_, (audio_source, interactable, _collision)) in scene.query_mut::<(
      &mut AudioSourceComponent,
      &mut Interactable,
      &Collision<Boss, Interactable>,
    )>() {
      match audio_source.state {
        SourceState::Playing => return,
        _ => {}
      }

      if interactable.ignore {
        continue;
      }

      let switch = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => false,
      };

      if !switch {
        return;
      }

      interactable.has_activated = true;

      audio_source.state = SourceState::Playing;
    }
  }

  fn handle_boss_trigger(&mut self, scene: &mut Scene) {
    for (_, (sensor, _)) in scene.query_mut::<(&mut BossSensor, &Collision<Player, BossSensor>)>() {
      sensor.activate = true;
    }
    for (_, (sensor, _)) in scene.query_mut::<(&mut BossSensor, &Collision<Boss, BossSensor>)>() {
      sensor.activate = true;
    }
  }

  fn handle_health_recovery_interaction(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = *backpack.get::<Seconds>().unwrap();

    let mut health_consumed = 0.0;

    let input = self.inputs.read();

    for (_, (_player, health, _collision)) in
      scene.query_mut::<(&Player, &Health, &Collision<Player, Interactable>)>()
    {
      if health.current_health >= health.total_health {
        return;
      }
    }

    for (_, (health_recovery, interactable, _collision)) in scene.query_mut::<(
      &mut HealthRecovery,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let interact = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !interact {
        continue;
      }

      interactable.has_activated = true;

      let old_capacity = health_recovery.capacity;

      health_recovery.capacity -= health_recovery.consumption_per_second * *delta_time;
      if health_recovery.capacity <= 0.0 {
        health_recovery.capacity = 0.0;
      }

      health_consumed = old_capacity - health_recovery.capacity;
    }

    if health_consumed == 0.0 {
      return;
    }

    for (_, (_player, health, _collision)) in
      scene.query_mut::<(&Player, &mut Health, &Collision<Player, Interactable>)>()
    {
      health.current_health += health_consumed;

      if health.current_health > health.total_health {
        health.current_health = health.total_health;
      }
    }
  }

  fn handle_camera_track_activator(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let mut current_activator = None;

    for (_, (activator, interactable, _collision)) in scene.query_mut::<(
      &CameraTrackActivator,
      &Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore || !interactable.has_activated {
        continue;
      }

      current_activator = Some(activator.clone());
    }

    let activator = match current_activator {
      Some(activator) => activator,
      None => return,
    };

    let camera_track_entity = scene.get_entity(activator.camera_track).unwrap();
    if let Some(camera_track) = scene.get_components_mut::<&mut CameraTrack>(*camera_track_entity) {
      camera_track.is_running = true;
    }
  }

  fn handle_player_mechanic_modifier(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let input = self.inputs.read();

    let mut current_modifier = None;

    for (_, (modifier, interactable, _collision)) in scene.query_mut::<(
      &PlayerMechanicModifier,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let interact = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !interact {
        continue;
      }

      interactable.has_activated = true;

      current_modifier = Some(modifier.clone());
    }

    let modifier = match current_modifier {
      Some(modifier) => modifier,
      None => return,
    };

    for (_, (_player, player_oxygen, movement, thrower, _collision)) in scene.query_mut::<(
      &Player,
      &mut Oxygen,
      &mut Movement,
      &mut Throwable,
      &Collision<Player, Interactable>,
    )>() {
      if let Some(jump) = &modifier.jump {
        movement.max_num_jumps = jump.max_num_jumps;
      }

      if let Some(grenade) = &modifier.grenade {
        thrower.active = grenade.enabled;
      }

      if let Some(oxygen) = &modifier.oxygen {
        player_oxygen.active = oxygen.enabled;
      }
    }
  }

  fn handle_platform_trigger(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let input = self.inputs.read();

    let mut current_trigger = None;

    for (_, (trigger, interactable, _collision)) in scene.query_mut::<(
      &PlatformTrigger,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let interact = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !interact {
        continue;
      }

      interactable.has_activated = true;

      current_trigger = Some(trigger.clone());
    }

    for (_, (trigger, interactable, _collision)) in scene.query_mut::<(
      &PlatformTrigger,
      &mut Interactable,
      &Collision<Boss, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let mut interact = false;
      if interactable.action == Interaction::Collision {
        interact = true;
      }

      if !interact {
        continue;
      }

      interactable.has_activated = true;

      current_trigger = Some(trigger.clone());
    }

    let trigger = match current_trigger {
      Some(trigger) => trigger,
      None => return,
    };

    for platform_id in trigger.platforms {
      let platform_entity = scene.get_entity(platform_id).unwrap();
      let platform = scene.get_components_mut::<&mut Platform>(*platform_entity);

      if let Some(platform) = platform {
        platform.enabled = true;
      }
    }
  }

  fn handle_level_loader_trigger(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let input = self.inputs.read();

    let mut current_trigger = None;

    for (_, (trigger, interactable, _collision)) in scene.query_mut::<(
      &LevelLoaderTrigger,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let interact = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !interact {
        continue;
      }

      interactable.has_activated = true;

      current_trigger = Some(trigger.clone());
    }

    let trigger = match current_trigger {
      Some(trigger) => trigger,
      None => return,
    };

    self.server_sender.send_reliable(TrustedInput::LoadLevel {
      level_name: trigger.level_name.clone(),
    });
  }

  fn handle_intro_trigger(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let input = self.inputs.read();

    let mut current_trigger = None;

    for (_, (trigger, interactable, _collision)) in scene.query_mut::<(
      &IntroTrigger,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let interact = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !interact {
        continue;
      }

      interactable.has_activated = true;

      current_trigger = Some(trigger.clone());
    }

    if current_trigger.is_none() {
      return;
    }

    for (_, intro) in scene.query_mut::<&mut UIIntro>() {
      intro.enabled = true;
    }
  }

  fn handle_ignore_interactable(&mut self, scene: &mut Scene) {
    for (_, (interactable, _collision)) in
      scene.query_mut::<(&mut Interactable, &Collision<Player, Interactable>)>()
    {
      if interactable.has_activated && interactable.one_time_use {
        interactable.ignore = true;
      }
    }

    for (_, (interactable, _collision)) in
      scene.query_mut::<(&mut Interactable, &Collision<Boss, Interactable>)>()
    {
      if interactable.has_activated && interactable.one_time_use {
        interactable.ignore = true;
      }
    }
  }
}

impl System for InteractableSystem {
  fn get_name(&self) -> &'static str {
    "InteractableSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_conversation_collision(scene);
    self.handle_audio_collision(scene);
    self.handle_audio_collision_boss(scene);
    self.handle_boss_trigger(scene);
    self.handle_health_recovery_interaction(scene, backpack);
    self.handle_camera_track_activator(scene, backpack);
    self.handle_player_mechanic_modifier(scene, backpack);
    self.handle_platform_trigger(scene, backpack);
    self.handle_level_loader_trigger(scene, backpack);
    self.handle_intro_trigger(scene, backpack);

    self.handle_ignore_interactable(scene);
  }
}
