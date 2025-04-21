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
  }
}

