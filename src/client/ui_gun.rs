use crate::shared::components::{Grenade, Gun, GunMode, Throwable};
use crate::shared::ui_components::{UIBlastAmmo, UIBurstAmmo, UIGrenade, UISingleAmmo};
// use crate::shared::ui_components::UICurrentWeaponStats;
// use crate::shared::ui_components::WeaponStat;
use engine::{
  application::{
    components::{SelfComponent, SpriteComponent, TextComponent},
    scene::{Scene, TransformComponent},
  },
  systems::{Backpack, Initializable, Inventory, System},
};

pub struct UIGunSystem {
  first_run: bool,
}

impl Initializable for UIGunSystem {
  fn initialize(_: &Inventory) -> Self {
    Self { first_run: true }
  }
}

impl UIGunSystem {
  fn cache_opacities(&mut self, scene: &mut Scene) {
    for (_, (text, ui_grenade)) in scene.query_mut::<(&TextComponent, &mut UIGrenade)>() {
      ui_grenade.cached_opacity = text.opacity;
      self.first_run = false;
    }
  }

  fn display_grenade(&mut self, scene: &mut Scene) {
    let mut player_throwable = None;
    for (_, (throwable, _)) in scene.query_mut::<(&Throwable, &SelfComponent)>() {
      player_throwable = Some(throwable.clone());
    }

    let player_throwable = match player_throwable {
      Some(player_throwable) => player_throwable,
      None => return,
    };

    let maybe_grenade_entity = match scene.get_prefab_with_id(player_throwable.item) {
      Some(prefab) if prefab.len() > 0 => prefab[0].0.clone(),
      _ => return,
    };

    let grenade = match maybe_grenade_entity.get::<Grenade>() {
      Some(grenade) => grenade.clone(),
      None => return,
    };

    for (_, (text, ui)) in scene.query_mut::<(&mut TextComponent, &UIGrenade)>() {
      text.opacity = ui.cached_opacity;
      if !player_throwable.active {
        text.opacity = 0.0;
        continue;
      }
    }

    for (_, (transform, sprite, ui)) in
      scene.query_mut::<(&mut TransformComponent, &mut SpriteComponent, &UIGrenade)>()
    {
      if !player_throwable.active {
        transform.scale.y = 0.0;
        continue;
      }

      let mut bar_size = *player_throwable.cooldown_timer / *player_throwable.grenade_cooldown;
      if bar_size >= 1.0 {
        bar_size = 1.0;
      }

      transform.scale.y = bar_size * 0.5;

      let grenade_is_flying = player_throwable.cooldown_timer < grenade.max_time_before_detonation;
      let grenade_can_be_teleported =
        grenade.has_teleport && player_throwable.cooldown_timer > grenade.minimum_time_for_teleport;

      if grenade_is_flying && grenade_can_be_teleported {
        if ui.is_active_teleport_bar {
          sprite.opacity = 1.0;
        } else {
          sprite.opacity = 0.0;
        }
      } else {
        if ui.is_active_teleport_bar {
          sprite.opacity = 0.0;
        } else {
          sprite.opacity = 1.0;
        }
      }
    }
  }

  fn display_ammo(&mut self, scene: &mut Scene) {
    let mut player_gun = None;
    for (_, (gun, _)) in scene.query_mut::<(&Gun, &SelfComponent)>() {
      player_gun = Some(gun.clone());
    }

    if let Some(player_gun) = player_gun {
      for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UISingleAmmo)>() {
        let player_ammo = player_gun.single_ammo;
        if player_gun.mode == GunMode::Single {
          text.max_width = 370.0;
          text.text = format!("SINGLE {:}", player_ammo);
        } else {
          text.max_width = 200.0;
          text.text = format!("{:}", player_ammo);
        }

        if player_gun.is_reloading {
          text.text = format!("RELOADING");
        }
      }
      for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UIBurstAmmo)>() {
        let player_ammo = player_gun.burst_ammo;
        if player_gun.mode == GunMode::Burst {
          text.max_width = 360.0;
          text.text = format!("BURST {:}", player_ammo);
        } else {
          text.max_width = 200.0;
          text.text = format!("{:}", player_ammo);
        }
      }
      for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UIBlastAmmo)>() {
        let player_ammo = player_gun.blast_ammo;
        if player_gun.mode == GunMode::Blast {
          text.max_width = 360.0;
          text.text = format!("BLAST {:}", player_ammo);
        } else {
          text.max_width = 200.0;
          text.text = format!("{:}", player_ammo);
        }
      }
      for (_, (sprite, ui)) in scene.query_mut::<(&mut SpriteComponent, &UISingleAmmo)>() {
        if ui.is_active_weapon {
          if player_gun.mode == GunMode::Single {
            sprite.opacity = 0.5;
          } else {
            sprite.opacity = 0.0;
          }
        } else {
          if player_gun.mode == GunMode::Single {
            sprite.opacity = 0.0;
          } else {
            sprite.opacity = 0.3;
          }
        }
      }
      for (_, (sprite, ui)) in scene.query_mut::<(&mut SpriteComponent, &UIBurstAmmo)>() {
        if ui.is_active_weapon {
          if player_gun.mode == GunMode::Burst {
            sprite.opacity = 0.5;
          } else {
            sprite.opacity = 0.0;
          }
        } else {
          if player_gun.mode == GunMode::Burst {
            sprite.opacity = 0.0;
          } else {
            sprite.opacity = 0.3;
          }
        }
      }
      for (_, (sprite, ui)) in scene.query_mut::<(&mut SpriteComponent, &UIBlastAmmo)>() {
        if ui.is_active_weapon {
          if player_gun.mode == GunMode::Blast {
            sprite.opacity = 0.5;
          } else {
            sprite.opacity = 0.0;
          }
        } else {
          if player_gun.mode == GunMode::Blast {
            sprite.opacity = 0.0;
          } else {
            sprite.opacity = 0.3;
          }
        }
      }

      // let gun_mode_ammo = match player_gun.mode {
      //   GunMode::Single => player_gun.single_ammo,
      //   GunMode::Burst => player_gun.burst_ammo,
      //   GunMode::Blast => player_gun.blast_ammo,
      // };

      // let gun_mode_name = match player_gun.mode {
      //   GunMode::Single => "SINGLE",
      //   GunMode::Burst => "BURST",
      //   GunMode::Blast => "BLAST",
      // };

      // for (_, (text, stats)) in scene.query_mut::<(&mut TextComponent, &UICurrentWeaponStats)>() {
      //   match stats.stat {
      //     WeaponStat::Name => {
      //       text.text = format!("{}", gun_mode_name);
      //     }
      //     WeaponStat::Ammo => {
      //       text.text = format!("{}", gun_mode_ammo);
      //     }
      //   }
      // }
    }
  }
}

impl System for UIGunSystem {
  fn get_name(&self) -> &'static str {
    "UIGunSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    if self.first_run {
      self.cache_opacities(scene);
    }

    self.display_ammo(scene);
    self.display_grenade(scene);
  }
}
