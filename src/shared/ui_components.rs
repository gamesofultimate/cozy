use engine::application::scene::ProvideAssets;
use engine::resources::sprite::SpriteId;
use engine::systems::Registry;
use engine::utils::units::Seconds;

use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};

pub struct UiComponents;

impl Registry for UiComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;
    StartInstructions::register();
    LoadingIndicator::register();
    InventoryDisplay::register();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct StartInstructions {}

impl ProvideAssets for StartInstructions {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct LoadingIndicator {}

impl ProvideAssets for LoadingIndicator {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct InventoryDisplay {}

impl ProvideAssets for InventoryDisplay {}
