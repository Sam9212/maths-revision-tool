use serde::Serialize;
use shared::{
    questions::QuestionSet,
    requests::{UserReqError, UserReqErrorKind::SerdeError},
    User,
};
use tauri_sys::{tauri::invoke, Error};

pub fn map_command_error<T>(res: Result<T, Error>) -> Result<T, UserReqError> {
    res.map_err(|e| match e {
        Error::Command(why) => why.into(),
        Error::Serde(s) => UserReqError::new(SerdeError, s),
    })
}

pub async fn invoke_get_question_sets() -> Result<Vec<QuestionSet>, UserReqError> {
    map_command_error(invoke("get_question_sets", &()).await)
}

pub async fn invoke_validate_login(
    username: String,
    password: String,
) -> Result<User, UserReqError> {
    #[derive(Serialize)]
    struct Payload {
        username: String,
        password: String,
    }
    map_command_error(invoke::<Payload, _>("validate_login", &Payload { username, password }).await)
}

pub async fn invoke_add_user(new_user: User) -> Result<(), UserReqError> {
    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct Payload {
        newUser: User,
    }
    map_command_error(invoke::<Payload, _>("add_user", &Payload { newUser: new_user }).await)
}

pub async fn invoke_get_usernames() -> Result<Vec<User>, UserReqError> {
    map_command_error(invoke("all_users", &()).await)
}

pub async fn invoke_add_question_set(new_set: QuestionSet) -> Result<(), UserReqError> {
    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct Payload {
        newSet: QuestionSet,
    }
    map_command_error(invoke::<Payload, _>("add_question_set", &Payload { newSet: new_set }).await)
}

pub async fn invoke_delete_question_set(name: String) -> Result<(), UserReqError> {
    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct Payload {
        name: String,
    }
    map_command_error(invoke::<Payload, _>("delete_question_set", &Payload { name }).await)
}

pub async fn invoke_unlock_user(username: String) -> Result<User, UserReqError> {
    #[derive(Serialize)]
    struct Payload {
        username: String,
    }
    map_command_error(invoke::<Payload, _>("unlock_user", &Payload { username }).await)
}

pub async fn invoke_delete_user(username: String) -> Result<User, UserReqError> {
    #[derive(Serialize)]
    struct Payload {
        username: String,
    }
    map_command_error(invoke::<Payload, _>("delete_user", &Payload { username }).await)
}
