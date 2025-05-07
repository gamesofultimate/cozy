use crate::planners::social::{FriendLocation, Friends};
use crate::shared::components::Friend;
use engine::{
  application::{
    components::{LightComponent, TextComponent},
    scene::{IdComponent, Scene, TransformComponent},
  },
  systems::{trusty::AssetManager, Backpack, Initializable, Inventory, System},
  utils::units::{Framerate, Radians, Seconds},
};
use std::f32::consts::PI;

pub struct LoadingSystem {}

impl Initializable for LoadingSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl System for LoadingSystem {
  fn get_name(&self) -> &'static str {
    "LoadingSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let manager = match backpack.get::<AssetManager>() {
      Some(manager) => manager,
      None => return,
    };

    //log::info!("manager: {:?}", &manager);
  }
}
