use crate::shared::components::Gun;
use crate::shared::components::GunMode;
use crate::shared::ui_components::UICurrentWeaponReticleBoundary;
use crate::shared::ui_components::UICurrentWeaponReticleCrosshair;
use crate::shared::ui_components::UIWeaponReticleConfigurator;
use engine::{
  application::{
    components::{SelfComponent, SpriteComponent},
    scene::Scene,
  },
  systems::{Backpack, Initializable, Inventory, System},
};

pub struct UIReticleSystem {}

impl Initializable for UIReticleSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl UIReticleSystem {
  fn get_configurator(&mut self, scene: &mut Scene) -> Option<UIWeaponReticleConfigurator> {
    for (_, configurator) in scene.query_mut::<&UIWeaponReticleConfigurator>() {
      return Some(configurator.clone());
    }

    None
  }

  fn display_reticle(&mut self, scene: &mut Scene) -> Option<()> {
    let configurator = self.get_configurator(scene)?;

    let mut player_gun = None;
    for (_, (gun, _)) in scene.query_mut::<(&Gun, &SelfComponent)>() {
      player_gun = Some(gun.clone());
    }

    let player_gun = player_gun?;

    for (_, (sprite, _)) in
      scene.query_mut::<(&mut SpriteComponent, &UICurrentWeaponReticleBoundary)>()
    {
      sprite.id = match player_gun.mode {
        GunMode::Single => configurator.single_boundary,
        GunMode::Burst => configurator.burst_boundary,
        GunMode::Blast => configurator.blast_boundary,
      };
    }

    for (_, (sprite, _)) in
      scene.query_mut::<(&mut SpriteComponent, &UICurrentWeaponReticleCrosshair)>()
    {
      sprite.id = match player_gun.mode {
        GunMode::Single => configurator.single_crosshair,
        GunMode::Burst => configurator.burst_crosshair,
        GunMode::Blast => configurator.blast_crosshair,
      };
    }

    Some(())
  }
}

impl System for UIReticleSystem {
  fn get_name(&self) -> &'static str {
    "UIReticleSystem"
  }

  fn run(&mut self, scene: &mut Scene, _: &mut Backpack) {
    self.display_reticle(scene);
  }
}
