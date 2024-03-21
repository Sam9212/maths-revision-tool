//! This is where all the logic for various facets of
//! the database system go, to make both the frontend
//! and also the command interface simpler in terms
//! of implementation. It handles various things, such
//! as:
//!
//! 1. Connecting to the database;
//! 2. Adding [`User`]s to the `users` collection; and
//! 3. Validating login details and adding user
//!    login strikes.

// Imports for the MongoDB database driver.
use mongodb::{
    bson::doc,
    sync::{Client, Collection, Database},
};

// Imports for the Database types in my shared library.
use shared::{
    questions::QuestionSet,
    requests::{
        UserReqError,
        UserReqErrorKind::{self, *},
    },
    responses::QuizReview,
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
    db: Database,
}

impl DatabaseManager {
    /// This function is a contructor for the DatabaseManager
    /// struct.
    pub fn connect() -> Result<Self, UserReqError> {
        // The database connects to the localhost server
        // map_err() lets me change the error into one that
        // is useful for me, made out of my own error type
        let client = Client::with_uri_str("mongodb://localhost:27017").map_err(|_| {
            UserReqError::new(
                ConnectionError,
                "could not initialize MongoDB connection".into(),
            )
        })?; // The question mark initiates a Try.
             // If the Result value is Ok, it stores that, if
             // it is Err, it returns that error early.

        let db = client.database("maths-revision-tool");

        // Inside an impl block, the Self type is an alias
        // for whatever the impl is being ran on, so in this
        // case Self = DatabaseManager
        Ok(Self { db })
    }

    pub fn get_users(&self) -> Collection<User> {
        // This is a getter for the collection in the database that
        // contains all the User structs.
        self.db.collection("user")
    }

    pub fn get_questions(&self) -> Collection<QuestionSet> {
        // This is a getter for the collection in the database that
        // contains all the QuestionSet structs.
        self.db.collection("question")
    }

    pub fn get_results(&self) -> Collection<QuizReview> {
        // This is a getter for the collection in the database that
        // contains all the QuizReview structs.
        self.db.collection("review")
    }

    pub fn validate_login(&self, username: String, password: String) -> Result<User, UserReqError> {
        // This is a query filter, it means that the select (aka find)
        // query only returns documents that have a username field
        // equal to the one provided by the function arguments above.
        let query = doc! { "username": username };
        let opt_user = self
            .get_users() // gets a handle to the collection
            .find_one(query.clone(), None) // finds the user with matching name
            .map_err(|_| UserReqError::new(ConnectionError, "Could not fetch user".into()))?;
        // ^ replaces any mongodb errors with a more generalised error that I made
        // so it can be transferred across the message passing tunnel in the code.

        // transforms Option into Result and Try's it immediately
        // to handle if the username was incorrect.
        let user = opt_user.ok_or(UserReqError::new(
            InvalidDetails,
            "The username or password was incorrect.".into(),
        ))?;

        // This case dictates that the user is locked out and thus
        // there is no point in checking their details further
        if user.strikes() >= 3 {
            Err(UserReqError::new(
                AccountLocked,
                "This account is locked out".into(),
            ))
        } else if user.password() == &password {
            // ^ this is the case where the username and password are both correct
            // An update query is made to set the strikes field to zero.
            let update = doc! {
                "$set" : doc! {
                    "strikes": 0
                }
            };

            // the update_one() call here takes the same previous query to get the right
            // user and also runs the update query, changing the value of the strikes
            // field on the database.
            self.get_users()
                .update_one(query, update, None)
                .expect("Strike Reset Failed.");
            Ok(user)
        } else {
            // strikes is increased by one with this update query
            let update = doc! {
                "$inc" : doc! {
                    "strikes": 1
                }
            };

            self.get_users()
                .update_one(query, update, None)
                .expect("Strike Add Failed.");

            // This error state indicates to the user of the function
            // that something went wrong, so that an error box can be displayed
            // to the user.
            Err(UserReqError::new(
                InvalidDetails,
                "The username or password was incorrect.".into(),
            ))
        }
    }

    /// This is a function used by the Registration page to
    /// add new [`User`] objects to the `users` collection.
    ///
    /// It is simple enough that it does not warrant a full
    /// walkthrough but it is useful to note that the
    /// function does do some mapping and manipulation to
    /// the actual result of the `insert_one` operation.
    ///
    /// The reason for this is simply because the [`InsertOneResult`]
    /// type provided by the MongoDB driver is not a suitable
    /// target for Serialization which means it cannot be
    /// transported to the frontend.
    pub fn add_user(&self, new_user: User) -> Result<(), UserReqError> {
        match self.get_users().insert_one(new_user, None) {
            Ok(_) => Ok(()), // We can discard the InsertOneResult
            Err(_) => Err(UserReqError::new(
                // Converting the error into my own error
                AddUserError,
                "Could not add user object to database".into(),
            )),
        }
    }

