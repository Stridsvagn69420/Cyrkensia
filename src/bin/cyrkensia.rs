use std::{env, io};
use std::process::exit;
use cyrkensia::Config;
use cyrkensia::meta::WIKI_HELP_URL;
use cyrkensia::server::CyrkensiaState;
use cyrkensia::server::redirect::trail_slash;
use cyrkensia::server::routes::{index, hostinfo};
use cyrkensia::server::middleware::{cors_everywhere, source_headers, license_headers};
use actix_web::{web, App, HttpServer};
use kagero::printer::{Printer, Colors};

/// Init
/// 
/// Server preparations.
fn init() -> io::Result<Config> {
	// Safe Args Collect to String
	let args: Vec<String> = env::args_os()
	.filter_map(|x| {
		let Some(arg) = x.to_str() else {
			return None;	
		};
		Some(arg.to_string())
	})
	.collect();

	// Config
	Config::load_cascade(args.get(1))
}

/// Server
/// 
/// Server startup.
async fn server(cfg: Config) -> io::Result<()> {
	// ---- Server Init ----
	let mut printer = Printer::default();
	let bindaddr = cfg.bindaddr.clone();
	let unbound_server = HttpServer::new(move || {
		// Initialize state
		let Ok(state) = CyrkensiaState::new(cfg.clone()) else {
			eprintln!("Cyrkensia failed trying to initialize!");
			exit(1);
		};

		// ---- App ----
		App::new()
		// State
		.app_data(web::Data::new(state))
		// Middleware
		.wrap(cors_everywhere())
		.wrap(source_headers())
		.wrap(license_headers())
		//Routes
		.route("/", web::get().to(hostinfo))
		.route("/{album}/", web::get().to(index))
		.route("/{album}", web::get().to(trail_slash))
	});

	// ---- Server Bind ----
	#[cfg(target_family = "unix")]
	let server = if bindaddr.starts_with('/') {
		unbound_server.bind_uds(bindaddr)?
	} else {
		unbound_server.bind(bindaddr)?
	};

	#[cfg(not(target_family = "unix"))]
	let server = unbound_server.bind(bindaddr)?;

	// ---- Ignite ----
	printer.println("Cyrkensia server successfully started!", Colors::CyanBright);
	server.run().await
}

#[actix_web::main]
async fn main() {
	// Init
	let mut console = Printer::default();
	let Ok(config) = init() else {
		console.errorln("Failed to read the config file for Cyrkensia!", Colors::RedBright);
		console.errorln(&("See ".to_owned() + WIKI_HELP_URL + " for more."), Colors::YellowBright);
		exit(1);
	};

	// Start
	if let Err(serv) = server(config).await {
		console.errorln("An error occured while running the server:", Colors::RedBright);
		eprintln!("{serv}");
	}

	// Exit
	console.println("Cyrkensia server successfully stopped!", Colors::CyanBright);
	exit(0)
}