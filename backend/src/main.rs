mod models;
mod routes;

use rocket::http::Method;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use dotenv::dotenv;

use crate::routes::{
  get_invitation,
  accept_invitation,
  get_invitations,
  invite,
  get_session,
  index,
  login,
  signup,
  user,
  get_global_moments,
  get_moments,
  get_moment,
  get_play_session,
  get_achievements,
  get_ranking,
  get_global_ranking,
};

#[launch]
fn rocket() -> _ {
  dotenv().ok();

  tracing_subscriber::FmtSubscriber::builder()
    .with_max_level(tracing::Level::DEBUG)
    .init();

  //env_logger::init();

  let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
      vec![Method::Get, Method::Post, Method::Patch]
        .into_iter()
        .map(From::from)
        .collect(),
    )
    .allow_credentials(true);

  rocket::build()
    .mount("/", routes![
      index,
      signup,
      login,
      user,
      get_session,
      get_invitations,
      get_invitation,
      invite,
      accept_invitation,
      get_global_moments,
      get_moments,
      get_moment,
      get_play_session,
      get_achievements,
      get_ranking,
      get_global_ranking,
    ])
    .attach(cors.to_cors().unwrap())
}
