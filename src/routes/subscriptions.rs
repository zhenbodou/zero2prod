use actix_web::{HttpResponse, web};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection,
    prelude::{DateTimeWithTimeZone, Uuid},
};

use crate::entity::subscriptions;

pub async fn subscribe(
    body: web::Json<SubscribeAddRequestBody>,
    connection: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let body = body.into_inner();
    let model = subscriptions::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(body.email),
        name: ActiveValue::Set(body.name),
        subscribed_at: ActiveValue::Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
    };
    let _result = model.insert(connection.get_ref()).await;
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct SubscribeAddRequestBody {
    pub name: String,
    pub email: String,
}
