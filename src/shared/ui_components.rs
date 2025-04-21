use engine::application::scene::ProvideAssets;
use engine::resources::sprite::SpriteId;
use engine::systems::Registry;
use engine::utils::units::Seconds;

use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};

pub struct UIComponents;

impl Registry for UIComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;
    UIHealth::register();
    UISingleAmmo::register();
    UIBurstAmmo::register();
    UIBlastAmmo::register();
    UIMissionTimer::register();
    UIOxygen::register();
    UIDialogText::register();
    UICurrentWeaponStats::register();
    UIGrenade::register();

    UIObjective::register();
    UIIntro::register();

    UICurrentWeaponReticleBoundary::register();
    UICurrentWeaponReticleCrosshair::register();
    UIWeaponReticleConfigurator::register();

    UIInteractable::register();
    UIRespawn::register();
    UITutorial::register();

    UIMegaPops::register();
    UIMegaPopsBarrier::register();
  }
}

fn default_use_colored_text() -> bool {
  false
}

fn default_target_opacity() -> f32 {
  0.3
}

fn default_max_scale_multiplier() -> f32 {
  2.0
}

fn default_show_below() -> f32 {
  0.5
}

#[derive(
  Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Registerable, Schema, Duplicate,
)]
pub enum UIType {
  Permanent,
  LowMarker,
  IncreasingBorder,
  DirectionalDamageRight,
  DirectionalDamageLeft,
}

fn default_ui_type() -> UIType {
  UIType::Permanent
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIHealth {
  #[serde(default = "default_use_colored_text")]
  #[schema(default = "false")]
  pub use_colored_text: bool,

  #[serde(default = "default_ui_type")]
  pub ui_type: UIType,

  #[serde(default = "default_target_opacity")]
  #[schema(default = "0.3")]
  pub target_opacity: f32,

  #[serde(default = "default_max_scale_multiplier")]
  #[schema(default = "2.0")]
  pub max_scale_multiplier: f32,

  #[serde(default = "default_show_below")]
  #[schema(default = "0.5")]
  pub show_below_health: f32,
}

impl ProvideAssets for UIHealth {}

fn default_is_active_weapon() -> bool {
  false
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UISingleAmmo {
  #[serde(default = "default_is_active_weapon")]
  #[schema(default = "false")]
  pub is_active_weapon: bool,
}

impl ProvideAssets for UISingleAmmo {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIBurstAmmo {
  #[serde(default = "default_is_active_weapon")]
  #[schema(default = "false")]
  pub is_active_weapon: bool,
}

impl ProvideAssets for UIBurstAmmo {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIBlastAmmo {
  #[serde(default = "default_is_active_weapon")]
  #[schema(default = "false")]
  pub is_active_weapon: bool,
}

impl ProvideAssets for UIBlastAmmo {}

fn default_is_active_teleport_bar() -> bool {
  false
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIGrenade {
  #[serde(default = "default_is_active_teleport_bar")]
  #[schema(default = "false")]
  pub is_active_teleport_bar: bool,

  #[serde(skip)]
  pub cached_opacity: f32,
}

impl ProvideAssets for UIGrenade {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIMissionTimer {}

impl ProvideAssets for UIMissionTimer {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIOxygen {
  #[serde(default = "default_use_colored_text")]
  #[schema(default = "false")]
  pub use_colored_text: bool,

  #[serde(default = "default_ui_type")]
  pub ui_type: UIType,

  #[serde(default = "default_target_opacity")]
  #[schema(default = "0.3")]
  pub target_opacity: f32,

  #[serde(default = "default_max_scale_multiplier")]
  #[schema(default = "2.0")]
  pub max_scale_multiplier: f32,

  #[serde(default = "default_show_below")]
  #[schema(default = "0.5")]
  pub show_below_oxygen: f32,
}

impl ProvideAssets for UIOxygen {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIDialogText {}

impl ProvideAssets for UIDialogText {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub enum WeaponStat {
  Name,
  Ammo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UICurrentWeaponStats {
  pub stat: WeaponStat,
}

impl ProvideAssets for UICurrentWeaponStats {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UICurrentWeaponReticleBoundary {}

impl ProvideAssets for UICurrentWeaponReticleBoundary {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UICurrentWeaponReticleCrosshair {}

impl ProvideAssets for UICurrentWeaponReticleCrosshair {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIWeaponReticleConfigurator {
  pub single_crosshair: SpriteId,
  pub single_boundary: SpriteId,
  pub burst_crosshair: SpriteId,
  pub burst_boundary: SpriteId,
  pub blast_crosshair: SpriteId,
  pub blast_boundary: SpriteId,
}

impl ProvideAssets for UIWeaponReticleConfigurator {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIInteractable {
  #[serde(skip)]
  pub cached_opacity: f32,
}

impl ProvideAssets for UIInteractable {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Schema, Duplicate)]
pub enum UIObjectiveType {
  Title,
  Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIObjective {
  pub section: UIObjectiveType,

  #[serde(skip)]
  pub cached_opacity: f32,
}

impl ProvideAssets for UIObjective {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIIntro {
  pub enabled: bool,
  pub ease_in_duration: Seconds,
  pub hold_duration: Seconds,
  pub ease_out_duration: Seconds,
  pub max_opacity: f32,

  #[serde(skip)]
  pub current_timer: Seconds,
}

impl ProvideAssets for UIIntro {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIRespawn {}

impl ProvideAssets for UIRespawn {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIMegaPops {
  #[schema(default = "0.5")]
  pub fade_in_timer: f32,

  #[schema(default = "1.0")]
  pub max_scale_multiplier: f32,

  #[schema(default = "false")]
  pub active: bool,
}

impl ProvideAssets for UIMegaPops {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIMegaPopsBarrier {
  #[schema(default = "0.5")]
  pub fade_in_timer: f32,

  #[schema(default = "1.0")]
  pub max_scale_multiplier: f32,

  #[schema(default = "false")]
  pub active: bool,
}

impl ProvideAssets for UIMegaPopsBarrier {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UITutorial {}

impl ProvideAssets for UITutorial {}
