//! Manages server-side sessions
//! 
//! This package extends [`actix_session`] to include functionality relating to 
//! unique user ids and session expiration. It also implements a new session 
//! store using CouchDB. 
//! 
//! TODO -> Define a session storage for CouchDB

/// Extended functionality for actix_session::Session
/// 
/// This crate is intended to import an alternative UI for the Session struct. The functions
/// in this package's Session trait can replace those used by [`actix_session::Session`].
/// 
/// The proposed Session tracks a unique user id in String format and an expiration. This session 
/// could be used with any Session storage method. The proposed lifecyle for a session is that the 
/// id is stored and an expiration time is set at login. Subsequent API calls using the same session
/// will pull the session id for use with whatever API call has been made, as well a refreshing the
/// expiration. 
pub mod session;


