mod models;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json;
use crate::models::Assurance;

async fn calcul_gain_impot(assurance: web::Json<Assurance>) -> impl Responder {
    let revenu = assurance.revenu;
    let impots = calculer_impot(revenu);
    HttpResponse::Ok().json(json!({
        "impot_total 1": impots
    }))
}

fn calculer_impot(revenu: f64) -> f64 {
    let mut impot_total = 0.0;
    if revenu <= 5000.0 {
        impot_total = revenu * 0.0;
    } else if revenu <= 20000.0 {
        impot_total = (revenu - 5000.0) * 0.26;
    } else if revenu <= 30000.0 {
        impot_total = (revenu - 20000.0) * 0.28 + 0.26 * (20000.0 - 5000.0);
    } else if revenu <= 50000.0 {
        impot_total = (revenu - 30000.0) * 0.32 + 0.28 * (30000.0 - 20000.0) + 0.26 * (20000.0 - 5000.0);
    } else {
        impot_total = (revenu - 50000.0) * 0.35 + 0.32 * (50000.0 - 30000.0) + 0.28 * (30000.0 - 20000.0) + 0.26 * (20000.0 - 5000.0);
    }
    impot_total
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/calcul_gain", web::post().to(calcul_gain_impot))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
