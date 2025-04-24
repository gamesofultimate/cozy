use std::f32::consts::PI;
use std::collections::HashMap;
use crate::shared::game_input::{GameInput, InputState};
use crate::shared::components::{
  TimeOfDay,
  Pickup,
  Action,
  Log,
  PickupSpace,
};
use engine::{
  application::{
    components::{TextComponent, LightComponent, NetworkedPlayerComponent, ParentComponent},
    scene::{Scene, Collision, IdComponent, TransformComponent},
  },
  systems::{
    Backpack, Initializable, Inventory, System,
  },
  utils::units::{Seconds, Framerate, Radians},
};

pub struct PickupsSystem {
}

impl Initializable for PickupsSystem {
  fn initialize(_: &Inventory) -> Self {

    Self { }
  }
}

impl PickupsSystem {
}

impl System for PickupsSystem {
  fn get_name(&self) -> &'static str {
    "PickupsSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    let mut sun_inclination = Radians::new(0.0);

    // TODO: Should be running 4 times the speed of normal time


    let mut spaces = HashMap::new();
    for (_, (id, network, _)) in scene.query_mut::<(&IdComponent, &NetworkedPlayerComponent, &PickupSpace)>() {
      spaces.insert(*network.connection_id, *id);
    }

    let mut insertions = vec![];
    //for (_, (log, _)) in scene.query_mut::<(&mut Log, &Collision<Action, Pickup>)>() {
    for (_, (id, input, network, collision)) in scene.query_mut::<(&IdComponent, &GameInput, &NetworkedPlayerComponent, &Collision<Action, Pickup>)>() {
      if input.check(InputState::Action) && let Some(id) = spaces.get(&network.connection_id) {
        //pickups.insert(network.connection_id, *id, collision.other);
        insertions.push((collision.other, *id));
      }
    }

    for (entity, parent_id) in insertions {
      scene.add_component(entity, ParentComponent::new(*parent_id));
      scene.add_component(entity, TransformComponent::default());
    }
  }
}
