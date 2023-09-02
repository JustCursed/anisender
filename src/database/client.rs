use once_cell::sync::Lazy;
use surrealdb::{
	engine::remote::ws::{Client, Ws},
	opt::auth::Root,
	Surreal,
};
use surrealdb::engine::remote::ws::Wss;

use crate::config::cfg;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn setup() -> surrealdb::Result<()> {
	DB.connect::<Ws>(cfg.db.url.clone()).await?;

	let _ = DB.signin(Root {
		username: &cfg.db.username,
		password: &cfg.db.password,
	});

	DB.use_ns(cfg.db.namespace.clone())
		.use_db(cfg.db.database.clone())
		.await?;

	// for site in &cfg.other.sites {
	// 	if DB.query("SELECT * FROM sites WHERE id = $site").bind(("site", site)).await?.take::<Option<>>(0) {
	// 		DB.query("CREATE $table SET id = $id, last_series: 0")
	// 			.bind(("table", "sites"))
	// 			.bind(("id", site))
	// 			.await?;
	// 	}
	// }

	Ok(())
}
