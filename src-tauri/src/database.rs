use mongodb::{
    sync::{
        Client, 
        Database,
        Collection,
    },
    bson::doc,
    error::Result,
    results::InsertOneResult,
};

use db_manager::User;

pub struct DatabaseManager {
    // client: Client,
    db: Database
}

impl DatabaseManager {
    pub fn connect() -> Result<Self> {
        let client = Client::with_uri_str("mongodb://localhost:27017")?;
        let db = client.database("maths-revision-tool");
        Ok(DatabaseManager { /*client,*/ db })
    }

    pub fn get_users(&self) -> Collection<User> {
        self.db.collection("user")
    }

    pub fn validate_login(&self, username: &str, password: &str) -> Result<Option<User>> {
        let query = doc! { "username": username };
        let opt_user = self.get_users().find_one(query.clone(), None)?;

        if let Some(ref user) = opt_user {
            if user.password() == password {
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

    pub fn add_user(&self, new_user: User) -> Result<InsertOneResult> {
        self.get_users().insert_one(new_user, None)
    }
}