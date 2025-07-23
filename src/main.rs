#![feature(async_closure, let_chains, variant_count)]

#[cfg(target_arch = "wasm32")]
use engine::systems::Scheduler;

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
mod client;
#[cfg(target_arch = "wasm32")]
use engine::application::bus::BrowserBus;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
mod server;

mod planners;
mod shared;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct Game {
  scheduler: Scheduler,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn init(
    id: String,
    assets_location: String,
    bus: BrowserBus,
    session_id: String,
    connection_id: String,
    unique_id: String,
    access_token: Option<String>,
    udp_url: String,
    tcp_url: String,
    gpu_tier: u32,
    recording_url: Option<String>,
  ) -> Self {
    let scheduler = client::main(
      id,
      assets_location,
      bus,
      session_id,
      connection_id,
      unique_id,
      access_token,
      udp_url,
      tcp_url,
      gpu_tier,
      recording_url,
    );

    Self { scheduler }
  }

  pub async fn start(self) {
    self.scheduler.run().await;
  }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn main() {
  server::main().await;
}
