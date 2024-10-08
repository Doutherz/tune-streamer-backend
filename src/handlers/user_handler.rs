use tide::http::Cookie;
use tide::{Error, Request, Response, Result, StatusCode};
use crate::services::user_service::{self, remove_token};
use crate::models::user_model::NewUser;

#[derive(serde::Deserialize)]
struct UserInput {
    username: String,
    password: String,
}

pub async fn add_user(mut req: Request<()>) -> Result<Response> {
    let user: UserInput = req.body_json().await?;

    if user_service::username_taken(&user.username).await? {
        return Err(Error::from_str(StatusCode::Conflict, "Username taken"));
    }

    let salt = user_service::gen_salt(10);
    let hashed_password = user_service::get_hash(&user.password, &salt)?;

    let user = NewUser {
        username: user.username,
        password: hashed_password,
        salt: salt,
    };

    user_service::add_user(user).await?;

    Ok(Response::builder(StatusCode::Created).build())
}

#[derive(serde::Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

pub async fn login(mut req: Request<()>) -> Result<Response> {
    let login_details: LoginInput = req.body_json().await?;

    if user_service::authenticate(&login_details.username, &login_details.password).await? {
        // generate a new token every login
        let token = user_service::gen_token(&login_details.username).await?;
        let mut res = Response::new(StatusCode::Ok);
        let mut cookie = Cookie::new("Session-Token", token);
        cookie.set_http_only(true);
        cookie.set_path("/");
            
        res.insert_cookie(cookie);
        return Ok(res);
    }

    Err(Error::from_str(StatusCode::Unauthorized, "Password incorrect"))
}

pub async fn logout(req: Request<()>) -> Result<Response> {
    let cookie = req.cookie("Session-Token");
    let mut cookie = match cookie {
        Some(cookie) => cookie,
        None => return Err(Error::from_str(StatusCode::Forbidden, "Not logged in")),
    };
    let mut res = Response::new(StatusCode::Ok);

    //remove session from database
    remove_token(&cookie.value()).await?;
    cookie.set_value("");
    cookie.set_path("/");
    cookie.set_http_only(true);
    
    res.insert_cookie(cookie);

    Ok(res)
}