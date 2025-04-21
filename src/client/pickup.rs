use crate::shared::components::{Enemy, Gun, GunMode, Oxygen, Pickup, Player};
use engine::{
  application::{
    components::{InputComponent, ModelComponent},
    scene::{Collision, Scene},
  },
  systems::{Backpack, Initializable, Inventory, System},
};

pub struct PickupSystem {}

impl Initializable for PickupSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl PickupSystem {
  fn handle_player_pickups(&mut self, scene: &mut Scene) {
    let mut player_pickups = vec![];
    for (entity, (gun, _player, collision)) in
      scene.query_mut::<(&Gun, &Pickup, &Collision<Pickup, Player>)>()
    {
      player_pickups.push((entity, gun.clone(), collision.clone()));
    }

    for (entity, gun, collision) in player_pickups {
      scene.remove_collision::<Pickup, Player>(entity, collision.other);
      let _ = scene.remove_entity(entity);

      for (_, (player_gun, _)) in scene.query_mut::<(&mut Gun, &Player)>() {
        if gun.mode == GunMode::Single {
          player_gun.single_ammo += gun.single_ammo;
        } else if gun.mode == GunMode::Burst {
          player_gun.burst_ammo += gun.burst_ammo;
        } else if gun.mode == GunMode::Blast {
          player_gun.blast_ammo += gun.blast_ammo;
        }
      }

      for (_, (model, _)) in scene.query_mut::<(&mut ModelComponent, &InputComponent)>() {
        model.skip = false;
      }
    }

    let mut player_pickups = vec![];
    for (entity, (oxygen, _player, collision)) in
      scene.query_mut::<(&Oxygen, &Pickup, &Collision<Pickup, Player>)>()
    {
      player_pickups.push((entity, oxygen.clone(), collision.clone()));
    }

    for (entity, oxygen, collision) in player_pickups {
      scene.remove_collision::<Pickup, Player>(entity, collision.other);
      let _ = scene.remove_entity(entity);

      for (_, (player_oxygen, _)) in scene.query_mut::<(&mut Oxygen, &Player)>() {
        player_oxygen.current_oxygen += oxygen.current_oxygen;
        player_oxygen.current_oxygen = player_oxygen.current_oxygen.min(player_oxygen.total_oxygen);
      }
    }
  }

  fn handle_enemy_pickups(&mut self, scene: &mut Scene) {
    let mut enemy_pickups = vec![];
    for (entity, (gun, collision)) in scene.query_mut::<(&Gun, &Collision<Pickup, Enemy>)>() {
      enemy_pickups.push((entity, gun.clone(), collision.clone()));
    }

    for (entity, mut gun, collision) in enemy_pickups {
      scene.remove_collision::<Pickup, Enemy>(entity, collision.other);
      let _ = scene.remove_entity(entity);

      gun.single_ammo = 999;
      gun.burst_ammo = 999;
      gun.blast_ammo = 999;
      // gun.pulse_ammo = 999;

      scene.add_component(collision.other, gun.clone());
    }
  }
}

impl System for PickupSystem {
  fn get_name(&self) -> &'static str {
    "PickupSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    self.handle_player_pickups(scene);
    self.handle_enemy_pickups(scene);
  }
}
