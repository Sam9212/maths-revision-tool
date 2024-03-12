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
use db_manager::{
    requests::{UserReqError, UserReqErrorKind::*},
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

    /// This is a getter for the collection in the database that
    /// contains all the User structs.
    pub fn get_users(&self) -> Collection<User> {
        self.db.collection("user")
    }

    /// This function is what retrieves a [`User`] object from
    /// the database after checking if provided login details
    /// are valid.
    ///
    /// It is one of the more complex pieces of code that interacts
    /// with the DB. It does a query to find a user with appropriate
    /// username, and then does the checks to make sure the account
    /// is unlocked and has a correct password provided for login.
    /// When an incorrect password is provided, a strike is added
    /// through an update query. If it is correct, then the strike
    /// count is reset.
    ///
    /// # Walkthrough
    /// The first line here may look a bit strange to someone
    /// unfamiliar with how NoSQL databases function.
    /// ```
    /// let query = doc! { "username": username };
    /// ```
    /// The solution I chose for databasing is MongoDB. It follows
    /// a Document Database Structure, as opposed to a Relational
    /// Tabular Structure. The reason for this is due to how well
    /// a Document database can meld with Rust's algebraic type
    /// system. A document database works by nesting 'documents'
    /// which act almost identically to JSON 'objects' and this
    /// provides me with the ability to instantly `Serialize` and
    /// `Deserialize` objects due to the power of static typing.
    /// I never have to worry about malformed documents because I
    /// can only insert documents into the database if they
    /// conform to the data structure I created with the [`User`]
    /// struct.
    ///
    /// The query itself is actually just acting as a filter for
    /// the `find_one` operation that I am running. Only documents
    /// that have a username field matching the input string will
    /// be returned, and because I validate all usernames to make
    /// sure they are unique, it will only ever return one User.
    ///
    /// The next line here achieves a lot. Lets break it down.
    /// ```
    /// let opt_user = self.get_users().find_one(query.clone(), None).map_err(|_| UserReqError::new(ConnectionError, "Could not fetch user"))?;
    /// ```
    /// The first part is a call to my internal API. It simply
    /// gets a handle to the `users` collection inside the Mongo
    /// DB. The next part is running a `find_one` operation using
    /// my query value and a [`None`] value which indicates that
    /// there are no special options to run with the `find_one`
    /// operation. Then I use a transformation function to convert
    /// the error type to my own error type which implements
    /// `Serialize` and `Deserialize` so the backend can send it to
    /// frontend for user feedback later. Then we finally use the
    /// little `?` operator which is a very powerful error propogation
    /// tool. How it works is it looks at a [`Result<T, E>`] type and
    /// it converts it into `T` if the value was a [`Ok(T)`]. If,
    /// however, it was an [`Err(E)`], then the function returns
    /// immediately with that [`Err(E)`] value.
    ///
    /// This next line is another line that achieves a similar purpose
    /// in error propogation.
    /// ```
    /// let user = opt_user.ok_or(UserReqError::new(InvalidDetails, "The username or password was incorrect."))?;
    /// ```
    /// Our rich error type encompasses states for invalid login
    /// details, which is why we want to use `ok_or` to convert
    /// from an [`Option<User>`] to a [`Result<User, UserReqError>`].
    /// We do this, and then we use the `?` operator again to
    /// give us the [`Ok(User)`] value out.
    ///
    /// After all this error handling, the logic begins to
    /// simplify down. It becomes a simple if-elseif-else chain
    /// with each branch returning a different result state at
    /// the end. The first is for if the account has too many
    /// locked strikes, in which case there is no point in
    /// checking if the password is correct, and thus the error
    /// state is returned immediately. Then we handle the case
    /// where the password is correct, returning the ok value
    /// with the user held inside. Finally, we have the case
    /// where the password isn't correct. Here we return an
    /// error state as well. The message for each is to be
    /// directly consumed by the frontend to display in a modal
    /// box.
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
                    UserReqError::new(StrikeResetError, "Strikes couldn't be reset!".to_owned())
                })?;
        }

        Ok(())
    }

    pub fn delete_user(&self, username: String) -> Result<(), UserReqError> {
        // Query filter
        let query = doc! { "username": username };
        let opt_user = self
            .get_users() // Gets user table handle
            .find_one(query.clone(), None) // Need to clone query to reuse later
            .map_err(|_| UserReqError::new(ConnectionError, "Could not fetch user".to_owned()))?;

        if opt_user.is_some() {
            // The deletion query just takes a query filter and deletes
            // the first matching object.
            self.get_users().delete_one(query, None).map_err(|_| {
                UserReqError::new(DeleteUserError, "The user couldn't be deleted!".to_owned())
            })?;
        }

        Ok(())
    }
}
