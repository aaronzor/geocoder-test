use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct GeocodeResponse {
    results: Vec<GeocodeResult>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeResult {
    geometry: GeocodeGeometry,
}

#[derive(Serialize, Deserialize)]
struct GeocodeGeometry {
    location: GeocodeLocation,
}

#[derive(Serialize, Deserialize)]
struct GeocodeLocation {
    lat: f64,
    lng: f64,
}

#[get("/coordinates/{address}")]
async fn coordinates(address: web::Path<String>) -> impl Responder {
    let encoded_address = urlencoding::encode(&address);
    let api_key = env::var("GOOGLE_GEOCODING_API_KEY").unwrap();
    let api_url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        encoded_address, api_key
    );
    let response = reqwest::get(api_url).await.unwrap();
    let geocode_response: GeocodeResponse = response.json().await.unwrap();
    let coordinates = &geocode_response.results[0].geometry.location;
    format!(
        "Latitude: {}, Longitude: {}",
        coordinates.lat, coordinates.lng
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(coordinates))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
