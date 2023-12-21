
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

use mongodb::{
    sync::{
        Client, 
        Database,
        Collection,
    },
    bson::{ 
        oid::ObjectId,
        doc,
    },
};

use db_manager::{
    requests::{
        UserReqErrorKind::*,
        UserReqError,
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
    /// The (effective) constructor for the [`DatabaseManager`]
    /// struct.
    /// 
    /// It initializes a connection to the MongoDB server and 
    /// retrieves a handle to the database that I want to connect
    /// to, conveniently named 'maths-revision-tool'. Then a struct
    /// of type `Self` is returned, which is an alias for the type
    /// mentioned at the start of the `impl` block.
    pub fn connect() -> Result<Self, UserReqError> {
        let client = Client::with_uri_str("mongodb://localhost:27017").map_err(|_| UserReqError::new(ConnectionError, "Could not initialize MongoDB connection!".into()))?;
        let db = client.database("maths-revision-tool");
        Ok(Self { /*client,*/ db })
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
        let query = doc! { "username": username };
        let opt_user = self.get_users().find_one(query.clone(), None).map_err(|_| UserReqError::new(ConnectionError, "Could not fetch user".into()))?;
        let user = opt_user.ok_or(UserReqError::new(InvalidDetails, "The username or password was incorrect.".into()))?;
        if user.strikes() >= 3 {
            Err(UserReqError::new(AccountLocked, "The attempts exceeded 3".into()))
        } else if user.password() == &password {
            let update = doc! {
                "$set" : doc! {
                    "strikes": 0
                }
            };

            self.get_users()
                .update_one(query, update, None)
                .expect("Strike Reset Failed.");
            Ok(user)
        } else {
            let update = doc! {
                "$inc" : doc! {
                    "strikes": 1
                }
            };

            self.get_users()
                .update_one(query, update, None)
                .expect("Strike Add Failed.");

            Err(UserReqError::new(InvalidDetails, "The username or password was incorrect.".into()))
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
        self.get_users().insert_one(new_user, None)
            .map(|_| ())
            .map_err(|_| UserReqError::new(AddUserError, "Could not add user object to database".into()))
    }
}