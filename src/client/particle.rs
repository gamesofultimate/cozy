use crate::shared::components::ParticleEmitter;

use engine::{
  application::scene::{IdComponent, Scene, TransformComponent},
  systems::{rendering::ParticleController, Backpack, Initializable, Inventory, System},
};

use nalgebra::{Unit, Vector3};

pub struct ParticleSystem {
  particle_controller: ParticleController,
}

impl Initializable for ParticleSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let particle_controller = inventory.get::<ParticleController>().clone();

    Self {
      particle_controller,
    }
  }
}

impl ParticleSystem {
  fn handle_particles(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let query = scene.query_mut::<(&IdComponent, &TransformComponent, &mut ParticleEmitter)>();
    for (_, (id, transform, particle_emitter)) in query {
      if particle_emitter.has_activated {
        continue;
      }

      particle_emitter.has_activated = true;

      self.particle_controller.emit(
        particle_emitter.particle_id,
        ***id,
        transform.translation,
        Unit::new_normalize(Vector3::y()),
      );
    }
  }
}

impl System for ParticleSystem {
  fn get_name(&self) -> &'static str {
    "ParticleSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_particles(scene, backpack);
  }
}
