/// The session module has functionality for setting and referencing a session's information
/// from with in an HTTP handler. 

use thiserror::Error;
use chrono::{DateTime, Local, Duration};
use actix_session::Session as ActixSession;


const SESSION_TOKEN: &str = "session";
const SESSION_EXIPRY: &str = "expires";
/// The default expiration time of a session, equal to 24 hours or 24*60 minutes. 
pub const SESSION_LENGTH_IN_MINUTES: i64 = 60*24;

/// Fuctionality for web sessions. 
/// 
/// Session stores a user identifying token. This token is set after a successful login and 
/// can be retrieved by subsequent API handlers. The user token can be used to verify if 
/// the given user has access to the requested functionality.
/// 
/// Session also stores and expiration time for the session, which defaults to 24 hours. 
/// This expiration time is restarted with `start()`, force expired with `expire()` and
/// validated agaisnt system time with `expire()`.
/// 
/// This package implements this traitfor the [`actix_session::Session`] struct.
pub trait Session {
    /// Sets a token field to equal to the argument string. Returns a
    /// [`SessionError::SerializationFailure`] if there is an error inserting the token into the 
    /// actix_session::Session's internal HashMap.
    fn set_token(&self, id: String) -> Result<(), SessionError>;

    /// Retrieves the value of the token field in the session as a String. Returns a
    /// [`SessionError::KeyNotFound`] error if the token field does not exist within the Session, 
    /// indicating a bad session. Returns a [`SessionError::SerializationFailure`] if there is an error 
    /// deserializing the token value.
    fn get_token(&self) -> Result<String, SessionError>;

    /// Sets the expiration of the session to the default setting, which is
    /// [`SESSION_LENGTH_IN_MINUTES`]. Returns a [`SessionError::SerializationFailure`] if there is
    /// an error inserting the token into the actix_session::Session's internal HashMap.
    fn start(&self) -> Result<(), SessionError>;

    /// Sets the expiration of the session to time now. This causes the token to fail
    /// any future [`Session::validate`] calls. Returns a [`SessionError::SerializationFailure`] if there is
    /// an error inserting the token into the actix_session::Session's internal HashMap.
    fn expire(&self) -> Result<(), SessionError>;

    /// Returns true if the expiration of the session is after time now; returns false
    /// otherwise. Returns a [`SessionError::KeyNotFound`] error if the expiration field does not 
    /// exist within the Session, indicating a bad session. Returns a 
    /// [`SessionError::SerializationFailure`] if there is an error deserializing the token value.
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

/// Errors associated with this package
#[derive(Error, Debug)]
pub enum SessionError {
    /// And empty error for testing only
    #[error("Test err")]
    Empty,

    /// Indicates a failure to serialize or deserialize. Derived from 
    /// [`serde_json::error::Error`].
    #[error("Serialization error: {}", .0)]
    SerializationFailure(#[from] serde_json::error::Error),  

    /// Indicates that the stored session is missing a required parameter. This should
    /// not happen in normal operation and indicatesthat the session has been corrupted. 
    #[error["Session missing parameter {}", .0]]
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

