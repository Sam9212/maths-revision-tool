use mongodb::{
    sync::{
        Client, 
        Database,
        Collection,
    },
    bson::doc,
    results::InsertOneResult,
};

use db_manager::{
    requests::{
        UserReqError,
        UserReqErrorKind::{
            ConnectionError,
            AddUserError
        },
    },
    User,
};

/// The struct that handles all the database operations.
/// 
/// This struct is what is responsible for connecting to
/// the Mongo database (which is currently served on my
/// own computer as opposed to an online server for testing
/// purposes) and is also responsible for running queries 
/// and fetching results which are then sent to the invoker
/// of the Tauri command responsible for each aspect of this
/// API. It makes use of a rich error system that I have 
/// implemented, however until Iteration 2 I have no plan to 
/// actually handle the errors that are passed all the way up
/// the control flow, and instead I will just crash the program 
/// when appropriate.
pub struct DatabaseManager {
    // client: Client,
    db: Database
}

impl DatabaseManager {
    /// This acts like a constructor for the database connection,
    /// initializing a connection to the server and retrieving a
    /// handle to the database of my choice, which has been named
    /// `maths-revision-tool`. It then returns a newly constructed 
    /// [`DatabaseManager`] 
    pub fn connect() -> Result<Self, UserReqError> {
        let client = Client::with_uri_str("mongodb://localhost:27017").map_err(|_| UserReqError::new(ConnectionError, "Could not initialize MongoDB connection!"))?;
        let db = client.database("maths-revision-tool");
        Ok(Self { /*client,*/ db })
    }

    pub fn get_users(&self) -> Collection<User> {
        self.db.collection("user")
    }

    pub fn validate_login(&self, username: &str, password: &str) -> Result<Option<User>, UserReqError> {
        let query = doc! { "username": username };
        let opt_user = self.get_users().find_one(query.clone(), None).map_err(|_| UserReqError::new(ConnectionError, "Could not fetch user"))?;

        if let Some(ref user) = opt_user {
            if user.strikes() > 2 {
                Ok(None)
            } else if user.password() == password {
                let update = doc! {
                    "$set" : doc! {
                        "strikes": 0
                    }
                };

                self.get_users()
                    .update_one(query, update, None)
                    .expect("Strike Reset Failed.");
               Ok(opt_user)
            } else {
                let update = doc! {
                    "$inc" : doc! {
                        "strikes": 1
                    }
                };

                self.get_users()
                    .update_one(query, update, None)
                    .expect("Strike Add Failed.");

                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn add_user(&self, new_user: User) -> Result<InsertOneResult, UserReqError> {
        self.get_users().insert_one(new_user, None).map_err(|_| UserReqError::new(AddUserError, "Could not add user object to database"))
    }
}