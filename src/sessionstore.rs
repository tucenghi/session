// Rewrite to implement actix_session::SessionStore

// use thiserror::Error;
// use actix_web::http::StatusCode;

// use super::session::Session;

// const SESSION_CACHE: &str = "session";


// #[derive(Debug, Clone)]
// pub struct Cache {
//     client: couch_rs::Client
// }


// impl Cache{
//     pub fn new(dbhost: &str, dbuser: &str, dbpass: &str) -> Cache {
//         println!("Initializing database with {}, {}, {}", dbhost, dbuser, dbpass);
//         let c = couch_rs::Client::new(dbhost, dbuser, dbpass).unwrap(); 
//         Cache{client: c}
//     }

//     pub async fn start_session(&self, mut session: Session) -> Result<Session, CacheError> {

//         let db = self.client.db(SESSION_CACHE).await?;

//         match db.upsert(&mut session).await {
//             Ok(response) => {
//                 session._id = response.id;
//                 session._rev = response.rev;
//                 Ok(session)
//             }
//             Err(e) => {
//                 Err(CacheError::ConnectionFailed(e))
//             }
//         }
//     }

//     pub async fn expire_session(&self, user_id: String) -> Result<Session, CacheError> {

//         let db = self.client.db(SESSION_CACHE).await?;

//         // Locate session if it exists
//         let get_result = db.get(&user_id).await;
//         let mut session: Session = match get_result {
//             Ok(session) => {session}
//             Err(e) => {
//                 match e.status {
//                     StatusCode::NOT_FOUND => { return Err(CacheError::DoesNotExist(e))}
//                     _ => { return Err(CacheError::ConnectionFailed(e))}
//                 }
//             }
//         };

//         session = session.expire();

//         // update expired session
//         let result = db.save(&mut session).await?;

//         session._id = result.id;
//         session._rev = result.rev;

//         Ok(session)
//     }

//     pub async fn get_session(&self, user_id: String) -> Result<Session, CacheError> {

//         let db = self.client.db(SESSION_CACHE).await?;

//         match db.get(&user_id).await {
//             Ok(session) => {Ok(session)}
//             Err(e) => {
//                 match e.status {
//                     StatusCode::NOT_FOUND => {Err(CacheError::DoesNotExist(e))}
//                     _ => {Err(CacheError::ConnectionFailed(e))}
//                 }
//             }
//         }
//     }
// }

// // Cache enumerates all possible errors returned from intereactions with CouchDB repository
// #[derive(Error, Debug)]
// pub enum CacheError {
//     /// Represents a generic connection error
//     #[error("CouchDB error: {}", .0)]
//     ConnectionFailed(#[from] couch_rs::error::CouchError),  

//     #[error("Unique identifier for this user does not exist")]
//     DoesNotExist(#[source] couch_rs::error::CouchError),

//     /// Represents failure to parse a json response from the database
//     #[error("Unable to parse database response")]
//     ParseFailure(#[from] serde_json::Error)
// }
