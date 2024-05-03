use std::env;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::Row;

use crate::{
    usecase::{
        create_circle::{CreateCircleInput, CreateCircleOutput, CreateCircleUsecase},
        fetch_circle::{FetchCircleInput, FetchCircleOutput, FetchCircleUsecase, MemberOutput},
        update_circle::{UpdateCircleInput, UpdateCircleOutPut, UpdateCircleUsecase},
    },
    AppState,
};

pub async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub async fn handle_get_test(State(state): State<AppState>) -> impl IntoResponse {
    println!("Fetching Circles");
    let circle_rows = match sqlx::query("SELECT * FROM circles")
        .fetch_all(&state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to fetch circles: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    for row in circle_rows {
        let id = row.get::<i64, _>("id");
        let name = row.get::<String, _>("name");
        let capacity = row.get::<i64, _>("capacity");
        let owner_id = row.get::<i64, _>("owner_id");
        println!("id: {:?}", id);
        println!("circle: {:?}", name);
        println!("capacity: {:?}", capacity);
        println!("owner_id: {:?}", owner_id);
    }

    let member_rows = match sqlx::query("SELECT * FROM members")
        .fetch_all(&state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to fetch members: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    for row in member_rows {
        let name = row.get::<String, _>("name");
        println!("member: {:?}", name);
    }

    (StatusCode::OK).into_response()
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleRequestBody {
    pub circle_name: String,
    pub capacity: i32,
    pub owner_name: String,
    pub owner_age: i32,
    pub owner_grade: i32,
    pub owner_major: String,
}

impl std::convert::From<CreateCircleRequestBody> for CreateCircleInput {
    fn from(
        CreateCircleRequestBody {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }: CreateCircleRequestBody,
    ) -> Self {
        CreateCircleInput::new(
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        )
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleResponseBody {
    pub circle_id: i32,
    pub owner_id: i32,
}

impl std::convert::From<CreateCircleOutput> for CreateCircleResponseBody {
    fn from(
        CreateCircleOutput {
            circle_id,
            owner_id,
        }: CreateCircleOutput,
    ) -> Self {
        CreateCircleResponseBody {
            circle_id,
            owner_id,
        }
    }
}

pub async fn handle_create_circle(
    State(state): State<AppState>,
    Json(body): Json<CreateCircleRequestBody>,
) -> Result<Json<CreateCircleResponseBody>, String> {
    let circle_circle_input = CreateCircleInput::from(body);
    let mut usecase = CreateCircleUsecase::new(state.circle_repository);
    usecase
        .execute(circle_circle_input)
        .await
        .map(CreateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetcheCircleResponseBody {
    pub circle_id: i32,
    pub circle_name: String,
    pub capacity: i32,
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

impl std::convert::From<FetchCircleOutput> for FetcheCircleResponseBody {
    fn from(
        FetchCircleOutput {
            circle_id,
            circle_name,
            capacity,
            owner,
            members,
        }: FetchCircleOutput,
    ) -> Self {
        FetcheCircleResponseBody {
            circle_id,
            circle_name,
            capacity,
            owner,
            members,
        }
    }
}

pub async fn handle_fetch_circle(
    State(state): State<AppState>,
    Path(param): Path<FetchCircleInputParam>,
) -> Result<Json<FetcheCircleResponseBody>, String> {
    let fetch_circle_input = FetchCircleInput::new(param.id);
    let usecase = FetchCircleUsecase::new(state.circle_repository);
    usecase
        .execute(fetch_circle_input)
        .await
        .map(FetcheCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInputParam {
    id: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateCircleRequestBody {
    pub circle_name: Option<String>,
    pub capacity: Option<i32>,
}

impl UpdateCircleRequestBody {
    pub fn convert_to_input(self, id: i32) -> UpdateCircleInput {
        UpdateCircleInput::new(id, self.circle_name, self.capacity)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateCircleResponseBody {
    pub id: i32,
}

impl std::convert::From<UpdateCircleOutPut> for UpdateCircleResponseBody {
    fn from(UpdateCircleOutPut { id }: UpdateCircleOutPut) -> Self {
        UpdateCircleResponseBody { id }
    }
}

pub async fn handle_update_circle(
    State(state): State<AppState>,
    Path(path): Path<UpdateCircleInputParam>,
    Json(body): Json<UpdateCircleRequestBody>,
) -> Result<Json<UpdateCircleResponseBody>, String> {
    let update_circle_input = body.convert_to_input(path.id);
    let mut usecase = UpdateCircleUsecase::new(state.circle_repository);

    usecase
        .execute(update_circle_input)
        .await
        .map(UpdateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
