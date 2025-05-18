use crate::{models::user::UserDTO, repositories::user::UserRepository, routes::prelude::*};

/// Get user profile description
#[utoipa::path(
    get,
    tag = "Users",
    path = "/users/{id}",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated"),
        (status = 404, description = "Not Found")
    ),
    security(
        ( "jwt_token" = [] )
    )
)]
pub async fn get_user_description<S>(
    State(state): State<Arc<AppState<S>>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserDTO>, Status>
where
    S: UserRepository,
{
    let user = UserRepository::find_by_id(&state.db, user_id).await?;

    Ok(Json(user.into()))
}
