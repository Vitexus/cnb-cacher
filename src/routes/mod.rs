mod currency;

use actix_web::{web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/today/{currency_code}.json")
            .route(web::get().to(currency::get_today_currency)),
    )
    .service(
        web::resource("/yesterday/{currency_code}.json")
            .route(web::get().to(currency::get_yesterday_currency)),
    );
}