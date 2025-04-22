use crate::shared::components::{
  Tileset,
};
use engine::application::components::{
  ModelComponent,
  TextComponent,
};
use engine::{
  application::{
    components::{CameraComponent, InputComponent, PhysicsComponent, SelfComponent},
    physics3d::RigidBodyHandle,
    scene::{Collision, IdComponent, Scene, TransformComponent},
    input::InputsReader, 
  },
  resources::{node::Transform, particles::ParticleId},
  systems::{
    controller::AudioController, physics::PhysicsController,
    controller::ParticleController, Backpack, Initializable, Inventory, System,
  },
  utils::physics,
  utils::units::{Decibels, Degrees, Meters, Mps, Seconds},
  Entity,
  nalgebra::{Point3, Rotation3, Unit, Vector3},
};
use std::fmt::Display;
use std::f32::consts::PI;
use std::mem::variant_count;
use uuid::Uuid;

use kahuna::*;
use kahuna::square_grid::*;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct States(u32);

const ST_GRASS: u32 = 1 << 0;
const ST_DIRT: u32 = 1 << 1;
const ST_SAND: u32 = 1 << 2;
const ST_WATER: u32 = 1 << 3;
const ST_ALL: u32 = ST_GRASS | ST_DIRT | ST_SAND | ST_WATER;

impl Display for States {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
			States(ST_GRASS) => write!(f, "Tile::Grass"),
			States(ST_DIRT) => write!(f, "Tile::Dirt"),
			States(ST_SAND) => write!(f, "Tile::Sand"),
			States(ST_WATER) => write!(f, "Tile::Water"),
			_ => write!(f, "?")
		}
  }
}

impl State for States {
  fn entropy(&self) -> u32 {
    let States(x) = *self;
    x.count_ones() - 1
  }
}

type Grid = SquareGrid<States>;

struct Rule;

impl CollapseRule<States, Grid> for Rule {
	fn neighbor_offsets(&self) -> Box<[<Grid as Space<States>>::CoordinateDelta]> {
		vec![
			(0, -1),
			(-1, 0),
			(1, 0),
			(0, 1)
		].into_boxed_slice()
	}
	
	fn collapse(&self, cell: &mut States, neighbors: &[Option<States>]) {
		let States(x) = cell;
		
		for rule in &RULES[..] {
			if *x & rule.state != 0 {
				for i in 0 .. 4 {
					if let Some(States(neighbor)) = neighbors[i] {
						if neighbor & rule.allowed_neighbors[i] == 0 {
							*x &= !rule.state;
						}
					}
				}
			}
		}
	}
	
	fn observe(&self, cell: &mut States, _neighbors: &[Option<States>]) {
		let States(x) = cell;
		let mut bits = vec![];
		for i in 0 .. 4 {
			if *x & (1 << i) != 0 {
				bits.push(i);
			}
		}
		*x = 1 << bits[thread_rng().gen_range(0..bits.len())];
	}
}


struct StateRule {
	state: u32,
	allowed_neighbors: [u32; 4]
}

const RULES: &'static [StateRule] = &[
	StateRule {
		state: ST_GRASS,
    /*
		allowed_neighbors: [
			ST_WATER | ST_DIRT | ST_GRASS,
			ST_WATER | ST_SAND | ST_GRASS, ST_WATER | ST_SAND | ST_GRASS,
			ST_WATER | ST_DIRT | ST_GRASS,
		]
    */
		allowed_neighbors: [
			ST_DIRT,
			ST_SAND, ST_WATER | ST_SAND,
			ST_DIRT,
		]
	},
	StateRule {
		state: ST_SAND,
		allowed_neighbors: [
			ST_WATER,
			ST_SAND | ST_GRASS, ST_SAND | ST_GRASS,
			ST_WATER,
		]
	},
	StateRule {
		state: ST_DIRT,
		allowed_neighbors: [
			ST_DIRT | ST_GRASS,
			ST_WATER, ST_WATER,
			ST_DIRT | ST_GRASS,
		]
	},
	StateRule {
		state: ST_WATER,
		allowed_neighbors: [
			!ST_DIRT,
			!ST_SAND, !ST_SAND,
			!ST_DIRT,
		]
	}
];



pub struct TilesetSystem {
  physics: PhysicsController,
}

impl Initializable for TilesetSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();

    Self {
      physics,
    }
  }
}

impl TilesetSystem {
}

impl System for TilesetSystem {
  fn get_name(&self) -> &'static str {
    "TilesetSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let mut entities = vec![];

    for (entity, (transform, tileset)) in scene.query_mut::<(&TransformComponent, &Tileset)>() {
      entities.push((entity, transform.clone(), tileset.clone()));
    }

    for (entity, transform, tileset) in entities {
      let mut grid = Grid::new(
        tileset.width as isize,
        tileset.length as isize,
        |_, _| States(ST_GRASS),
      );

      let half_x = tileset.width / 2;
      let half_z = tileset.length / 2;
      for x in 0..tileset.width {
        for z in 0..tileset.length {
          let prefab = grid[(x as isize, z as isize)];
          log::info!("tile: {:?}", &prefab.to_string());
          scene.spawn_prefab_with(&prefab.to_string(), |prefab| {
            let mut transform = transform.clone();
            transform.translation.x -= half_x as f32;
            transform.translation.x += x as f32;
            transform.translation.z -= half_z as f32;
            transform.translation.z += z as f32;

            prefab.transform = transform.into();
          });
        }
      }
    }
    scene.clear_component::<Tileset>();
  }
}
