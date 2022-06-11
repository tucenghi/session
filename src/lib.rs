// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// Eventually, put this back into  session
//pub mod session;

use thiserror::Error;
use chrono::{DateTime, Local, Duration};
use actix_session::Session as ActixSession;


const SESSION_TOKEN: &str = "session";
const SESSION_EXIPRY: &str = "expires";
const SESSION_LENGTH_IN_MINUTES: i64 = 60*24;


pub trait Session {
    fn set_token(&self, id: String) -> Result<(), SessionError>;

    fn get_token(&self) -> Result<String, SessionError>;

    fn start(&self) -> Result<(), SessionError>;

    fn expire(&self) -> Result<(), SessionError>;

    fn validate(&self) -> Result<bool, SessionError>;
}

impl Session for ActixSession {
    fn set_token(&self, id: String) -> Result<(), SessionError> {
        self.insert(SESSION_TOKEN, id)?;
        Ok(())
    }

    fn get_token(&self) -> Result<String, SessionError> {
        match self.get::<String>(SESSION_TOKEN)? {
            Some(token) => Ok(token),
            None => Err(SessionError::KeyNotFound(SESSION_TOKEN.to_string()))
        }
    }

    fn start(&self) -> Result<(), SessionError> {
        let now = Local::now();
        self.insert(SESSION_EXIPRY, now + Duration::minutes(SESSION_LENGTH_IN_MINUTES))?;
        Ok(())
        
    }

    fn expire(&self) -> Result<(), SessionError> {
        let now = Local::now();  
        self.insert(SESSION_EXIPRY, now)?;
        Ok(())
    }

    fn validate(&self) -> Result<bool, SessionError> {

        match self.get::<DateTime<Local>>(SESSION_EXIPRY)? {
            Some(exp) => Ok(exp > Local::now()),
            None => Err(SessionError::KeyNotFound(SESSION_EXIPRY.to_string())),
        }
        

    }
}

// RepositoryError enumerates all possible session errors
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Test err")]
    Empty,

    #[error("Serialization error: {}", .0)]
    SerializationFailure(#[from] serde_json::error::Error),  

    #[error["Session cookie missing parameter {}", .0]]
    KeyNotFound(String),

}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test as actixtest;
    use actix_session::SessionExt;

    #[test]
    fn session_set_token() -> Result<(), SessionError> {

        let empty_session = actixtest::TestRequest::default().to_http_request().get_session(); 
        let set_session = actixtest::TestRequest::default().to_http_request().get_session(); 
        set_session.insert(SESSION_TOKEN, "some value")?;
        let id = "12345679";

        let tests: &[(
            &str, 
            ActixSession, 
            String, 
            Option<String>, 
            SessionError
        )] = &[
            (
                "success-empty-session", 
                empty_session, 
                String::from(id), 
                Some(String::from(id)), 
                SessionError::Empty
            ),
            (
                "success-preset-session", 
                set_session, 
                String::from(id), 
                Some(String::from(id)), 
                SessionError::Empty
            ),
        ];

        for (name, arg_session, arg_id, exp_token, exp_err) in tests {
            match arg_session.set_token(arg_id.to_string()) {
                Ok(_) => {
                    assert_eq!(arg_session.get::<String>(SESSION_TOKEN)?, *exp_token, "{}", name)
                }
                Err(got_err) => {
                    // assert types of enum are the same? does not compile and may not be required
                    //assert!(matches!(got_err, exp_err), "{}", name); 
                    assert_eq!(got_err.to_string(), exp_err.to_string())
                }
            }
        }
        Ok(())
    }
}
