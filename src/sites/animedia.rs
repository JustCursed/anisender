use scraper::{Html, Selector};

pub async fn has_timer(document: Html) -> bool {
	document
		.select(&Selector::parse(r#"div#timer"#).unwrap())
		.next()
		.is_some()
}

pub async fn get_series(document: Html) -> u32 {
	document
		.select(&Selector::parse(r#"div.info"#).unwrap())
		.next()
		.unwrap()
		.inner_html()
		.replace('>', "")
		.replace("</", "")
		.split("span")
		.map(String::from)
		.collect::<Vec<String>>()[1]
		.split(' ')
		.map(String::from)
		.collect::<Vec<String>>()[0]
		.parse::<u32>()
		.unwrap()
}
