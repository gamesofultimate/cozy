use crate::client::game_input::GameInput;
use crate::shared::components::Muzzle;
use crate::shared::components::{
  Barrier, Boss, Bullet, Enemy, Gun, GunMode, GunModelConfigurator, GunSoundConfigurator, Health,
  Player,
};
use engine::application::components::ModelComponent;
use engine::application::components::ParentComponent;
use engine::{
  application::{
    components::{CameraComponent, InputComponent, PhysicsComponent, SelfComponent},
    physics3d::RigidBodyHandle,
    scene::{Collision, IdComponent, Scene, TransformComponent},
  },
  resources::{node::Transform, particles::ParticleId},
  systems::{
    controller::AudioController, input::InputsReader, physics::PhysicsController,
    rendering::ParticleController, Backpack, Initializable, Inventory, System,
  },
  utils::physics,
  utils::units::{Decibels, Degrees, Meters, Mps, Seconds},
  Entity,
};
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use rand::Rng;
use rapier3d::prelude::{QueryFilter, Ray};
use std::f32::consts::PI;
use std::mem::variant_count;
use uuid::Uuid;

const POOL_CAPACITY: usize = 2_000;

struct BulletPool {
  pool: Vec<Entity>,
  current: usize,
}

impl BulletPool {
  pub fn new() -> Self {
    let pool = Vec::with_capacity(POOL_CAPACITY);
    Self { pool, current: 0 }
  }

  pub fn get(&mut self) -> Entity {
    let current = self.current;
    self.current = (self.current + 1) % POOL_CAPACITY;
    self.pool[current].clone()
  }

  pub fn recycle(&mut self, scene: &mut Scene, entity: Entity) {
    scene.recycle(entity);
  }

  pub fn initialize(&mut self, scene: &mut Scene) {
    let entities = scene
      .spawn_batch((0..POOL_CAPACITY).map(|_| ()))
      .collect::<Vec<_>>();
    self.pool = entities;
  }
}

pub struct ShootingSystem {
  inputs: InputsReader<GameInput>,
  physics_controller: PhysicsController,
  particle_controller: ParticleController,
  audio_controller: AudioController,
  pool: BulletPool,
}

impl Initializable for ShootingSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let physics_controller = inventory.get::<PhysicsController>().clone();
    let particle_controller = inventory.get::<ParticleController>().clone();
    let audio_controller = inventory.get::<AudioController>().clone();
    let pool = BulletPool::new();

    Self {
      inputs,
      physics_controller,
      particle_controller,
      audio_controller,
      pool,
    }
  }
}

impl ShootingSystem {
  fn gun_index_to_mode(index: i32) -> GunMode {
    match index {
      0 => GunMode::Single,
      1 => GunMode::Burst,
      2 => GunMode::Blast,
      // 3 => GunMode::PulseBeam,
      _ => GunMode::Single,
    }
  }

