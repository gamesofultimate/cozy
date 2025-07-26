use rocket::{get, post};

use crate::models::{
    AcceptInvitation, Access, GameSession, GetMoments, GetMoment, GetPlaySession, GetInvitation, GetOrCreateSession, Invite, Login, Signup,
    GetAchievements, GetRankings,
};
use rocket::serde::json::Json;

use admin::{
    AcceptInvitationError, Admin, Moment, PlaySession, Auth, CreateSessionError, GetUserError, Invitation,
    InvitationError, GetMomentsError, GetMomentError, GetPlaySessionError, InvitationsError, InviteError, LoginError, Role, SignupError,
    GetAchievementsError,
    Achievement,
    Ranking,
    PlayerRank,
    GetRankingError,
};

#[post("/invite", data = "<invite>")]
pub async fn invite(invite: Json<Invite>) -> Json<Result<Invitation, InviteError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .invite(invite.email.to_string(), invite.access_token, None, None)
        .await
    {
        Ok(invitation) => Json(Ok(invitation)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/accept-invitation", data = "<signup>")]
pub async fn accept_invitation(
    signup: Json<AcceptInvitation<'_>>,
) -> Json<Result<Access, AcceptInvitationError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .accept_invitation(
            signup.invitation_token,
            signup.username.to_string(),
            signup.password.to_string(),
            None,
        )
        .await
    {
        Ok(access_token) => Json(Ok(Access { access_token })),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-invitations", data = "<access>")]
pub async fn get_invitations(
    access: Option<Json<Access>>,
) -> Json<Result<Vec<Invitation>, InvitationsError>> {
    let access = match access {
        Some(access) => access,
        None => return Json(Err(InvitationsError::UserIsLoggedOut)),
    };
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin.get_invitations(access.access_token).await {
        Ok(invitations) => Json(Ok(invitations)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-invitation", data = "<invitation>")]
pub async fn get_invitation(
    invitation: Json<GetInvitation>,
) -> Json<Result<(Invitation, Auth), InvitationError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin.get_invitation(invitation.invitation_token).await {
        Ok(invitation) => Json(Ok(invitation)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/user", data = "<access>")]
pub async fn user(access: Option<Json<Access>>) -> Json<Result<Auth, GetUserError>> {
    let access = match access {
        Some(access) => access,
        None => return Json(Err(GetUserError::UserIsLoggedOut)),
    };
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin.get_user(access.access_token).await {
        Ok(user) => Json(Ok(user)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/login", data = "<login>")]
pub async fn login(login: Json<Login<'_>>) -> Json<Result<Access, LoginError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .login(login.username.to_string(), login.password.to_string())
        .await
    {
        Ok(access_token) => Json(Ok(Access { access_token })),
        Err(err) => Json(Err(err)),
    }
}

#[post("/signup", data = "<signup>")]
pub async fn signup(signup: Json<Signup<'_>>) -> Json<Result<Access, SignupError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    // TODO: Save the data for mad mimic as well
    // Send an email to tashiro

    match admin
        .signup(
            signup.username.to_string(),
            signup.password.to_string(),
            Role::User,
            Some(signup.email.to_string()),
            None,
        )
        .await
    {
        Ok(access) => Json(Ok(Access {
            access_token: access.access_token,
        })),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-session", data = "<session>")]
pub async fn get_session(
    session: Json<GetOrCreateSession<'_>>,
) -> Json<Result<GameSession, CreateSessionError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .get_or_create_session(session.unique_key.to_string(), session.branch.to_string())
        .await
    {
        Ok((session, version)) => Json(Ok(GameSession { session, version })),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-global-moments")]
pub async fn get_global_moments() -> Json<Result<Vec<Moment>, GetMomentsError>> {
  /*
  let mut moments = vec![
    Moment::default(),
    Moment::default(),
    Moment::default(),
  ];

  for moment in &mut moments {
    moment.video_url = String::from("https://headquarters-games-staging-staging.storage.googleapis.com/recordings/16b2af24-e938-4c16-9c18-e7f6afa12873/04153aba-e1df-43f2-ab3d-54abd73cf742/ebe55617-0c3f-44aa-851f-1d8909994a00.mkv");
  }

  Json(Ok(moments))
  */
  let api = dotenv::var("API_KEY").unwrap();
  let secret = dotenv::var("SECRET_KEY").unwrap();
  let address = dotenv::var("ADMIN_ADDRESS").unwrap();
  let mut admin = Admin::new(&address, &api, &secret).await;

  match admin
      .get_global_moments()
      .await
  {
      Ok(moments) => Json(Ok(moments)),
      Err(err) => Json(Err(err)),
  }
}

#[post("/get-moments", data = "<config>")]
pub async fn get_moments(
    config: Json<GetMoments>,
) -> Json<Result<Vec<Moment>, GetMomentsError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .get_moments(config.unique_id)
        .await
    {
        Ok(moments) => Json(Ok(moments)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-moment", data = "<config>")]
pub async fn get_moment(
    config: Json<GetMoment>,
) -> Json<Result<Moment, GetMomentError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .get_moment(config.moment_id)
        .await
    {
        Ok(moments) => Json(Ok(moments)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-play-session", data = "<config>")]
pub async fn get_play_session(
    config: Json<GetPlaySession>,
) -> Json<Result<PlaySession, GetPlaySessionError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .get_play_session(config.session_id)
        .await
    {
        Ok(play_sessions) => Json(Ok(play_sessions)),
        Err(err) => Json(Err(err)),
    }
}


#[post("/get-achievements", data = "<config>")]
pub async fn get_achievements(
    config: Json<GetAchievements>,
) -> Json<Result<Vec<Achievement>, GetAchievementsError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    /*
    use uuid::Uuid;
    use chrono::Utc;
    Json(Ok(vec![
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
      Achievement {
        id: Uuid::new_v4(),
        game_id: Uuid::new_v4(),
        shortname: String::from("test"),

        achieved_image_url: String::from("test"),
        unachieved_image_url: String::from("test"),
        title: String::from("Test"),
        description: String::from("Test test test test test"),

        created_at: Utc::now(),
        updated_at: Utc::now(),
      },
    ]))
    */
    match admin
        .get_achievements(config.unique_id)
        .await
    {
        Ok(achivements) => Json(Ok(achivements)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-ranking", data = "<config>")]
pub async fn get_ranking(
    config: Json<GetRankings>,
) -> Json<Result<(Ranking, Vec<PlayerRank>), GetRankingError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .get_ranking("kills", 0, 3, Some(config.unique_id))
        .await
    {
        Ok(rankings) => Json(Ok(rankings)),
        Err(err) => Json(Err(err)),
    }
}

#[post("/get-global-ranking")]
pub async fn get_global_ranking() -> Json<Result<(Ranking, Vec<PlayerRank>), GetRankingError>> {
    let api = dotenv::var("API_KEY").unwrap();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let address = dotenv::var("ADMIN_ADDRESS").unwrap();
    let mut admin = Admin::new(&address, &api, &secret).await;

    match admin
        .get_ranking("kills", 0, 3, None)
        .await
    {
        Ok(rankings) => Json(Ok(rankings)),
        Err(err) => Json(Err(err)),
    }
}

#[get("/")]
pub async fn index() -> &'static str {
    "ok"
}
