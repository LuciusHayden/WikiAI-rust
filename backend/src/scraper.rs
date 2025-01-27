use serde::{Serialize, Deserialize};

// note that escraper is the scraper crate, it had to be renamed
//
#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct References {
    pub references : Vec<Reference>,
}

impl References {
    pub async fn new(url : &str) -> References {
        get_references(url).await
    }

    pub async fn new_empty() -> References {
        References { references : Vec::new(), }
    }
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Reference {
    pub link : String,
    pub id : usize,
} 

pub async fn get_references(url : &str) -> References {

    let response = reqwest::get(url).await.unwrap();
    let html_string = response.text().await.unwrap();

    let html = escraper::Html::parse_document(&html_string); 

    let references_selector = escraper::Selector::parse("ol.references").unwrap();

    let mut references_vec : Vec<Reference> = Vec::new(); 

    let mut counter = 1;

    for reference in html.select(&references_selector) {
        for r in reference.select(&escraper::Selector::parse("span.reference-text").unwrap()) {

            if let Some(link) = r 
                .select(&escraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
            {
                references_vec.push(Reference {
                    link: link.to_owned(),
                    id : counter,
             });
                counter += 1;
            }
        }
    }
    References {references : references_vec }

}

#[cfg(test)]
pub mod test {
    use super::*;

    #[tokio::test]
    async fn test_scraper() {
        let _ = get_references("https://en.wikipedia.org/wiki/Chocolate_chip_cookie").await;
    }

}
