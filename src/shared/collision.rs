use engine::{
  application::{
    components::PhysicsComponent,
    physics3d::{ColliderHandle, CollisionEvent},
    scene::Scene,
  },
  systems::{
    physics::{CollisionsReader, PhysicsController},
    Backpack, Initializable, Inventory, System,
  },
  Entity,
  Query,
  QueryItem,
};

use crate::shared::components::{
  Ball,
  Goal,
  Floor,
  Player,
};


pub struct CollisionSystem {
  physics: PhysicsController,
  collisions_reader: CollisionsReader,
}

impl Initializable for CollisionSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    let collisions_reader = inventory.get::<CollisionsReader>().clone();

    Self {
      physics,
      collisions_reader,
    }
  }
}

impl System for CollisionSystem {
  fn get_name(&self) -> &'static str {
    "CollisionSystem"
  }

  fn attach(&mut self, _: &mut Scene, _: &mut Backpack) {}

  fn provide(&mut self, _inventory: &Inventory) {}

  fn run(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    let collisions = self.collisions_reader.read().collect::<Vec<_>>();

    for collision_event in collisions {
      match collision_event {
        CollisionEvent::Started(collider1, collider2, _) => {
          self.handle_collision_start::<Ball, Goal>(scene, collider1, collider2);
        }

        CollisionEvent::Stopped(collider1, collider2, _) => {
          self.handle_collision_stop::<Ball, Goal>(scene, collider1, collider2);
        }
      }
    }
  }
}

impl CollisionSystem {
  fn check_collision<'a, A, B>(
    e1: Entity,
    a1: &'a Option<A>,
    a2: &'a Option<A>,
    e2: Entity,
    b1: &'a Option<B>,
    b2: &'a Option<B>,
  ) -> Option<(Entity, &'a A, Entity, &'a B)> {
    if let (Some(a), Some(b)) = (&a1, &b2) {
      Some((e1, a, e2, b))
    } else if let (Some(a), Some(b)) = (&a2, &b1) {
      Some((e2, a, e1, b))
    } else {
      None
    }
  }

  fn get_entity_and_components<'a, Q: Query + 'a>(
    &self,
    scene: &'a mut Scene,
    collider: ColliderHandle,
  ) -> Option<(Entity, QueryItem<'a, Q>)> {
    let entity = self.physics.get_entity_from_collider_handle(collider)?;
    let result = scene.get_components_mut::<Q>(entity)?;

    Some((entity, result))
  }

  fn handle_collision_start<A, B>(
    &mut self,
    scene: &mut Scene,
    collider1: ColliderHandle,
    collider2: ColliderHandle,
  ) -> Option<()>
  where
    A: std::fmt::Debug + Clone + Send + Sync + 'static,
    B: std::fmt::Debug + Clone + Send + Sync + 'static,
  {
    let (entity1, (first1, second1)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>)>(scene, collider1) {
        Some((entity, (first, second))) => (entity, (first.cloned(), second.cloned())),
        None => return None,
      };

    let (entity2, (first2, second2)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>)>(scene, collider2) {
        Some((entity, (first, second))) => (entity, (first.cloned(), second.cloned())),
        None => return None,
      };

    if let Some((entity1, _, entity2, _)) =
      Self::check_collision(entity1, &first1, &first2, entity2, &second1, &second2)
    {
      scene.add_collision::<A, B>(entity1, entity2);
    }

    Some(())
  }

  fn handle_collision_stop<A, B>(
    &mut self,
    scene: &mut Scene,
    collider1: ColliderHandle,
    collider2: ColliderHandle,
  ) -> Option<()>
  where
    A: std::fmt::Debug + Clone + Send + Sync + 'static,
    B: std::fmt::Debug + Clone + Send + Sync + 'static,
  {
    let (entity1, (first1, second1)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>)>(scene, collider1) {
        Some((entity, (first, second))) => (entity, (first.cloned(), second.cloned())),
        None => return None,
      };

    let (entity2, (first2, second2)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>)>(scene, collider2) {
        Some((entity, (first, second))) => (entity, (first.cloned(), second.cloned())),
        None => return None,
      };

    if let Some(_) = Self::check_collision(entity1, &first1, &first2, entity2, &second1, &second2) {
      scene.remove_collision::<A, B>(entity1, entity2);
    }

    Some(())
  }
}
