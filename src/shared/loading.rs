use crate::shared::components::{
  Friend,
};
use crate::planners::social::{
  Friends,
  FriendLocation,
};
use engine::{
  application::{
    components::{TextComponent, LightComponent},
    scene::{Scene, IdComponent, TransformComponent},
  },
  systems::{
    trusty::AssetManager,
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Seconds, Framerate, Radians},
};
use std::f32::consts::PI;

pub struct LoadingSystem {
}

impl Initializable for LoadingSystem {
  fn initialize(_: &Inventory) -> Self {

    Self { }
  }
}

impl System for LoadingSystem {
  fn get_name(&self) -> &'static str {
    "LoadingSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let manager = match backpack.get::<AssetManager>() {
      Some(manager) => {
        manager.clone()
      }
      None => return,
    };

    //log::info!("manager: {:?}", &manager);
  }
}
