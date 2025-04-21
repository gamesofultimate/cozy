use crate::client::game_input::GameInput;
use crate::shared::components::Player;
use crate::shared::components::{
  Explosion, Grenade, GrenadeNoise, Health, RotationalMovement, Throwable,
};
use engine::resources::node::Transform;
use engine::{
  application::{
    components::{
      AudioSourceComponent, CameraComponent, InputComponent, LightComponent, ParentComponent,
      PhysicsComponent, SelfComponent, SourceState,
    },
    scene::{Collision, IdComponent, PrefabId, Scene, TransformComponent},
  },
  systems::{
    input::InputsReader, physics::PhysicsController, Backpack, Initializable, Inventory, System,
  },
  utils::units::Seconds,
  Entity,
};

use nalgebra::{Isometry3, Vector3};
use rapier3d::prelude::{Ball, QueryFilter};

pub struct GrenadeSystem {
  inputs: InputsReader<GameInput>,
  physics: PhysicsController,
  timer: Seconds,
}

impl Initializable for GrenadeSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let physics = inventory.get::<PhysicsController>().clone();
    let timer = Seconds::new(0.0);
    Self {
      inputs,
      physics,
      timer,
    }
  }
}

impl GrenadeSystem {
  fn handle_input(
    &mut self,
    scene: &mut Scene,
    backpack: &mut Backpack,
    input: &GameInput,
  ) -> Option<(Entity, PrefabId, PhysicsComponent, Transform, Throwable)> {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    let mut grenade = None;

    for (entity, (_, id, transform, physics, thrower)) in scene.query_mut::<(
      &SelfComponent,
      &IdComponent,
      &TransformComponent,
      &PhysicsComponent,
      &mut Throwable,
    )>() {
      if !thrower.active {
        continue;
      }

      thrower.cooldown_timer += delta_time;
      if input.right_shoulder
        || input.right_click && thrower.cooldown_timer >= thrower.grenade_cooldown
      {
        thrower.current_charge += delta_time;

        if thrower.current_charge >= thrower.charge_time {
          thrower.current_charge = Seconds::zero();
          thrower.cooldown_timer = Seconds::zero();
          grenade = Some((
            entity,
            **id,
            physics.clone(),
            transform.get(),
            thrower.clone(),
          ));
        }
      } else {
        if thrower.current_charge >= thrower.charge_time {
          thrower.current_charge = Seconds::zero();
        }
      }
    }

    grenade
  }

  fn handle_throw(
    &mut self,
    scene: &mut Scene,
    _: Entity,
    player_id: PrefabId,
    _: PhysicsComponent,
    _: Transform,
    throwable: Throwable,
  ) -> Option<()> {
    let mut camera_transform = None;
    let mut camera_input = None;

    // Verify the prefab exists
    scene.get_prefab_with_id(throwable.item)?;

    for (_, (parent, transform, input, _camera)) in scene.query_mut::<(
      &ParentComponent,
      &TransformComponent,
      &InputComponent,
      &CameraComponent,
    )>() {
      if parent.parent_id == player_id.clone() {
        camera_transform = Some(transform);
        camera_input = Some(input);
        break;
      }
    }

    let camera_transform = camera_transform?;
    let camera_input = camera_input?;

    let input = camera_input.clone();
    let front = input.get_front().clone();
    let offset = Transform::with_translation(Vector3::new(0.25, 0.5, 1.5));

    let world_transform = camera_transform.world_transform() * offset;

    let entity = scene.spawn_prefab_id_with(throwable.item, |prefab| {
      prefab.transform = world_transform.into();
      prefab.transform.scale = Vector3::new(1.0, 1.0, 1.0) * throwable.scale_multiplier;
    });

    if let Some((entity, prefab)) = entity {
      if let Some(physics) = prefab.get::<PhysicsComponent>() {
        self
          .physics
          .insert_with_filter(entity, &physics, world_transform, vec![]);

        self
          .physics
          .set_linear_velocity(&physics, front.into_inner() * *throwable.velocity);

        self
          .physics
          .set_angular_damping(&physics, throwable.angular_dampening);

        if let Some(rotational) = prefab.get::<RotationalMovement>() {
          self.physics.set_angular_velocity(
            &physics,
            rotational.added_rotation * *rotational.rotation_speed,
          );
        }
      }
    }

    Some(())
  }

