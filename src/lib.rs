use lazy_static::lazy_static;
use reqwest::{Client, header::HeaderMap};
use serde::{de::DeserializeOwned, Serialize};

lazy_static! {
	pub static ref CLIENT: Client = Client::builder()
		.redirect(reqwest::redirect::Policy::none())
		.build()
		.unwrap();
}

pub struct Database {
	url: String,
	username: String,
	password: String,
	database: String,
	namespace: String,
	headers: HeaderMap,
}

impl Default for Database {
	fn default() -> Self {
		Database::new()
	}
}

impl Database {
	pub fn new() -> Database {
		Database {
			url: "".into(),
			database: "".into(),
			username: "".into(),
			password: "".into(),
			namespace: "".into(),
			headers: HeaderMap::new(),
		}
	}

	pub fn url(&mut self, url: String) -> &mut Database {
		self.url = url;
		self
	}

	pub fn username(&mut self, username: String) -> &mut Database {
		self.username = username;
		self
	}

	pub fn password(&mut self, password: String) -> &mut Database {
		self.password = password;
		self
	}

	pub fn database(&mut self, database: String) -> &mut Database {
		self.database = database;
		self
	}

	pub fn namespace(&mut self, namespace: String) -> &mut Database {
		self.namespace = namespace;
		self
	}

	pub fn build(&mut self) -> Database {
		self.headers.insert("DB", (self.database).parse().unwrap());
		self.headers.insert("NS", (self.namespace).parse().unwrap());
		self.headers
			.insert("Accept", "application/json".parse().unwrap());
		self.headers.insert(
			"Content-Type",
			"application/x-www-form-urlencoded".parse().unwrap(),
		);

		Database {
			url: self.url.clone(),
			username: self.username.clone(),
			password: self.password.clone(),
			database: self.database.clone(),
			namespace: self.namespace.clone(),
			headers: self.headers.clone(),
		}
	}

	// The SQL endpoint enables advanced SurrealQL queries (POST /sql)
	pub async fn sql(self, sql_query: &'static str) -> String {
		CLIENT
			.post("http://0.0.0.0:8000/sql")
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.body(sql_query)
			.send()
			.await
			.expect("Failed to send sql request")
			.text()
			.await
			.expect("Failed to get a response text")
	}

	// Selects all records in a table from the database (GET /key/:table)
	pub async fn select_table<T: DeserializeOwned>(self, table: &str) -> T {
		CLIENT
			.get(format!("http://0.0.0.0:8000/key/{table}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.send()
			.await
			.expect("Failed to send get request")
			.json::<T>()
			.await
			.expect("Failed to get a json from struct")
	}

	// Creates a record in the database table (POST /key/:table)
	pub async fn create_record<T: Serialize + ?Sized>(self, table: &str, body: &T) -> String {
		CLIENT
			.post(format!("http://0.0.0.0:8000/key/{table}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.json(body)
			.send()
			.await
			.expect("Failed to send post request")
			.text()
			.await
			.expect("Failed to get a response text")
	}

	// Deletes all records in a table from the database (DELETE /key/:table)
	pub async fn delete_all_records(self, table: &str) -> String {
		CLIENT
			.delete(format!("http://0.0.0.0:8000/key/{table}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.send()
			.await
			.expect("Failed to send delete request")
			.text()
			.await
			.expect("Failed to get a response text")
	}

	// Selects the specific record from the database (GET /key/:table/:id)
	pub async fn select_by_record_id<T: DeserializeOwned>(self, table: &str, id: &str) -> T {
		CLIENT
			.get(format!("http://0.0.0.0:8000/key/{table}/{id}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.send()
			.await
			.expect("Failed to send get request")
			.json::<T>()
			.await
			.expect("Failed to get a json from struct")
	}

	// Creates the specific record in the database (POST /key/:table/:id)
	pub async fn create_specific_record<T: Serialize + ?Sized>(
		self,
		table: &str,
		id: &str,
		body: &T,
	) -> String {
		CLIENT
			.post(format!("http://0.0.0.0:8000/key/{table}/{id}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.json(body)
			.send()
			.await
			.expect("Failed to send post request")
			.text()
			.await
			.expect("Failed to get a response text")
	}

	// Updates the specified record in the database (PUT /key/:table/:id)
	pub async fn update_record<T: Serialize + ?Sized>(
		self,
		table: &str,
		id: &str,
		body: &T,
	) -> String {
		CLIENT
			.put(format!("http://0.0.0.0:8000/key/{table}/{id}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.json(body)
			.send()
			.await
			.expect("Failed to send put request")
			.text()
			.await
			.expect("Failed to get a response text")
	}

	// Modifies the specified record in the database (PATCH /key/:table/:id)
	pub async fn modify_record<T: Serialize + ?Sized>(
		self,
		table: &str,
		id: &str,
		body: &T,
	) -> String {
		CLIENT
			.patch(format!("http://0.0.0.0:8000/key/{table}/{id}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.json(body)
			.send()
			.await
			.expect("Failed to send patch request")
			.text()
			.await
			.expect("Failed to get a response text")
	}

	// Deletes the specified record from the database (DELETE /key/:table/:id)
	pub async fn delete_record(self, table: &str, id: &str) -> String {
		CLIENT
			.delete(format!("http://0.0.0.0:8000/key/{table}/{id}"))
			.basic_auth(self.username, Some(self.password))
			.headers(self.headers)
			.send()
			.await
			.expect("Failed to send delete request")
			.text()
			.await
			.expect("Failed to get a response text")
	}
}
