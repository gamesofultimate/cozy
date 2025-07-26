use admin::{Session, Version};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Invitations {
  pub access_token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetInvitation {
  pub invitation_token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AcceptInvitation<'r> {
  pub username: &'r str,
  pub password: &'r str,
  pub invitation_token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Invite {
  pub email: String,
  pub access_token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login<'r> {
  pub username: &'r str,
  pub password: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Signup<'r> {
  pub username: &'r str,
  pub password: &'r str,
  pub email: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameSession {
  pub session: Session,
  pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Access {
  pub access_token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetOrCreateSession<'r> {
    pub unique_key: &'r str,
    pub branch: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetMoments {
    pub unique_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetMoment {
    pub moment_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetPlaySession {
    pub session_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAchievements {
    pub unique_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetRankings {
    pub unique_id: Uuid,
}