  fn handle_explosion(
    &mut self,
    scene: &mut Scene,
    backpack: &mut Backpack,
    game_input: &GameInput,
  ) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();
    let mut _activate_grenade_noise = false;
    let mut removals = vec![];

    for (_, (_, _, _)) in scene.query_mut::<(
      &Collision<Grenade, PhysicsComponent>,
      &Grenade,
      &mut AudioSourceComponent,
    )>() {
      _activate_grenade_noise = true;
    }

    for (_, (_, audio)) in scene.query_mut::<(&GrenadeNoise, &mut AudioSourceComponent)>() {
      if _activate_grenade_noise {
        audio.state = SourceState::Playing;
      } else {
        audio.state = SourceState::Stopped;
      }
    }

    for (entity, (transform, grenade, light)) in
      scene.query_mut::<(&TransformComponent, &mut Grenade, &mut LightComponent)>()
    {
      if grenade.current_time < grenade.max_time_before_detonation {
        grenade.current_time += delta_time;
        if let LightComponent::Point {
          intensity,
          falloff,
          radius,
          ..
        } = light
        {
          *intensity = 4.0 * *grenade.current_time;
          *falloff = 0.0;
          *radius = 1.0 * *grenade.current_time;
        }
      } else {
        removals.push((entity, transform.clone(), grenade.clone()));
      }
      if grenade.has_teleport
        && grenade.current_time > grenade.minimum_time_for_teleport
        && (game_input.right_click || game_input.right_shoulder)
      {
        grenade.is_teleport_ready = true;
        removals.push((entity, transform.clone(), grenade.clone()));
      }
    }

    for (entity, transform, grenade) in removals {
      if grenade.has_teleport && grenade.is_teleport_ready {
        for (_, (_, physics_comp)) in scene.query_mut::<(&mut Player, &PhysicsComponent)>() {
          let teleport_translation = Vector3::new(
            transform.translation.x,
            transform.translation.y + 0.5,
            transform.translation.z,
          );
          self
            .physics
            .set_translation(&physics_comp, teleport_translation);
        }

        let _ = scene.remove_entity(entity);
      } else {
        scene.spawn_prefab_id_with(grenade.noise, |prefab| {
          prefab.transform.translation = transform.translation;
        });

        let _ = scene.remove_entity(entity);

        let shape_pos = Isometry3::new(transform.translation, Vector3::new(0.0, 0.0, 0.0));
        let shape = Ball::new(*grenade.explosion_radius);
        let filter = QueryFilter::only_kinematic();

        let entities_exploded = self
          .physics
          .intersections_with_shape(&shape_pos, &shape, filter);

        for (entity, _) in entities_exploded {
          if let Some((entity_transform, entity_health)) =
            scene.get_components_mut::<(&TransformComponent, &mut Health)>(entity)
          {
            let distance_to_grenade =
              (transform.translation - entity_transform.translation).magnitude();

            let damage_factor = 1.0 - distance_to_grenade / *grenade.explosion_radius;
            let damage_factor = damage_factor.max(0.0);

            entity_health.current_health -= grenade.damage * damage_factor;
          }
        }

        scene.spawn_prefab_id_with(grenade.explosion_prefab, |prefab| {
          prefab.transform.translation = transform.translation;
        });
        self.timer = Seconds::new(0.0);
      }
    }
    for (_, (explosion, transform)) in
      scene.query_mut::<(&mut Explosion, &mut TransformComponent)>()
    {
      self.timer += delta_time;
      transform.scale *= explosion.scale_multiplier - *self.timer;
    }
    _activate_grenade_noise = false;
  }
}

impl System for GrenadeSystem {
  fn get_name(&self) -> &'static str {
    "GrenadeSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let input = self.inputs.read();
    if let Some((entity, player_id, physics, transform, thrower)) =
      self.handle_input(scene, backpack, &input)
    {
      self.handle_throw(scene, entity, player_id, physics, transform, thrower);
    }

    self.handle_explosion(scene, backpack, &input);
  }
}
