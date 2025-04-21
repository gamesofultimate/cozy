use engine::{
  application::scene::{PrefabId, ProvideAssets},
  resources::{audio::AudioId, model::ModelId, particles::ParticleId, sprite::SpriteId},
  systems::Registry,
  utils::{
    easing::Easing,
    interpolation::Interpolator,
    units::{Decibels, Degrees, Kph, Meters, Mps, Rps, Seconds},
  },
  nalgebra::{Unit, Vector3},
};
use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct GameComponents;

impl Registry for GameComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;
    AnimatedCamera::register();
    Movement::register();
    Health::register();
    Oxygen::register();
    MissionTimer::register();
    LineOfSight::register();
    Pickup::register();
    DoorSensor::register();
    Player::register();
    Gun::register();
    Throwable::register();
    Grenade::register();
    GrenadeNoise::register();
    Enemy::register();
    Bullet::register();
    Door::register();
    Noise::register();
    Lift::register();
    LiftSensor::register();
    Patroller::register();
    PatrolPath::register();
    Lifecycle::register();
    Interactable::register();
    Conversation::register();
    Generator::register();
    GeneratorLight::register();
    GeneratorPowered::register();
    LockdownEnabler::register();
    LockdownDisabler::register();
    LockdownAffected::register();
    LightInterpolation::register();
    CameraShake::register();
    GunModelConfigurator::register();
    GunSoundConfigurator::register();
    ParticleEmitter::register();
    HealthRecovery::register();
    CameraTrackActivator::register();
    CameraTrack::register();
    PlayerMechanicModifier::register();
    Objective::register();
    Platform::register();
    PlatformTrigger::register();
    Preloader::register();
    LevelLoader::register();
    LevelLoaderTrigger::register();
    LevelStart::register();
    LevelLoaderIgnore::register();
    Muzzle::register();
    IntroTrigger::register();
    IntroEnd::register();
    Checkpoint::register();
    Boss::register();
    BossSensor::register();
    BossCameraSensor::register();
    Spawner::register();
    Barrier::register();
    TutorialSensor::register();
    RotationalMovement::register();
    Explosion::register();
    AudioFade::register();
    FadeSensor::register();
    Safezone::register();
    Citizen::register();
    Goal::register();
    Ball::register();
    Floor::register();
    LoadingCameraTrack::register();
    LevelName::register();
  }
}

fn default_mission_time() -> Seconds {
  return Seconds::new(600.0);
}

fn default_crouching_speed() -> Kph {
  return Kph::new(2.0);
}

fn default_dashing_speed() -> Kph {
  return Kph::new(40.0);
}

fn default_dash_time() -> Seconds {
  Seconds::new(0.15)
}

fn default_dash_cooldown() -> Seconds {
  Seconds::new(5.0)
}

fn default_jump_peak_time() -> Seconds {
  Seconds::new(0.5)
}

fn default_jump_fall_time() -> Seconds {
  Seconds::new(0.5)
}

fn default_jump_distance() -> Meters {
  Meters::new(4.0)
}

fn default_jump_height() -> Meters {
  Meters::new(2.0)
}

fn default_max_num_jumps() -> u32 {
  2
}

fn default_time_between_jumps() -> Seconds {
  Seconds::new(0.3)
}

fn default_ground_check_distance() -> Meters {
  Meters::new(1.5)
}

fn default_oxygen() -> f32 {
  100.0
}

fn default_oxygen_loss() -> f32 {
  1.0
}

fn default_health_loss() -> f32 {
  10.0
}

fn default_low_percentage_oxy() -> f32 {
  20.0
}

fn default_one_way() -> LiftType {
  LiftType::ReturnOnLeave
}

