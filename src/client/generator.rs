use crate::shared::components::{Generator, GeneratorLight, Interactable, Interaction, Player};
use engine::{
  application::{
    components::{AudioSourceComponent, LightComponent, SourceState},
    scene::{Collision, Scene},
  },
  systems::{input::InputsReader, Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};

use crate::client::GameInput;

pub struct GeneratorSystem {
  inputs: InputsReader<GameInput>,
  first_run: bool,
}

impl Initializable for GeneratorSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();

    Self {
      inputs,
      first_run: true,
    }
  }
}

impl GeneratorSystem {
  fn prepare_lights(&mut self, scene: &mut Scene) {
    let query = scene.query_mut::<&Generator>();

    let mut generator_clone = None;
    for (_, generator) in query {
      generator_clone = Some(generator.clone());
    }

    let generator = match generator_clone {
      Some(generator) => generator,
      None => return,
    };

    self.first_run = false;

    let query = scene.query_mut::<(&mut GeneratorLight, &mut LightComponent)>();
    for (_, (generator_light, light)) in query {
      if let LightComponent::Point { intensity, .. } = light {
        generator_light.cached_intensity = *intensity;

        if generator.active != generator_light.inverted {
          *intensity = 0.0;
        }
      }

      if let LightComponent::Spot { intensity, .. } = light {
        generator_light.cached_intensity = *intensity;

        if generator.active != generator_light.inverted {
          *intensity = 0.0;
        }
      }
    }
  }

  fn handle_generator_collision(&mut self, scene: &mut Scene) {
    let input = self.inputs.read();

    for (_, (generator, interactable, _collision)) in scene.query_mut::<(
      &mut Generator,
      &mut Interactable,
      &Collision<Player, Interactable>,
    )>() {
      if interactable.ignore {
        continue;
      }

      let switch = match interactable.action {
        Interaction::Collision => true,
        Interaction::ActionKey => input.interact,
      };

      if !switch {
        continue;
      }

      interactable.has_activated = true;

      generator.active = !generator.active;
    }
  }

  fn handle_generator_lights(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = *backpack.get::<Seconds>().unwrap();
    let query = scene.query_mut::<&Generator>();

    let mut generator_clone = None;
    for (_, generator) in query {
      generator_clone = Some(generator.clone());
    }

    let generator = match generator_clone {
      Some(generator) => generator,
      None => return,
    };

    let query = scene.query_mut::<(
      &mut GeneratorLight,
      &mut LightComponent,
      Option<&mut AudioSourceComponent>,
    )>();
    for (_, (generator_light, light, maybe_audio_source)) in query {
      if let LightComponent::Point { intensity, .. } | LightComponent::Spot { intensity, .. } =
        light
      {
        let (delay, target_intensity) = match generator.active != generator_light.inverted {
          true => (
            generator_light.turn_on_delay,
            generator_light.cached_intensity,
          ),
          false => (generator_light.turn_off_delay, 0.0),
        };

        if generator_light.current_timer < delay {
          generator_light.current_timer += delta_time;
        } else {
          *intensity = target_intensity;

          if generator.active
            && let Some(audio_source) = maybe_audio_source
          {
            audio_source.state = SourceState::Playing;
          }
        }
      }
    }
  }
}

impl System for GeneratorSystem {
  fn get_name(&self) -> &'static str {
    "GeneratorSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if self.first_run {
      self.prepare_lights(scene);
      return;
    }

    self.handle_generator_collision(scene);
    self.handle_generator_lights(scene, backpack);
  }
}
