mod currency;

use actix_web::web;

use crate::handlers::currency::{get_today_currency, get_yesterday_currency};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/today/{CURRENCY_CODE}.json")
            .route(web::get().to(get_today_currency)),
    );
    cfg.service(
        web::resource("/yesterday/{CURRENCY_CODE}.json")
            .route(web::get().to(get_yesterday_currency)),
    );
}