use engine::{
  application::{
    components::ParentComponent,
    physics3d::{ColliderHandle, CollisionEvent},
    scene::Scene,
  },
  systems::{
    physics::{CollisionsReader, PhysicsController},
    Backpack, Initializable, Inventory, System,
  },
  Entity, Query, QueryItem,
};
use std::collections::VecDeque;

use crate::shared::components::{Action, Pickup, Tile, WaterSource};

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
    let collisions = self
      .collisions_reader
      .read()
      .into_iter()
      .collect::<Vec<_>>();
    let mut count = 0;

    for collision_event in collisions {
      count += 1;
      match collision_event {
        CollisionEvent::Started(collider1, collider2, _) => {
          self.handle_collision_start::<Action, Pickup>(scene, collider1, collider2);
          self.handle_collision_start::<Action, Tile>(scene, collider1, collider2);
          self.handle_collision_start::<Action, WaterSource>(scene, collider1, collider2);
        }

        CollisionEvent::Stopped(collider1, collider2, _) => {
          self.handle_collision_stop::<Action, Pickup>(scene, collider1, collider2);
          self.handle_collision_stop::<Action, Tile>(scene, collider1, collider2);
          self.handle_collision_stop::<Action, WaterSource>(scene, collider1, collider2);
        }
      }
    }
    //log::info!("collisions: {count}");
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
    let (entity1, (first1, second1, maybe_parent_1)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>, Option<&ParentComponent>)>(
        scene, collider1,
      ) {
        Some((entity, (first, second, maybe_parent))) => (
          entity,
          (first.cloned(), second.cloned(), maybe_parent.cloned()),
        ),
        None => return None,
      };

    let (entity2, (first2, second2, maybe_parent_2)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>, Option<&ParentComponent>)>(
        scene, collider2,
      ) {
        Some((entity, (first, second, maybe_parent))) => (
          entity,
          (first.cloned(), second.cloned(), maybe_parent.cloned()),
        ),
        None => return None,
      };

    if let Some((entity1, _, entity2, _)) =
      Self::check_collision(entity1, &first1, &first2, entity2, &second1, &second2)
    {
      scene.add_collision::<A, B>(entity1, entity2);

      let mut queue = VecDeque::new();
      if let Some(parent) = maybe_parent_1 {
        queue.push_back(parent.parent_id);
      }

      if let Some(parent) = maybe_parent_2 {
        queue.push_back(parent.parent_id);
      }

      while let Some(parent_id) = queue.pop_front() {
        let entity = match scene.get_entity(parent_id) {
          Some(data) => data.clone(),
          None => continue,
        };
        scene.add_collision_to::<A, B>(entity, entity1, entity2);
        if let Some(Some(parent)) = scene.get_components_mut::<Option<&ParentComponent>>(entity) {
          queue.push_back(parent.parent_id);
        }
      }
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
    let (entity1, (first1, second1, maybe_parent_1)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>, Option<&ParentComponent>)>(
        scene, collider1,
      ) {
        Some((entity, (first, second, maybe_parent))) => (
          entity,
          (first.cloned(), second.cloned(), maybe_parent.cloned()),
        ),
        None => return None,
      };

    let (entity2, (first2, second2, maybe_parent_2)) =
      match self.get_entity_and_components::<(Option<&A>, Option<&B>, Option<&ParentComponent>)>(
        scene, collider2,
      ) {
        Some((entity, (first, second, maybe_parent))) => (
          entity,
          (first.cloned(), second.cloned(), maybe_parent.cloned()),
        ),
        None => return None,
      };

    if let Some(_) = Self::check_collision(entity1, &first1, &first2, entity2, &second1, &second2) {
      scene.remove_collision::<A, B>(entity1, entity2);

      let mut queue = VecDeque::new();
      if let Some(parent) = maybe_parent_1 {
        queue.push_back(parent.parent_id);
      }
      if let Some(parent) = maybe_parent_2 {
        queue.push_back(parent.parent_id);
      }

      while let Some(parent_id) = queue.pop_front() {
        let entity = match scene.get_entity(parent_id) {
          Some(data) => data.clone(),
          None => continue,
        };
        scene.remove_collision_to::<A, B>(entity);
        if let Some(Some(parent)) = scene.get_components_mut::<Option<&ParentComponent>>(entity) {
          queue.push_back(parent.parent_id);
        }
      }
    }

    Some(())
  }
}