  fn gun_mode_to_prefab_name(mode: GunMode) -> &'static str {
    match mode {
      GunMode::Single => "SingleLaserBullet",
      GunMode::Burst => "BurstLaserBullet",
      GunMode::Blast => "BlastLaserBullet",
      // GunMode::PulseBeam => "PulseBeamBullet",
    }
  }

  fn gun_has_ammo(gun: &Gun) -> bool {
    match gun.mode {
      GunMode::Single => gun.single_ammo > 0,
      GunMode::Burst => gun.burst_ammo > 0,
      GunMode::Blast => gun.blast_ammo > 0,
      // GunMode::PulseBeam => gun.pulse_beam_ammo > 0,
    }
  }

  fn gun_has_low_ammo(gun: &Gun) -> bool {
    match gun.mode {
      GunMode::Single => gun.single_ammo <= 5,
      GunMode::Burst => gun.burst_ammo <= 5,
      GunMode::Blast => gun.blast_ammo <= 5,
      // GunMode::PulseBeam => gun.pulse_beam_ammo < 5,
    }
  }

  fn handle_gun_readiness(gun: &mut Gun, override_single_mode: bool, delta_time: Seconds) -> bool {
    let gun_mode_refire_rate = match gun.mode {
      GunMode::Burst => Seconds::new(*gun.refire_rate / gun.burst_multiplier),
      _ => gun.refire_rate,
    };

    if let Some(ref mut refire_timer) = gun.refire_timer {
      *refire_timer += delta_time;
    }
    if let Some(refire_timer) = gun.refire_timer {
      if refire_timer > gun_mode_refire_rate {
        gun.refire_timer = None;
      }
    }

    let mut can_shoot = gun.refire_timer.is_none();

    // Player can only shoot Singles when clicking
    if !override_single_mode {
      match gun.mode {
        GunMode::Single | GunMode::Blast => can_shoot = !gun.holding_trigger,
        _ => {}
      }
    }

    can_shoot
  }

  fn get_bullet_prefab(mode: GunMode, is_player: bool, is_boss: bool) -> String {
    let prefab_name = if is_player {
      match mode {
        GunMode::Single => "Single Bullet",
        GunMode::Burst => "Burst Bullet",
        GunMode::Blast => "Blast Bullet",
        // GunMode::PulseBeam => "Pulse Beam Bullet",
      }
    } else if is_boss {
      match mode {
        GunMode::Single => "Boss Bullet",
        GunMode::Burst => "Boss Bullet",
        GunMode::Blast => "Boss Bullet",
        // GunMode::PulseBeam => "Pulse Beam Bullet",
      }
    } else {
      match mode {
        GunMode::Single => "Enemy Blast Bullet",
        GunMode::Burst => "Enemy Blast Bullet",
        GunMode::Blast => "Enemy Blast Bullet",
        // GunMode::PulseBeam => "Enemy Blast Bullet",
      }
    };

    prefab_name.to_string()
  }

  fn get_bullet_direction(spread_angle: Degrees, direction: Vector3<f32>) -> Unit<Vector3<f32>> {
    let half_spread_angle = *spread_angle / 2.0;
    let random_angle_x: f32 = rand::thread_rng().gen_range(-1.0..1.0) * half_spread_angle;
    let random_angle_y: f32 = rand::thread_rng().gen_range(-1.0..1.0) * half_spread_angle;
    let random_angle_z: f32 = rand::thread_rng().gen_range(-1.0..1.0) * half_spread_angle;

    let random_angle_radians_x = random_angle_x.to_radians();
    let random_angle_radians_y = random_angle_y.to_radians();
    let random_angle_radians_z = random_angle_z.to_radians();

    let rotation_matrix_x = Rotation3::from_axis_angle(&Vector3::x_axis(), random_angle_radians_x);
    let rotation_matrix_y = Rotation3::from_axis_angle(&Vector3::y_axis(), random_angle_radians_y);
    let rotation_matrix_z = Rotation3::from_axis_angle(&Vector3::z_axis(), random_angle_radians_z);

    let bullet_direction = rotation_matrix_x * rotation_matrix_y * rotation_matrix_z * direction;
    Unit::new_normalize(bullet_direction)
  }

  fn handle_player_shooting(
    &mut self,
    scene: &mut Scene,
    backpack: &mut Backpack,
    input: &GameInput,
  ) -> Option<()> {
    let mut gun_sound_configurator = None;
    for (_, configurator) in scene.query_mut::<&GunSoundConfigurator>() {
      gun_sound_configurator = Some(configurator.clone());
    }

    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    let mut gun_mode = GunMode::Single;
    let mut player_id = None;
    let mut play_empty_clip_sound = None;

    for (entity, (_, id, physics, gun)) in
      scene.query_mut::<(&SelfComponent, &IdComponent, &PhysicsComponent, &mut Gun)>()
    {
      if gun.is_reloading {
        gun.reload_timer += delta_time;
        if gun.reload_timer > gun.single_reload_duration {
          gun.reload_timer = Seconds::new(0.0);
          gun.is_reloading = false;
          gun.single_ammo = 30;
        }

        continue;
      }

      gun.mode = match (
        input.weapon_slot0,
        input.weapon_slot1,
        input.weapon_slot2,
        input.weapon_slot3,
      ) {
        (true, _, _, _) => GunMode::Single,
        (_, true, _, _) => GunMode::Burst,
        (_, _, true, _) => GunMode::Blast,
        // (_, _, _, true) => GunMode::PulseBeam,
        _ => gun.mode,
      };

      let mut current_weapon_index = gun.mode as i32;
      if input.weapon_previous {
        current_weapon_index -= 1;
      } else if input.weapon_next {
        current_weapon_index += 1;
      }

      let gun_modes_count = variant_count::<GunMode>();
      current_weapon_index = current_weapon_index.max(0).min(gun_modes_count as i32 - 1);
      gun.mode = Self::gun_index_to_mode(current_weapon_index);
      gun_mode = gun.mode;
      let can_gun_shoot = Self::handle_gun_readiness(gun, false, delta_time);

      gun.holding_trigger = input.weapon_shoot;
      gun.is_shooting = false;

      if input.reload && gun_mode == GunMode::Single {
        if gun.single_ammo < 30 {
          gun.single_ammo = 0;
          gun.is_reloading = true;
        }
      }

      if input.weapon_shoot && can_gun_shoot {
        gun.refire_timer = Some(Seconds::zero());

        if !Self::gun_has_ammo(gun) {
          play_empty_clip_sound = Some(entity);
          continue;
        } else if Self::gun_has_low_ammo(gun) {
          play_empty_clip_sound = Some(entity);
        }

        gun.is_shooting = true;

        match gun.mode {
          GunMode::Single => gun.single_ammo -= 1,
          GunMode::Burst => gun.burst_ammo -= 1,
          GunMode::Blast => gun.blast_ammo -= 1,
          // GunMode::PulseBeam => gun.pulse_beam_ammo -= 1,
        }

        player_id = Some((id.clone(), gun.clone(), physics.clone()));
      }
    }

    let mut player_gun = None;
    for (_, (_, gun)) in scene.query_mut::<(&SelfComponent, &mut Gun)>() {
      player_gun = Some(gun.clone());
    }

    if let Some(entity) = play_empty_clip_sound {
      if let Some(gun_sound_configurator) = gun_sound_configurator.clone() {
        self.audio_controller.spawn_sound(
          scene,
          entity,
          gun_sound_configurator.empty,
          Decibels(0.0),
          None,
        );
      }
    }

    let mut gun_model_configurator = None;
    for (_, configurator) in scene.query_mut::<&GunModelConfigurator>() {
      gun_model_configurator = Some(configurator.clone());
    }

    let gun_model_configurator = match gun_model_configurator {
      Some(configurator) => configurator,
      None => return None,
    };

    for (_, (_, model)) in scene.query_mut::<(&CameraComponent, &mut ModelComponent)>() {
      model.id = match gun_mode {
        GunMode::Single => gun_model_configurator.single,
        GunMode::Burst => gun_model_configurator.burst,
        GunMode::Blast => gun_model_configurator.blast,
      };

      let gun = match &player_gun {
        Some(gun) => gun,
        None => continue,
      };

      if !Self::gun_has_ammo(&gun) {
        model.id = gun_model_configurator.reload;
      }
    }

    if let Some((_player_id, gun, physics)) = player_id.clone() {
      let mut camera_transform = None;
      let mut camera_input = None;

      for (_, (transform, input, _camera)) in
        scene.query_mut::<(&TransformComponent, &InputComponent, &CameraComponent)>()
      {
        camera_transform = Some(transform.clone());
        camera_input = Some(input);
        break;
      }

      let camera_transform = camera_transform?;
      let camera_input = camera_input?;

      let transform = camera_transform.world_transform();

      let direction = camera_input.get_front();

      let (num_bullets, spread_angle) = match gun.mode {
        GunMode::Blast => (gun.bullet_amount, gun.spread_angle),
        _ => (1, Degrees::zero()),
      };

      let mut muzzle_transform = None;
      let mut shot_noise = None;

      for (_, (t, model, muzzle, _)) in scene.query_mut::<(
        &TransformComponent,
        &mut ModelComponent,
        &Muzzle,
        &ParentComponent,
      )>() {
        let rand = rand::thread_rng().gen_range(0.08..0.14);
        muzzle_transform = Some(t);
        shot_noise = Some(muzzle.noise_prefab);
        model.transform.scale = Vector3::new(rand, rand, rand);
        model.skip = false;
        break;
      }
      let transform_to_use = muzzle_transform?.get();

      let center = transform.translation;
      let max_distance = *gun.max_distance;
      let rigid_body_handle = match self
        .physics_controller
        .get_rigid_body(&physics.joint.body.id)
      {
        Some(handle) => handle,
        None => return None,
      };
      let filter = QueryFilter::default()
        .exclude_rigid_body(rigid_body_handle)
        .exclude_sensors();
      let solid = false;

      let ray = Ray::new(center.into(), direction.into_inner());
      let mut new_direction = Vector3::new(0.0, 0.0, 0.0);

      // Doesn't work
      // let muzzle_world_transform = muzzle_transform?.world_transform();

      // Works
      let muzzle_world_transform = camera_transform.world_transform() * transform_to_use;
      scene.spawn_prefab_id_with(shot_noise.unwrap(), |prefab| {
        prefab.transform.translation = muzzle_world_transform.translation;
      });

      if let Some((_, _handle, intersection)) =
        self
          .physics_controller
          .raycast(&ray, max_distance, solid, filter)
      {
        let x = ray.point_at(intersection.toi).x - muzzle_world_transform.translation.x;
        let y = ray.point_at(intersection.toi).y - muzzle_world_transform.translation.y;
        let z = ray.point_at(intersection.toi).z - muzzle_world_transform.translation.z;
        new_direction.x = x;
        new_direction.y = y;
        new_direction.z = z;
      } else {
        new_direction = *direction;
      }

      // let bullet_prefab_name = Self::gun_mode_to_prefab_name(gun.mode);
      for _i in 0..num_bullets {
        let bullet_direction = Self::get_bullet_direction(spread_angle, new_direction);
        let bullet_entity = self.pool.get();

        let bullet_quaternion = physics::direction_to_quaternion(bullet_direction);
        let bullet_rotation = bullet_quaternion.euler_angles();
        let bullet_rotation = Vector3::new(bullet_rotation.0, bullet_rotation.1, bullet_rotation.2);

        let prefab_name = Self::get_bullet_prefab(gun.mode, true, false);
        scene.spawn_entity_and_prefab_with(bullet_entity, &prefab_name, |prefab| {
          prefab.transform.translation =
            muzzle_world_transform.translation + *bullet_direction * 2.0;
          prefab.transform.rotation = bullet_rotation;

          if let Some(bullet) = prefab.get_mut::<Bullet>() {
            bullet.direction = bullet_direction;
            bullet.damage = gun.damage;
            bullet.shot_from = transform.translation;
          };
        });

        // DEBUG ONLY
        // #[cfg(feature = "debug-bullets")]
        // {
        //   use engine::systems::rendering::DebugController;
        //   use nalgebra::Vector4;

        //   if let Some(debug_controller) = backpack.get_mut::<DebugController>() {
        //     debug_controller.draw_ray(
        //       transform.translation,
        //       *bullet_direction * *gun.max_distance,
        //       Vector4::new(1.0, 1.0, 0.0, 1.0),
        //       10.0,
        //     );
        //   }
        // }

        // DEBUG END
        // self.spawn_bullet(scene, bullet_direction, transform, bullet_prefab_name);

        // self.spawn_impact_mesh(
        //   scene,
        //   Vector3::new(hit.x, hit.y, hit.z),
        //   bullet_direction,
        //   transform,
        //   gun.id,
        //   gun.particle_id,
        //   bullet_prefab_name,
        // );
      }
    } else {
      for (_, (_, model, _, _)) in scene.query_mut::<(
        &TransformComponent,
        &mut ModelComponent,
        &Muzzle,
        &ParentComponent,
      )>() {
        model.skip = true;
        break;
      }
    }

    Some(())
  }

  fn handle_enemy_shooting(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    // Get all alive players
    let mut alive_players = vec![];
    for (entity, (_player, health, transform, _physics)) in
      scene.query_mut::<(&Player, &Health, &TransformComponent, &PhysicsComponent)>()
    {
      if health.current_health > 0.0 {
        alive_players.push((entity, transform.clone()));
      }
    }

    if alive_players.is_empty() {
      return None;
    }

    let mut shooters = vec![];
    let mut is_boss = false;
    for (_, (enemy, health, transform, physics, gun, maybe_boss)) in scene.query_mut::<(
      &Enemy,
      &Health,
      &TransformComponent,
      &PhysicsComponent,
      &mut Gun,
      Option<&Boss>,
    )>() {
      if !gun.is_shooting || health.current_health <= 0.0 {
        continue;
      }

      let can_gun_shoot = Self::handle_gun_readiness(gun, true, delta_time);
      gun.trigger_shooting_anim_enemy = can_gun_shoot;
      if !can_gun_shoot || !Self::gun_has_ammo(gun) {
        continue;
      }

      gun.refire_timer = Some(Seconds::zero());
      if gun.mode == GunMode::Single {
        gun.single_ammo -= 1;
      } else if gun.mode == GunMode::Burst {
        gun.burst_ammo -= 1;
      } else if gun.mode == GunMode::Blast {
        gun.blast_ammo -= 1;
      }

      // The Shoot Action (GOAP) says we should be shooting a player
      // Find the neareast player the enemy can see and shoot at them
      let mut nearest_seen_player = None;
      for (player_entity, player_transform) in alive_players.iter() {
        let direction = player_transform.translation - transform.translation;
        let distance = direction.magnitude();

        let enemy_handle = match self
          .physics_controller
          .get_rigid_body(&physics.joint.body.id)
        {
          Some(handle) => handle,
          None => continue,
        };

        if self.enemy_has_line_of_sight(
          &enemy_handle,
          transform,
          gun.max_distance,
          &player_entity,
          player_transform,
          backpack,
        ) {
          if nearest_seen_player.is_none() {
            nearest_seen_player = Some((player_transform.clone(), distance));
          } else {
            if let Some((_nearest_transform, nearest_distance)) = nearest_seen_player {
              if distance < nearest_distance {
                nearest_seen_player = Some((player_transform.clone(), distance));
              }
            }
          }
        }
      }

      if let Some((player_transform, _)) = nearest_seen_player {
        shooters.push((
          enemy.clone(),
          transform.clone(),
          physics.clone(),
          player_transform,
          gun.clone(),
        ));
      }
      if maybe_boss.is_some() {
        is_boss = true;
      }
    }

    for (enemy_component, enemy_transform, _, player_transform, gun) in shooters.drain(..) {
      let transform = enemy_transform.get();

      let direction = player_transform.translation - transform.translation;
      let direction = Unit::new_normalize(direction);

      let (num_bullets, mut spread_angle) = match gun.mode {
        GunMode::Blast => (gun.bullet_amount, gun.spread_angle),
        _ => (1, Degrees::new(0.0)),
      };

      spread_angle += enemy_component.aim_inaccuracy_degrees;

      // let start = transform.translation + Vector3::y() * 0.75;
      // let bullet_prefab_name = Self::gun_mode_to_prefab_name(gun.mode);

      log::info!("spawning {:} bullets", num_bullets);
      for _i in 0..num_bullets {
        let bullet_direction = Self::get_bullet_direction(spread_angle, *direction);
        let bullet_entity = self.pool.get();

        let bullet_quaternion = physics::direction_to_quaternion(bullet_direction);
        let bullet_rotation = bullet_quaternion.euler_angles();
        let bullet_rotation = Vector3::new(bullet_rotation.0, bullet_rotation.1, bullet_rotation.2);

        let prefab_name = if is_boss {
          Self::get_bullet_prefab(gun.mode, false, true)
        } else {
          Self::get_bullet_prefab(gun.mode, false, false)
        };

        scene.spawn_entity_and_prefab_with(bullet_entity, &prefab_name, |prefab| {
          prefab.transform.translation = transform.translation + *bullet_direction * 2.0;
          prefab.transform.rotation = bullet_rotation;

          if let Some(bullet) = prefab.get_mut::<Bullet>() {
            bullet.direction = bullet_direction;
            bullet.damage = gun.damage;
            bullet.shot_from = transform.translation;
          };
        });

        // // DEBUG ONLY
        // #[cfg(feature = "debug-bullets")]
        // {
        //   use engine::systems::rendering::DebugController;
        //   use nalgebra::Vector4;

        //   if let Some(debug_controller) = backpack.get_mut::<DebugController>() {
        //     debug_controller.draw_ray(
        //       start,
        //       *bullet_direction * *gun.max_distance,
        //       Vector4::new(1.0, 1.0, 0.0, 1.0),
        //       10.0,
        //     );
        //   }
        // }

        // // DEBUG END
        // self.spawn_bullet(scene, bullet_direction, transform, bullet_prefab_name);

        // self.spawn_impact_mesh(
        //   scene,
        //   Vector3::new(hit.x, hit.y, hit.z),
        //   bullet_direction,
        //   transform,
        //   gun.id,
        //   gun.particle_id,
        //   bullet_prefab_name,
        // );
      }
    }

    Some(())
  }

  fn enemy_has_line_of_sight(
    &mut self,
    enemy_handle: &RigidBodyHandle,
    enemy_translation: &TransformComponent,
    max_distance: Meters,
    target_entity: &Entity,
    end_translation: &TransformComponent,
    _: &mut Backpack,
  ) -> bool {
    let filter = QueryFilter::default()
      .exclude_rigid_body(*enemy_handle)
      .exclude_sensors();

    for i in 1..4 {
      let start = enemy_translation.translation + Vector3::y() * 0.75;
      let finish = end_translation.translation + Vector3::y() * 0.25 * (i as f32);

      let direction = finish - start;
      let direction = Unit::new_normalize(direction);

      #[cfg(feature = "debug-bullets")]
      {
        use engine::systems::rendering::DebugController;
        use nalgebra::Vector4;

        if let Some(debug_controller) = backpack.get_mut::<DebugController>() {
          debug_controller.draw_ray(
            start,
            *direction * *max_distance,
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            0.1,
          );
        }
      }

      let ray = Ray::new(Point3::from(start), *direction);
      if let Some((entity, _, _)) =
        self
          .physics_controller
          .raycast(&ray, *max_distance, true, filter)
      {
        if entity == *target_entity {
          return true;
        } else {
          continue;
        }
      }
    }

    false
  }

  fn spawn_impact_mesh(
    &mut self,
    scene: &mut Scene,
    impact_point: Vector3<f32>,
    direction: Unit<Vector3<f32>>,
    transform: Transform,
    gun_id: Uuid,
    particle_id: ParticleId,
    prefab_name: &str,
  ) {
    let entity = scene.spawn_prefab(prefab_name);

    let quaternion = physics::direction_to_quaternion(direction);

    let mut world_transform = transform;
    world_transform.translation += Vector3::y() * 0.5;
    world_transform.rotation = quaternion;
    world_transform.translation = impact_point;

    if let Some((entity, prefab)) = entity {
      if let Some(physics) = prefab.get::<PhysicsComponent>() {
        self
          .physics_controller
          .insert_with_filter(entity, &physics, world_transform, vec![]);
        self
          .particle_controller
          .emit(particle_id, gun_id, impact_point, direction);
      }
    }
  }

  fn spawn_bullet(
    &mut self,
    scene: &mut Scene,
    direction: Unit<Vector3<f32>>,
    transform: Transform,
    prefab_name: &str,
  ) {
    let entity = scene.spawn_prefab(prefab_name);

    let quaternion = physics::direction_to_quaternion(direction);
    let mut world_transform = transform;
    world_transform.translation += Vector3::y() * 0.5;
    world_transform.rotation = quaternion;

    log::info!("spawning bullet");

    if let Some((entity, prefab)) = entity {
      log::info!("spawning bullet...");
      if let Some(physics) = prefab.get::<PhysicsComponent>() {
        log::info!("bullet spawned");
        self
          .physics_controller
          .insert_with_filter(entity, &physics, world_transform, vec![]);

        self
          .physics_controller
          .set_linear_velocity(&physics, direction.into_inner() * 50.0);
      }
    }
  }

  pub fn handle_damage(&mut self, scene: &mut Scene) {
    let mut entities = vec![];

    let query = scene.query_mut::<(&Player, &Collision<Bullet, PhysicsComponent>)>();
    for (entity, (_, collision)) in query {
      entities.push((entity.clone(), collision.other));
    }

    let query = scene.query_mut::<(&Enemy, &Collision<Bullet, PhysicsComponent>)>();
    for (entity, (_, collision)) in query {
      entities.push((entity.clone(), collision.other));
    }

    for (entity, bullet_entity) in entities {
      let bullet = match scene.get_components_mut::<&Bullet>(bullet_entity) {
        Some(bullet) => bullet.clone(),
        _ => continue,
      };

      let mut angle_between_shot_front = 0.0;
      let player_position;

      if let Some((_, transform)) =
        scene.get_components_mut::<(&Player, &TransformComponent)>(entity)
      {
        player_position = transform.translation;

        for (_, (_, _, input)) in
          scene.query_mut::<(&mut TransformComponent, &CameraComponent, &InputComponent)>()
        {
          let camera_direction = *input.get_front();
          let shot_from_direction = bullet.shot_from - player_position;

          let dot_camera_shotfrom = camera_direction.dot(&shot_from_direction);

          let magnitude_camera_direction = camera_direction.magnitude();
          let magnitude_shot_from_direction = shot_from_direction.magnitude();

          let cos_theta =
            dot_camera_shotfrom / (magnitude_shot_from_direction * magnitude_camera_direction);

          let cross_camera_shotfrom = camera_direction.cross(&shot_from_direction);

          let cross_magnitude = cross_camera_shotfrom.magnitude();

          let sin_theta =
            cross_magnitude / (magnitude_camera_direction * magnitude_shot_from_direction);

          let sign;

          if cross_camera_shotfrom.y < 0.0 {
            sign = -1.0
          } else {
            sign = 1.0
          };

          let angle_in_radians = sign * sin_theta.atan2(cos_theta);
          angle_between_shot_front = angle_in_radians * (180.0 / PI);
        }
      }

      let mut has_barrier = false;
      if let Some(barrier) = scene.get_components_mut::<&mut Barrier>(entity) {
        if barrier.current_barrier > 0.0 {
          has_barrier = true;
        } else {
          has_barrier = false;
        }
      }

      if let Some(health) = scene.get_components_mut::<&mut Health>(entity) {
        if has_barrier == false {
          health.current_health -= bullet.damage;
          health.recently_damaged = true;
          health.last_damaged_from = Some(bullet.shot_from)
        }
      }

      if let Some((player_health, _)) =
        scene.get_components_mut::<(&mut Health, &SelfComponent)>(entity)
      {
        player_health.angle = angle_between_shot_front;
        if player_health.angle > 45.0 || player_health.angle < -45.0 {
          player_health.blood_timer = Seconds::new(0.0);
          player_health.start_blood_timer = true;
        }
      }
    }
  }

  pub fn move_bullets(&mut self, scene: &mut Scene, backpack: &Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    for (_, (transform, physics, bullet)) in
      scene.query_mut::<(&mut TransformComponent, &mut PhysicsComponent, &mut Bullet)>()
    {
      let mps: Mps = bullet.speed.into();
      let displacement = *mps * *delta_time;
      self.physics_controller.set_kinematic_translation(
        physics,
        transform.translation + *bullet.direction * displacement,
      );
    }
  }

  fn remove_bullets(&mut self, scene: &mut Scene) {
    let mut colliding = vec![];
    for (entity, (collision, _)) in
      scene.query_mut::<(&Collision<Bullet, PhysicsComponent>, &Bullet)>()
    {
      colliding.push((entity, collision.other));
    }

    for (entity, other) in colliding {
      let other_physics = match scene.get_components_mut::<&mut PhysicsComponent>(other) {
        Some(physics) => physics.clone(),
        None => continue,
      };

      if other_physics.joint.body.is_sensor {
        continue;
      }

      let physics = scene
        .get_components_mut::<&mut PhysicsComponent>(entity)
        .unwrap()
        .clone();

      self.physics_controller.despawn(&physics);

      self.pool.recycle(scene, entity);
      scene.remove_collision::<Bullet, PhysicsComponent>(entity, other);
    }
  }
}

impl System for ShootingSystem {
  fn get_name(&self) -> &'static str {
    "ShootingSystem"
  }

  fn attach(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    // Pre-allocate pool of bullets. That way, we don't thrash memory
    self.pool.initialize(scene);
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let input = self.inputs.read();
    self.handle_player_shooting(scene, backpack, &input);
    self.handle_enemy_shooting(scene, backpack);

    self.move_bullets(scene, backpack);
    self.handle_damage(scene);
    self.remove_bullets(scene);
  }
}
