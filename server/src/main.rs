use actix_web::{get, App, HttpResponse, HttpServer, middleware::Logger, web, HttpRequest, rt};
use actix_cors::Cors;
use tokio::time::{self, Duration};
use dotenv::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::error;
use env_logger::Env;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, serde::Deserialize)]
struct GenesisTime {
	genesis_time: String,
}

#[derive(Debug, serde::Deserialize)]
struct GenesisData {
	data: GenesisTime,
}

async fn slot_num() -> Result<u64> {
	let ep = env::var("BEACON_URL").expect("BEACON_URL must be set") + "/eth/v1/beacon/genesis";
	// println!("{ep}");
	let client = reqwest::Client::new();
	let resp = client.get(&ep).send().await?.text().await?;
	let genesis: GenesisData = serde_json::from_str(&resp)?;
	let time = genesis.data.genesis_time.parse::<u64>()?;

	let timestamp = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.unwrap()
					.as_secs();
	
	// println!("{timestamp}");
	Ok((timestamp - time)/12)
}

async fn epoch_num() -> Result<u64> {
	Ok(slot_num().await.unwrap() / 32)
}

#[get("/slot")]
async fn slot(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
	let (response, mut session, mut _msg_stream) = actix_ws::handle(&req, body)?;

	rt::spawn(async move {
		let mut interval = time::interval(Duration::from_secs(20));
		loop {
			interval.tick().await;
			let slot_number = slot_num().await.unwrap();
			session.text(slot_number.to_string()).await.unwrap();
		}
	});

	Ok(response)
}

#[get("/epoch")]
async fn epoch(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
	let (response, mut session, mut _msg_stream) = actix_ws::handle(&req, body)?;

	rt::spawn(async move {
		let mut interval = time::interval(Duration::from_secs(300));
		loop {
			interval.tick().await;
			let slot_number = epoch_num().await.unwrap();
			session.text(slot_number.to_string()).await.unwrap();
		}
	});

	Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	
	env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
			.wrap(
				Cors::default().allow_any_origin())
			.wrap(Logger::default())
			.service(slot)
			.service(epoch)
	})
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}