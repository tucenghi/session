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
    #[error("Serialization error: {}", .0)]
    SerializationFailure(#[from] serde_json::error::Error),  

    #[error["Session cookie missing parameter {}", .0]]
    KeyNotFound(String),

}