    pub fn unlock_user(&self, username: String) -> Result<(), UserReqError> {
        // Query filter
        let query = doc! { "username": username };
        let opt_user = self
            .get_users() // Gets user table handle
            .find_one(query.clone(), None) // Need to clone query to reuse later
            .map_err(|_| UserReqError::new(ConnectionError, "Could not fetch user".to_owned()))?;

        if opt_user.is_some() {
            // If the user was found
            let update = doc! {
                "$set": doc! {
                    "strikes": 0,
                }
            };

            self.get_users()
                .update_one(query, update, None)
                .map_err(|_| {
                    UserReqError::new(StrikeResetError, "strikes could not be reset".to_owned())
                })?;
        }

        Ok(())
    }

    pub fn delete_user(&self, username: String) -> Result<(), UserReqError> {
        // Query filter
        let query = doc! { "username": &username };
        let is_user = self
            .get_users() // Gets user table handle
            .find_one(query.clone(), None) // Need to clone query to reuse later
            .map_err(|_| UserReqError::new(ConnectionError, "db operation failed".to_owned()))?
            .is_some();

        if is_user {
            // The deletion query just takes a query filter and deletes
            // the first matching object.
            self.delete_redundant_reviews(username)?;
            match self.get_users().delete_one(query, None) {
                Ok(_) => Ok(()),
                Err(_) => Err(UserReqError::new(
                    DeleteUserError,
                    "the user could not be deleted".to_owned(),
                )),
            }
        } else {
            Err(UserReqError::new(
                UserReqErrorKind::InvalidDetails,
                "could not find user".to_owned(),
            ))
        }
    }

    pub fn add_question_set(&self, new_set: QuestionSet) -> Result<(), UserReqError> {
        match self.get_questions().insert_one(new_set, None) {
            Ok(_) => Ok(()), // We can discard the InsertOneResult
            Err(_) => Err(UserReqError::new(
                // Converting the error into my own error
                AddSetError,
                "could not add question set to database".into(),
            )),
        }
    }

    pub fn delete_question_set(&self, name: String) -> Result<(), UserReqError> {
        // Query filter
        let query = doc! { "name": name };
        let is_q = self
            .get_questions() // Gets question table handle
            .find_one(query.clone(), None) // Need to clone query to reuse later
            .map_err(|_| UserReqError::new(ConnectionError, "db operation failed".to_owned()))?
            .is_some();

        // The deletion query just takes a query filter and deletes
        // the first matching object
        if is_q {
            match self.get_questions().delete_one(query, None) {
                Ok(_) => Ok(()),
                Err(_) => Err(UserReqError::new(
                    DeleteQuestionsError,
                    "the question could not be deleted".to_owned(),
                )),
            }
        } else {
            Err(UserReqError::new(
                InvalidDetails,
                "could not find question set".to_owned(),
            ))
        }
    }

    pub fn get_question_set(&self, set_name: String) -> Result<QuestionSet, UserReqError> {
        let query = doc! { "name": set_name };
        let set = self
            .get_questions() // Gets question table handle
            .find_one(query.clone(), None) // Need to clone query to reuse later
            .map_err(|_| UserReqError::new(ConnectionError, "db operation failed".to_owned()))?;

        match set {
            Some(set) => Ok(set),
            None => Err(UserReqError::new(
                InvalidDetails,
                "could not find question set".to_owned(),
            )),
        }
    }

    pub fn get_question_sets(&self) -> Result<Vec<QuestionSet>, UserReqError> {
        match self.get_questions().find(None, None) {
            Ok(v) => Ok(v.filter_map(|v| v.ok()).collect()),
            Err(_) => Err(UserReqError::new(
                FetchQuestionsError,
                "could not fetch question sets".into(),
            )),
        }
    }

    pub fn get_quiz_reviews(&self) -> Result<Vec<QuizReview>, UserReqError> {
        match self.get_results().find(None, None) {
            Ok(v) => Ok(v.filter_map(|v| v.ok()).collect()),
            Err(_) => Err(UserReqError::new(
                FetchReviewsError,
                "could not fetch quiz reviews".into(),
            )),
        }
    }

    pub fn add_quiz_review(&self, new_review: QuizReview) -> Result<(), UserReqError> {
        match self.get_results().insert_one(new_review, None) {
            Ok(_) => Ok(()), // We can discard the InsertOneResult
            Err(_) => Err(UserReqError::new(
                // Converting the error into my own error
                AddReviewError,
                "could not add quiz review to database".into(),
            )),
        }
    }

    fn delete_redundant_reviews(&self, username: String) -> Result<(), UserReqError> {
        self.get_results()
            .delete_many(doc! { "username": username }, None)
            .map_err(|_| {
                UserReqError::new(
                    DeleteReviewError,
                    "could not delete quiz reviews".to_owned(),
                )
            })?;
        Ok(())
    }
}