fn default_deactivate() -> bool {
  false
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Movement {
  pub walking_speed: Kph,
  pub running_speed: Kph,

  #[serde(default = "default_crouching_speed")]
  #[schema(default = "{ kph: 2.0 }")]
  pub crouching_speed: Kph,
  #[serde(default = "default_dashing_speed")]
  #[schema(default = "{ kph: 40.0 }")]
  pub dashing_speed: Kph,
  #[serde(default = "default_dash_time")]
  #[schema(default = "{ seconds: 0.15 }")]
  pub dash_time: Seconds,
  #[serde(default = "default_dash_cooldown")]
  #[schema(default = "{ seconds: 5.0 }")]
  pub dash_cooldown: Seconds,
  #[serde(default = "default_jump_peak_time")]
  #[schema(default = "{ seconds: 0.5 }")]
  pub jump_peak_time: Seconds,
  #[serde(default = "default_jump_fall_time")]
  #[schema(default = "{ seconds: 0.5 }")]
  pub jump_fall_time: Seconds,
  #[serde(default = "default_jump_height")]
  #[schema(default = "{ meters: 2.0 }")]
  pub jump_height: Meters,
  #[serde(default = "default_jump_distance")]
  #[schema(default = "{ meters: 4.0 }")]
  pub jump_distance: Meters,
  #[serde(default = "default_ground_check_distance")]
  #[schema(default = "{ meters: 1.5 }")]
  pub distance_to_ground_check: Meters,
  #[serde(default = "default_max_num_jumps")]
  #[schema(default = "2")]
  pub max_num_jumps: u32,
  #[serde(default = "default_time_between_jumps")]
  #[schema(default = "{ seconds: 0.3 }")]
  pub time_between_jumps: Seconds,

  #[serde(skip)]
  pub dash_timer: Seconds,
  #[serde(skip)]
  pub dash_cooldown_timer: Seconds,
  #[serde(skip)]
  pub is_dashing: bool,
  #[serde(skip)]
  pub is_jumping: bool,
  #[serde(skip)]
  pub is_grounded: bool,
  #[serde(skip)]
  pub y_velocity: Mps,
  #[serde(skip)]
  pub jump_count: u32,
  #[serde(skip)]
  pub extra_jump_timer: Seconds,
}

impl ProvideAssets for Movement {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct MissionTimer {
  #[serde(default = "default_mission_time")]
  #[schema(default = "{ seconds: 600 }")]
  pub current_time: Seconds,
  #[serde(default = "default_mission_time")]
  #[schema(default = "{ seconds: 600 }")]
  pub max_time: Seconds,
}

impl ProvideAssets for MissionTimer {}

fn default_low_percentage_health() -> f32 {
  20.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Health {
  pub current_health: f32,
  pub total_health: f32,

  #[serde(default = "default_low_percentage_health")]
  #[schema(default = "20.0")]
  pub low_percentage: f32,

  #[serde(skip)]
  pub recently_damaged: bool,

  #[serde(skip)]
  pub start_blood_timer: bool,

  #[serde(skip)]
  pub last_damaged_from: Option<Vector3<f32>>,

  #[serde(skip)]
  pub angle: f32,

  #[serde(skip)]
  pub blood_timer: Seconds,
}

fn default_active() -> bool {
  true
}

impl ProvideAssets for Health {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Oxygen {
  #[serde(default = "default_active")]
  #[schema(default = "true")]
  pub active: bool,
  #[serde(default = "default_oxygen")]
  #[schema(default = "600")]
  pub current_oxygen: f32,
  #[serde(default = "default_oxygen")]
  #[schema(default = "600")]
  pub total_oxygen: f32,
  #[serde(default = "default_oxygen_loss")]
  #[schema(default = "1.0")]
  pub oxygen_loss_per_second: f32,
  #[serde(default = "default_health_loss")]
  #[schema(default = "10.0")]
  pub health_loss_per_second: f32,
  #[serde(default = "default_low_percentage_oxy")]
  #[schema(default = "20.0")]
  pub low_percentage: f32,
}

impl ProvideAssets for Oxygen {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Pickup {}

impl ProvideAssets for Pickup {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Ammo {
  gun_mode: GunMode,
  ammo_amount: u32,
}

impl ProvideAssets for Ammo {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Player {
  #[serde(skip)]
  pub auto_destination: Option<Vector3<f32>>,

  #[serde(skip)]
  pub death_transition_timer: Seconds,

  #[serde(skip)]
  pub death_transition: bool,
}

impl ProvideAssets for Player {}

pub fn default_aim_inaccuracy() -> Degrees {
  Degrees::new(5.0)
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Enemy {
  #[serde(default = "default_aim_inaccuracy")]
  #[schema(default = "{ meters: 5.0 }")]
  pub aim_inaccuracy_degrees: Degrees,

  #[serde(skip)]
  pub active: bool,
}

impl ProvideAssets for Enemy {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Noise {}

pub fn default_direction() -> Unit<Vector3<f32>> {
  Unit::new_normalize(Vector3::z())
}

impl ProvideAssets for Noise {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Bullet {
  pub speed: Kph,

  #[serde(default = "default_direction")]
  #[schema(default = "[1.0, 0.0, 0.0]")]
  pub direction: Unit<Vector3<f32>>,

  #[serde(skip)]
  pub shot_from: Vector3<f32>,

  #[serde(skip)]
  pub damage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Ball {}
impl ProvideAssets for Ball {}

#[derive(
  Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate,
)]
pub enum GoalId {
  Team1,
  Team2,
}
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Goal {
  pub team: GoalId,
}
impl ProvideAssets for Goal {}


#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Floor {
}
impl ProvideAssets for Floor {}
impl ProvideAssets for Bullet {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LineOfSight {}

impl ProvideAssets for LineOfSight {}

#[derive(
  Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate,
)]
pub enum GunMode {
  Single,
  Burst,
  Blast,
  // PulseBeam,
}

fn default_gun_mode() -> GunMode {
  GunMode::Single
}

fn default_refire_rate() -> Seconds {
  Seconds::new(1.0)
}

fn default_burst_multiplier() -> f32 {
  10.0
}

fn default_spread_angle() -> Degrees {
  Degrees::new(5.0)
}

fn default_bullet_amount() -> i32 {
  8
}

fn default_burst_ammo() -> i32 {
  60
}

fn default_blast_ammo() -> i32 {
  12
}

fn default_single_ammo() -> i32 {
  40
}

fn default_single_reload_duration() -> Seconds {
  Seconds::new(2.0)
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Gun {
  pub max_distance: Meters,
  pub damage: f32,

  pub particle_id: ParticleId,
  pub id: Uuid,

  #[serde(default = "default_single_ammo")]
  pub single_ammo: i32,

  #[serde(default = "default_burst_ammo")]
  pub burst_ammo: i32,

  #[serde(default = "default_blast_ammo")]
  pub blast_ammo: i32,

  #[serde(default = "default_single_reload_duration")]
  #[schema(default = "{ seconds: 2.0 }")]
  pub single_reload_duration: Seconds,

  #[serde(default = "default_refire_rate")]
  pub refire_rate: Seconds,

  #[serde(default = "default_burst_multiplier")]
  pub burst_multiplier: f32,

  #[serde(default = "default_bullet_amount")]
  pub bullet_amount: i32,

  #[serde(default = "default_spread_angle")]
  pub spread_angle: Degrees,

  #[serde(default = "default_gun_mode")]
  pub mode: GunMode,

  #[serde(skip)]
  pub current_rounds: f32,

  #[serde(skip)]
  pub is_shooting: bool,

  #[serde(skip)]
  pub trigger_shooting_anim_enemy: bool,

  #[serde(skip)]
  pub refire_timer: Option<Seconds>,

  #[serde(skip)]
  pub holding_trigger: bool,

  #[serde(skip)]
  pub is_reloading: bool,

  #[serde(skip)]
  pub reload_timer: Seconds,
}

impl ProvideAssets for Gun {}

fn default_velocity_multiplier() -> Mps {
  return Mps::new(12.0);
}

fn default_angular_dampening() -> f32 {
  return 25.0;
}

fn default_scale_multiplier() -> f32 {
  return 1.0;
}

fn default_grenade_cooldown() -> Seconds {
  return Seconds::new(10.0);
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Throwable {
  pub active: bool,

  #[serde(default = "default_grenade_cooldown")]
  pub grenade_cooldown: Seconds,
  pub charge_time: Seconds,
  pub item: PrefabId,

  #[serde(default = "default_velocity_multiplier")]
  pub velocity: Mps,

  #[serde(default = "default_angular_dampening")]
  pub angular_dampening: f32,

  #[serde(default = "default_scale_multiplier")]
  pub scale_multiplier: f32,

  #[serde(skip)]
  pub current_charge: Seconds,

  #[serde(default = "default_grenade_cooldown")]
  #[serde(skip)]
  pub cooldown_timer: Seconds,
}

impl ProvideAssets for Throwable {}

fn default_minimum_teleport() -> Seconds {
  return Seconds::new(1.0);
}

fn default_has_teleport() -> bool {
  return true;
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Grenade {
  pub explosion_prefab: PrefabId,
  pub max_time_before_detonation: Seconds,
  #[serde(default = "default_minimum_teleport")]
  pub minimum_time_for_teleport: Seconds,
  pub explosion_radius: Meters,
  pub damage: f32,
  pub noise: PrefabId,

  #[serde(default = "default_has_teleport")]
  #[schema(default = "true")]
  pub has_teleport: bool,

  #[serde(skip)]
  pub is_teleport_ready: bool,

  #[serde(skip)]
  pub current_time: Seconds,
}

impl ProvideAssets for Grenade {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct GrenadeNoise {}

impl Default for Grenade {
  fn default() -> Self {
    Self {
      explosion_prefab: PrefabId::default(),
      max_time_before_detonation: Seconds::new(3.0),
      minimum_time_for_teleport: Seconds::new(1.0),
      explosion_radius: Meters::new(5.0),
      damage: 50.0,
      noise: PrefabId::default(),
      has_teleport: true,

      is_teleport_ready: false,
      current_time: Seconds::new(0.0),
    }
  }
}

impl ProvideAssets for GrenadeNoise {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct DoorSensor {
  pub door: PrefabId,
}

impl ProvideAssets for DoorSensor {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LiftSensor {
  pub lift: PrefabId,
}

impl ProvideAssets for LiftSensor {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum Direction {
  X { offset: f32 },
  Y { offset: f32 },
  Z { offset: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum DoorState {
  Closed,
  Opening { origin: Vector3<f32> },
  Open,
  Closing { origin: Vector3<f32> },
}

fn default_door_state() -> DoorState {
  DoorState::Closed
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Door {
  pub opening_speed: Kph,
  pub offset: Direction,
  #[serde(skip, default = "default_door_state")]
  pub door_state: DoorState,
}

impl ProvideAssets for Door {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum LiftState {
  Stopped0,
  Moving0 { origin: Vector3<f32> },
  Stopped1,
  Moving1 { origin: Vector3<f32> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum LiftType {
  OneWayOnly,    //get in the lift, moves to target, does not go back
  ReturnOnLeave, //get in the lift, moves to target, returns to base when player leaves
  MoveOnEnter, //get in the lift, moves to target, leave lift. get in the lift again, moves to base
}

fn default_lift_state() -> LiftState {
  LiftState::Stopped0
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Lift {
  pub moving_speed: Kph,
  pub offset: Direction,
  #[serde(skip, default = "default_lift_state")]
  pub lift_state: LiftState,
  #[serde(default = "default_one_way")]
  #[schema(default = "false")]
  pub lift_type: LiftType,
  #[serde(default = "default_deactivate")]
  #[schema(default = "false")]
  pub deactivate: bool,
  #[serde(skip)]
  pub direction_val: i32,
}

impl ProvideAssets for Lift {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Patroller {
  pub patrol_path: PrefabId,
}

impl ProvideAssets for Patroller {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct PatrolPath {
  pub path: Vec<PrefabId>,
}

impl ProvideAssets for PatrolPath {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Lifecycle {
  #[serde(default = "default_lifetime")]
  #[schema(default = "{ seconds: 3 }")]
  pub lifetime: Seconds,

  #[schema(default = "false")]
  pub is_dead: bool,

  #[schema(default = "false")]
  #[serde(default = "default_despawn_physics_immediately")]
  pub despawn_physics_immediately: bool,

  #[serde(skip)]
  pub lifetime_timer: Seconds,
  #[serde(skip)]
  pub is_marked_for_despawn: bool,
}

impl ProvideAssets for Lifecycle {}

fn default_lifetime() -> Seconds {
  return Seconds::new(3.0);
}

fn default_despawn_physics_immediately() -> bool {
  return false;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum Interaction {
  Collision,
  ActionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Interactable {
  pub action: Interaction,

  #[serde(default = "default_one_time_use")]
  #[schema(default = "true")]
  pub one_time_use: bool,

  #[serde(skip)]
  pub has_activated: bool,

  #[serde(skip)]
  pub ignore: bool,
}

impl ProvideAssets for Interactable {}

fn default_one_time_use() -> bool {
  true
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Dialog {
  pub text: String,
  pub duration: Seconds,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Conversation {
  pub dialog: Vec<Dialog>,

  #[serde(skip)]
  pub current_duration_timer: Seconds,

  #[serde(skip)]
  pub active: bool,

  #[serde(skip)]
  pub index: usize,
}

impl ProvideAssets for Conversation {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Generator {
  pub active: bool,
}

impl ProvideAssets for Generator {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct GeneratorLight {
  pub turn_on_delay: Seconds,
  pub turn_off_delay: Seconds,

  #[serde(default = "Default::default")]
  pub inverted: bool,

  #[serde(skip)]
  pub cached_intensity: f32,

  #[serde(skip)]
  pub current_timer: Seconds,
}

impl ProvideAssets for GeneratorLight {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct GeneratorPowered {}

impl ProvideAssets for GeneratorPowered {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LockdownEnabler {
  #[serde(skip)]
  pub has_activated: bool,
}

impl ProvideAssets for LockdownEnabler {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LockdownDisabler {
  #[serde(skip)]
  pub has_activated: bool,
}

impl ProvideAssets for LockdownDisabler {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LockdownAffected {}

impl ProvideAssets for LockdownAffected {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Lockdown {
  pub state: bool,
}

impl ProvideAssets for Lockdown {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LightInterpolation {
  pub enabled: bool,

  #[schema(default = "[1.0, 1.0, 1.0]", component = "Rgb")]
  pub start_radiance: Vector3<f32>,
  #[schema(default = "[1.0, 1.0, 1.0]", component = "Rgb")]
  pub end_radiance: Vector3<f32>,

  pub frequency: Seconds,

  #[serde(skip)]
  pub current_timer: Seconds,
}

impl ProvideAssets for LightInterpolation {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct CameraShake {
  #[schema(default = "'Linear'")]
  pub damage_shake_easing: Easing,
  #[schema(default = "{ seconds: 0.1 }")]
  pub damage_shake_duration: Seconds,
  #[schema(default = "0.05")]
  pub damage_shake_strength: f32,

  #[schema(default = "'Linear'")]
  pub grenade_shake_easing: Easing,
  #[schema(default = "{ seconds: 0.1 }")]
  pub grenade_shake_duration: Seconds,
  #[schema(default = "0.05")]
  pub grenade_shake_strength: f32,
  #[schema(default = "{ meters: 0.1 }")]
  pub grenade_shake_radius: Meters,

  #[serde(skip)]
  pub current_damage_shake_direction: Vector3<f32>,
  #[serde(skip)]
  pub current_damage_shake_strength: f32,
  #[serde(skip)]
  pub current_damage_timer: Seconds,

  #[serde(skip)]
  pub current_grenade_shake_direction: Vector3<f32>,
  #[serde(skip)]
  pub current_grenade_shake_strength: f32,
  #[serde(skip)]
  pub current_grenade_timer: Seconds,
}

impl ProvideAssets for CameraShake {}

impl Default for CameraShake {
  fn default() -> Self {
    Self {
      damage_shake_easing: Easing::Linear,
      damage_shake_duration: Seconds::new(0.1),
      damage_shake_strength: 0.05,

      current_damage_shake_direction: Vector3::zeros(),
      current_damage_shake_strength: 0.0,
      current_damage_timer: Seconds::new(0.0),

      grenade_shake_easing: Easing::Linear,
      grenade_shake_duration: Seconds::new(0.1),
      grenade_shake_strength: 0.05,
      grenade_shake_radius: Meters::new(0.1),

      current_grenade_shake_direction: Vector3::zeros(),
      current_grenade_shake_strength: 0.0,
      current_grenade_timer: Seconds::new(0.0),
    }
  }
}

// NOTE: This should actually provide assets.
// Creating a `Preloader` is an anti-pattern.
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct GunModelConfigurator {
  pub hands: ModelId,
  pub single: ModelId,
  pub burst: ModelId,
  pub blast: ModelId,
  pub reload: ModelId,
}

impl ProvideAssets for GunModelConfigurator {}

// NOTE: This should actually provide assets.
// Creating a `Preloader` is an anti-pattern.
#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct GunSoundConfigurator {
  pub single: AudioId,
  pub burst: AudioId,
  pub blast: AudioId,
  pub empty: AudioId,
}
impl ProvideAssets for GunSoundConfigurator {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]

pub struct ParticleEmitter {
  pub particle_id: ParticleId,

  #[serde(skip)]
  pub has_activated: bool,
}

impl ProvideAssets for ParticleEmitter {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct HealthRecovery {
  pub capacity: f32,
  pub consumption_per_second: f32,
}

impl ProvideAssets for HealthRecovery {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Schema, Duplicate)]
pub struct CameraPathNode {
  pub prefab: PrefabId,
  pub duration: Seconds,
  pub easing: Easing,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct CameraTrackActivator {
  pub camera_track: PrefabId,
}

impl ProvideAssets for CameraTrackActivator {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct CameraTrack {
  pub nodes: Vec<CameraPathNode>,

  #[serde(skip)]
  pub is_running: bool,

  #[serde(skip)]
  pub current_index: usize,

  #[serde(skip)]
  pub current_timer: Seconds,
}

impl ProvideAssets for CameraTrack {}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Duplicate)]
pub struct JumpMechanic {
  pub max_num_jumps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Duplicate)]
pub struct GrenadeMechanic {
  pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Duplicate)]
pub struct OxygenMechanic {
  pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct PlayerMechanicModifier {
  pub jump: Option<JumpMechanic>,
  pub grenade: Option<GrenadeMechanic>,
  pub oxygen: Option<OxygenMechanic>,
}

impl ProvideAssets for PlayerMechanicModifier {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Objective {
  pub text: String,
}

impl ProvideAssets for Objective {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Schema, Duplicate)]
pub enum PlatformType {
  Toggle,
  LoopOnce,
  LoopForever,
  OnBossBeaten,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Platform {
  pub enabled: bool,

  pub translation: Vector3<f32>,
  pub one_way_duration: Seconds,

  pub platform_type: PlatformType,

  #[serde(skip)]
  pub current_timer: Seconds,

  #[serde(skip)]
  pub is_returning: bool,

  #[serde(skip)]
  pub has_activated_after_boss: bool,
}

impl ProvideAssets for Platform {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct PlatformTrigger {
  pub enable: bool,
  pub platforms: Vec<PrefabId>,
}

impl ProvideAssets for PlatformTrigger {}

// NOTE: This component should not exist.
// Creating a `Preloader` is an anti-pattern.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Preloader {
  pub models: Vec<ModelId>,
  pub audios: Vec<AudioId>,
  pub particles: Vec<ParticleId>,
  pub sprites: Vec<SpriteId>,
}

impl ProvideAssets for Preloader {
  fn provide_assets(&self, ids: &mut Vec<Uuid>) {
    for model in &self.models {
      ids.push(**model);
    }

    for audio in &self.audios {
      ids.push(**audio);
    }

    for particle in &self.particles {
      ids.push(**particle);
    }

    for sprite in &self.sprites {
      ids.push(**sprite);
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LevelLoader {
  pub level_name: String,
}

impl ProvideAssets for LevelLoader {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LevelLoaderTrigger {
  pub level_name: String,
}

impl ProvideAssets for LevelLoaderTrigger {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LevelStart {}

impl ProvideAssets for LevelStart {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LevelLoaderIgnore {}

impl ProvideAssets for LevelLoaderIgnore {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Muzzle {
  pub noise_prefab: PrefabId,
}

impl ProvideAssets for Muzzle {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct IntroTrigger {
  pub auto_player_destination: PrefabId,
}

impl ProvideAssets for IntroTrigger {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct IntroEnd {}

impl ProvideAssets for IntroEnd {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Checkpoint {
  #[serde(skip)]
  pub saved: bool,
  #[serde(skip)]
  pub store: bool,
}

impl ProvideAssets for Checkpoint {}

impl Checkpoint {
  pub fn new() -> Self {
    Self {
      saved: false,
      store: false,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Boss {}

impl ProvideAssets for Boss {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct BossSensor {
  #[serde(skip)]
  pub activate: bool,
}

impl ProvideAssets for BossSensor {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct BossCameraSensor {
  #[serde(skip)]
  pub activate: bool,
}

impl ProvideAssets for BossCameraSensor {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Spawner {
  pub entity_prefab: PrefabId,
  pub placer_entity_prefab: PrefabId,
  pub max_entities_to_spawn: i32,
  #[serde(skip)]
  pub current_entities_spawned: i32,
  #[serde(skip)]
  pub current_timer: Seconds,
  pub time_between_spawns: Seconds,
}

impl ProvideAssets for Spawner {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Barrier {
  pub current_barrier: f32,
  pub total_barrier: f32,
  pub proxies: Vec<PrefabId>,
}

impl ProvideAssets for Barrier {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct SaveState {}

impl ProvideAssets for SaveState {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct RestoreState {}

impl ProvideAssets for RestoreState {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct TutorialSensor {}

fn default_added_rotation() -> Vector3<f32> {
  return Vector3::new(0.2, 0.3, 0.1);
}

fn default_rotation_speed() -> Rps {
  return Rps::new(0.5);
}

impl ProvideAssets for TutorialSensor {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct RotationalMovement {
  #[serde(default = "default_added_rotation")]
  #[schema(default = "[0.2, 0.3, 0.1]")]
  pub added_rotation: Vector3<f32>,

  #[serde(default = "default_rotation_speed")]
  pub rotation_speed: Rps,
}

impl ProvideAssets for RotationalMovement {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Explosion {
  pub scale_multiplier: f32,
}

impl ProvideAssets for Explosion {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct AudioFade {
  pub fade_in: bool,
  pub fade_in_prefab: PrefabId,
  pub fade_time: Seconds,
  pub fade_volume: Decibels,
  pub fade_in_sensor_start_volume: Decibels,

  #[serde(skip)]
  pub fade_timer: Seconds,
  #[serde(skip)]
  pub fade_in_timer: Seconds,
  #[serde(skip)]
  pub fade_in_start_volume: Decibels,
  #[serde(skip)]
  pub activate_timer: bool,
  #[serde(skip)]
  pub fade_out_finished: bool,
  #[serde(skip)]
  pub activate_in_timer: bool,
}

impl ProvideAssets for AudioFade {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct FadeSensor {
  #[serde(skip)]
  pub activate: bool,
}

impl ProvideAssets for FadeSensor {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct AnimatedCamera {
  #[serde(skip)]
  pub is_active: bool,
}

impl ProvideAssets for AnimatedCamera {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Safezone {
  #[serde(skip)]
  pub is_occupied: bool,
}

impl ProvideAssets for Safezone {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Citizen {}

impl ProvideAssets for Citizen {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LoadingCameraTrack {
  pub enabled: bool,
  pub duration: Seconds,

  #[serde(skip)]
  pub current_timer: Seconds,
}

impl ProvideAssets for LoadingCameraTrack {}

#[allow(unused)]
pub struct RequestLevelLoad;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum LevelName {
  Loading,
  Jail,
  Moon,
}

impl ProvideAssets for LevelName {}

#[allow(unused)]
pub struct LevelState {
  pub current: LevelName,
  pub next: LevelName,
  pub interpolator: Option<Interpolator>,
}
impl LevelState {
  pub fn new(level: LevelName) -> Self {
    Self {
      current: level,
      next: level,
      interpolator: None,
    }
  }
}
