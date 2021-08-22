use actix_web::{web::Path, HttpResponse, Responder};
use ferrischat_common::types::{InternalServerErrorJson, NotFoundJson};

/// DELETE /api/v0/users/{user_id}
pub async fn delete_user(req: HttpRequest, auth: crate::Authorization) -> impl Responder {
    let user_id = get_item_id!(req, "user_id");

    if user_id != auth.0 {
        HttpResponse::Forbidden().finish()
    }
    let bigint_user_id = u128_to_bigdecimal!(user_id);

    let db = get_db_or_fail!();
    let resp = sqlx::query!(
        "DELETE FROM users WHERE id = $1 RETURNING (id)",
        bigint_user_id
    )
    .fetch_optional(db)
    .await?;

    match resp {
        Ok(r) => match r {
            Some(_) => HttpResponse::NoContent().finish(),
            None => HttpResponse::NotFound().json(NotFoundJson {
                message: "User not found".to_string(),
            }),
        },
        Err(e) => HttpResponse::InternalServerError().json(InternalServerErrorJson {
            reason: format!("DB Returned a error: {}", e),
        }),
    }
}
