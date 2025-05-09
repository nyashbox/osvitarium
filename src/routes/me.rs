use crate::{app::status::AppStatus as Status, repositories::user::User};

use sea_orm::prelude::Json as SeaJson;

use axum::{Json, extract::Extension};

use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    user_id: i32,
    username: String,
    fullname: String,
    description: String,
    role: String,
    role_id: i32,
    metadata: SeaJson,
}

pub async fn me_get_handler(Extension(user): Extension<User>) -> Result<Json<Response>, Status> {
    let user_role: String;
    let user_aux_sub: i32;

    let user = match user {
        User::Student(model, student) => {
            user_aux_sub = student.student_id;
            user_role = "student".into();

            model
        }
        User::Teacher(model, teacher) => {
            user_aux_sub = teacher.teacher_id;
            user_role = "teacher".into();

            model
        }
        User::Principal(model, principal) => {
            user_aux_sub = principal.principal_id;
            user_role = "principal".into();

            model
        }
    };

    Ok(Json(Response {
        user_id: user.user_id,
        username: user.username,
        fullname: user.fullname,
        description: user.description,
        role: user_role,
        role_id: user_aux_sub,
        metadata: user.metadata,
    }))
}

#[cfg(test)]
mod tests {
    mod me_get_handler {
        use std::sync::Arc;

        use tower::ServiceExt;

        use axum::{body::Body, http::StatusCode};
        use entity::{
            sea_orm_active_enums::UserRole, student::Model as StudentModel,
            user::Model as UserModel,
        };

        use crate::{
            app::state::AppState,
            repositories::user::{MockUserRepository, User},
            routes::build_routes,
        };

        use axum::http::Request;

        #[tokio::test]
        pub async fn success() {
            let mut mock = MockUserRepository::new();
            let auth_header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdXhfc3ViIjoxLCJzdWIiOjEsImlhdCI6MTExMTExMTExMTEsImV4cCI6OTk5OTk5OTk5OTksInJvbGUiOiJzdHVkZW50In0.zOIV8xbN1eIM_n7AciKbuTkpgKbCHK6Kf1vFMgv3SKY";

            mock.expect_find_by_id().returning(move |_| {
                Box::pin(async move {
                    Ok(User::Student(
                        UserModel {
                            user_id: 1,
                            username: "johndoe".into(),
                            fullname: "John Doe".into(),
                            password: "password".into(),
                            description: " ".into(),
                            metadata: "{}".into(),
                            role: Some(UserRole::Student),
                        },
                        StudentModel {
                            user_id: 1,
                            student_id: 1,
                        },
                    ))
                })
            });

            let router = build_routes(Arc::new(AppState {
                db: mock,
                secret: "secret".into(),
            }));

            let request = Request::builder()
                .uri("/me")
                .method("GET")
                .header("content-type", "application/json")
                .header("Authorization", auth_header)
                .body(Body::empty())
                .unwrap();

            let res = router.oneshot(request).await.unwrap();

            assert_eq!(
                res.status(),
                StatusCode::OK,
                "When user uses CORRECT JWT TOKEN, 'Ok' (200) MUST BE returned"
            );
        }
    }
}
