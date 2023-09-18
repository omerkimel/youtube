use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    HttpRequest,
    Responder,
    http::{header::ContentType, StatusCode}
};

use rand::Rng;
use std::iter;
use regex::Regex;
use std::collections::HashSet;


use serde::{Serialize, Deserialize};
use derive_more::{Display};
//use std::fmt::{Display, Debug};

#[derive(Deserialize, Serialize)]
pub struct Token {
    token: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}


#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    BadUserRequest
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::FORBIDDEN,
            UserError::BadUserRequest => StatusCode::BAD_REQUEST
        }
    }
}

fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

fn checkIfEmailValid(email: String) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    email_regex.is_match(&email)
}

#[post("/login")]
pub async fn login(loginData: Json<LoginRequest>) -> Result<Json<Token>, UserError> {
    if loginData.password == "r2isthebest" && checkIfEmailValid(loginData.email.clone()) {
        let newToken = Token {
            token: generate(10)
        };
        // Tokens.insert(newToken.token);
        Ok(Json(newToken))
    }
    else {
        Err(UserError::UserNotFound)
    }
}

#[post("/try_luck")]
pub async fn try_luck(request: HttpRequest) -> Result<String, UserError> {
    if true {
        let req_headers = request.headers();
        let basic_auth_header = req_headers.get("Authorization");
        let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();
        println!("{}", basic_auth);
        Ok("somthing".to_string())
    }
    else {
        Err(UserError::UserNotFound)
    }
}

// #[post("/task")]
// pub async fn submit_task(
//     ddb_repo: Data<DDBRepository>,
//     request: Json<SubmitTaskRequest>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     let task = Task::new (
//         request.user_id.clone(),
//         request.task_type.clone(),
//         request.source_file.clone(),
//     );

//     let task_identifier = task.get_global_id();
//     match ddb_repo.put_task(task).await {
//         Ok(()) => Ok(Json(TaskIdentifier { task_global_id: task_identifier })),
//         Err(_) => Err(TaskError::TaskCreationFailure)
//     }
// }

// async fn state_transition(
//     ddb_repo: Data<DDBRepository>, 
//     task_global_id: String,
//     new_state: TaskState,
//     result_file: Option<String>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     let mut task = match ddb_repo.get_task(
//         task_global_id
//     ).await {
//         Some(task) => task,
//         None => return Err(TaskError::TaskNotFound)
//     };

//     if !task.can_transition_to(&new_state) {
//         return Err(TaskError::BadTaskRequest);
//     };
    
//     task.state = new_state;
//     task.result_file = result_file;

//     let task_identifier = task.get_global_id();
//     match ddb_repo.put_task(task).await {
//         Ok(()) => Ok(Json(TaskIdentifier { task_global_id: task_identifier })),
//         Err(_) => Err(TaskError::TaskUpdateFailure)
//     }
// }

// #[put("/task/{task_global_id}/start")]
// pub async fn start_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     state_transition(
//         ddb_repo, 
//         task_identifier.into_inner().task_global_id, 
//         TaskState::InProgress, 
//         None
//     ).await
// }

// #[put("/task/{task_global_id}/pause")]
// pub async fn pause_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     state_transition(
//         ddb_repo, 
//         task_identifier.into_inner().task_global_id, 
//         TaskState::Paused, 
//         None
//     ).await
// }

// #[put("/task/{task_global_id}/fail")]
// pub async fn fail_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     state_transition(
//         ddb_repo, 
//         task_identifier.into_inner().task_global_id, 
//         TaskState::Failed, 
//         None
//     ).await
// }

// #[put("/task/{task_global_id}/complete")]
// pub async fn complete_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>,
//     completion_request: Json<TaskCompletionRequest>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     state_transition(
//         ddb_repo, 
//         task_identifier.into_inner().task_global_id, 
//         TaskState::Completed, 
//         Some(completion_request.result_file.clone())
//     ).await
// }