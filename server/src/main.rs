use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use dotenv::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;


#[derive(Debug, serde::Deserialize)]
struct GenesisTime {
	genesis_time: String,
}

#[derive(Debug, serde::Deserialize)]
struct GenesisData {
	data: GenesisTime,
}

async fn slot_num() -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
	let ep = env::var("BEACON_URL").expect("BEACON_URL must be set") + "/eth/v1/beacon/genesis";
	println!("{ep}");
	let client = reqwest::Client::new();
	let resp = client.get(&ep).send().await?.text().await?;
	let genesis: GenesisData = serde_json::from_str(&resp)?;
	let time = genesis.data.genesis_time.parse::<u64>()?;

	let timestamp = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.unwrap()
					.as_secs();
	
	println!("{timestamp}");
	Ok((timestamp - time)/12)
}

#[get("/slot")]
async fn slot() -> impl Responder {
	HttpResponse::Ok().body(slot_num().await.unwrap().to_string())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

    HttpServer::new(|| 
        App::new()
			.wrap(
				Cors::new().supports_credentials().finish())
			.service(slot)
    )
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}