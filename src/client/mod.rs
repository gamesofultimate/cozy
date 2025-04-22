mod camera;

use crate::shared::{
  inputs,
  components,
  collision,
  //tileset,
  game_input::GameInput,
};
use engine::application::bus::BrowserBus;
use engine::systems::browser::BrowserActor;
use engine::systems::hdr::HdrMultiplayerPipeline;
use engine::systems::Scheduler;
use engine::utils::browser::grow_memory;

// 1080p
const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_HEIGHT: u32 = 1080;
const FRAMES_PER_SECOND: u64 = 60;
const GROW_MEMORY_IN_MB: u32 = 800;

pub fn main(
  canvas_id: String,
  assets_location: String,
  bus: BrowserBus,
  session_id: String,
  connection_id: String,
  unique_id: String,
  access_token: Option<String>,
  udp_url: String,
  tcp_url: String,
  recording_url: Option<String>,
) -> Scheduler {
  wasm_logger::init(wasm_logger::Config::default());
  grow_memory(GROW_MEMORY_IN_MB);
  let mut scheduler = Scheduler::new(FRAMES_PER_SECOND, [0, 0, 0, 255], canvas_id);

  log::debug!("assets location: {:?}", &assets_location);

  let hdr = HdrMultiplayerPipeline::<GameInput>::new(
    assets_location,
    session_id,
    connection_id,
    unique_id,
    access_token,
    udp_url,
    tcp_url,
    recording_url,
  );

  scheduler.attach_plugin(hdr);

  scheduler.attach_registry::<components::GameComponents>();
  scheduler.attach_system::<inputs::InputsSystem>();
  //scheduler.attach_system::<tileset::TilesetSystem>();
  scheduler.attach_system::<collision::CollisionSystem>();
  scheduler.attach_system::<camera::CameraSystem>();

  scheduler
}
