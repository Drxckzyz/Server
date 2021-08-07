use crate::auth::token_gen::generate_random_bits;
use actix_web::web::{self, HttpResponse, Path};
use actix_web::{HttpRequest, Responder};
use ferrischat_common::types::InternalServerErrorJson;
use ferrischat_macros::get_db_or_fail;
use num_traits::FromPrimitive;
use sqlx::types::BigDecimal;

pub async fn get_token(req: HttpRequest) -> impl Responder {
    let bits = match generate_random_bits() {
        Some(b) => b,
        None => {
            return HttpResponse::InternalServerError().json(InternalServerErrorJson {
                reason: "failed to generate random bits for token generation".to_string(),
            })
        }
    };
    let user_id: u128 = match req.match_info().get("user_id") {
        Some(id) => match id.parse() {
            Ok(id) => id,
            Err(e) => {
                return HttpResponse::BadRequest().json(InternalServerErrorJson {
                    reason: format!("Failed to parse user ID as u128: {}", e),
                })
            }
        },
        None => {
            return HttpResponse::InternalServerError().json(InternalServerErrorJson {
                reason: "User ID not found in match_info".to_string(),
            })
        }
    };

    let token = base64::encode_config(bits, base64::URL_SAFE);
    let db = get_db_or_fail!();
    if let Err(e) = sqlx::query!("INSERT INTO auth_tokens VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET auth_token = $2", BigDecimal::from_u128(user_id), token).execute(db).await {
        return HttpResponse::InternalServerError().json(InternalServerErrorJson {
            reason: format!("DB returned a error: {}", e)
        })
    };

    return HttpResponse::Ok().body(format!(
        "{}.{}",
        base64::encode_config(user_id.to_string(), base64::URL_SAFE),
        token,
    ));
}