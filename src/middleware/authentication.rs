use std::{future::Future, pin::Pin};

use tide::{Error, Next, Request, Result, StatusCode};

use crate::services::user_service::get_session_user;

pub fn is_auth<'a>(req: Request<()>, next: Next<'a, ()>) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>>{
    Box::pin(async {
        let session = match req.cookie("Session-Token") {
            Some(session) => session,
            None => return Err(Error::from_str(StatusCode::Forbidden, "Not logged in cannot access content"))
        };
        //check if a user has that session
        let _user = get_session_user(session.value()).await?;

        

        Ok(next.run(req).await)
    })
}
