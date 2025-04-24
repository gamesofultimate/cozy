mod network_controller;

use crate::planners::idling;
use crate::planners::life;
use crate::planners::social;
use crate::shared::inputs;
use crate::shared::collision;
use crate::shared::timeofday;
use crate::shared::pickups;
use crate::shared::components;
//use crate::shared::tileset;
use crate::shared::game_input::GameInput;
use crate::server::network_controller::NetworkController;
use engine::systems::hdr::HdrPipeline;
use engine::systems::Scheduler;

const FRAMES_PER_SECOND: u64 = 60;

pub async fn main() {
  dotenv::dotenv().ok();
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  log::info!("Number of cpus: {:}", num_cpus::get());

  let rpc_address = {
    let address = dotenv::var("RPC_ADDRESS").unwrap();
    let port = dotenv::var("RPC_PORT").unwrap();

    format!("{}:{}", address, port).parse().unwrap()
  };

  let session_address = {
    let address = dotenv::var("GAME_ADDRESS").unwrap();
    let port = dotenv::var("GAME_PORT").unwrap();

    format!("{}:{}", address, port).parse().unwrap()
  };

  let (hdr, _) = HdrPipeline::<NetworkController, GameInput>::new("resources", rpc_address, session_address);
  let mut scheduler = Scheduler::new(FRAMES_PER_SECOND);
  scheduler.attach_plugin(hdr);
  scheduler.attach_registry::<components::GameComponents>();
  scheduler.attach_registry::<idling::IdleRegistry>();
  scheduler.attach_registry::<life::LifeRegistry>();
  scheduler.attach_registry::<social::SocialRegistry>();
  scheduler.attach_system::<inputs::InputsSystem>();
  scheduler.attach_system::<timeofday::TimeOfDaySystem>();
  scheduler.attach_system::<pickups::PickupsSystem>();
  //scheduler.attach_system::<tileset::TilesetSystem>();
  scheduler.attach_system::<collision::CollisionSystem>();

  scheduler.run().await;
}
