use async_trait::async_trait;
use engine::systems::network::InternalSender;
use engine::systems::Backpack;
use engine::utils::game::Level;
use engine::{
  application::{
    components::NetworkedPlayerComponent, downloader::DownloadSender, gamefile::Gamefile,
    input::TrustedInput, scene::Scene,
  },
  networking::connection::{ConnectionId, PlayerId, Protocol},
  systems::{
    network::{ChannelEvents, ClientSender},
    trusty::ServerControls,
    Initializable, Inventory,
  },
};
use std::collections::HashSet;

#[derive(Clone)]
pub struct PlayerConnections {
  pub connections: HashSet<ConnectionId>,
}

#[allow(dead_code)]
pub struct NetworkController {
  download_sender: DownloadSender,
  server: InternalSender<ServerControls>,
  client_sender: ClientSender<TrustedInput>,
  connections: PlayerConnections,
}

impl Initializable for NetworkController {
  fn initialize(inventory: &Inventory) -> Self {
    let download_sender = inventory.get::<DownloadSender>().clone();
    let client_sender = inventory.get::<ClientSender<TrustedInput>>().clone();
    let server = inventory.get::<InternalSender<ServerControls>>().clone();
    Self {
      client_sender,
      download_sender,
      server,
      connections: PlayerConnections {
        connections: HashSet::new(),
      },
    }
  }
}

#[async_trait]
impl ChannelEvents for NetworkController {
  fn on_session_start(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let level_path = match backpack.get::<Level>() {
      Some(level) => level.0.clone(),
      //None => "01_loading.lvl".to_string(),
      None => "10_main.lvl".to_string(),
    };

    // split path by '/resources/'
    let level_path = level_path.split("/resources/").collect::<Vec<&str>>();
    let level_path = level_path.get(1).unwrap_or(&level_path.get(0).unwrap());

    log::info!("Loading level: {:?}", level_path);
    let gamefile = Gamefile::from_file(&self.download_sender, &level_path);
    scene.load_level(&gamefile);
    scene.instance_entities(&gamefile);
  }

  fn on_player_joined(
    &mut self,
    scene: &mut Scene,
    backpack: &mut Backpack,
    connections: &HashSet<ConnectionId>,
    connection_id: ConnectionId,
    player: Option<(PlayerId, String)>,
    protocol: Protocol,
  ) {
    if protocol != Protocol::Tcp {
      return;
    }

    log::info!("PLAYER CONNECTED!!! {:?}", &connection_id);

    scene.spawn_prefab_and_children_with(
      "Player",
      |prefab| {
        if let Some((_, username)) = &player {
          prefab.tag.name = username.to_string();
        };

        prefab.push(NetworkedPlayerComponent::new(connection_id));
      },
      |prefab| {
        prefab.push(NetworkedPlayerComponent::new(connection_id));
      },
    );

    self
      .server
      .send(ServerControls::SyncWorld { connection_id });

    self.connections.connections = connections.clone();
    backpack.insert::<PlayerConnections>(self.connections.clone());
  }

  fn on_player_left(
    &mut self,
    _scene: &mut Scene,
    backpack: &mut Backpack,
    connection_id: ConnectionId,
    _protocol: Protocol,
  ) {
    self
      .server
      .send(ServerControls::DisconnectPlayer { connection_id });

    self.connections.connections.remove(&connection_id);
    backpack.insert::<PlayerConnections>(self.connections.clone());
  }
}
