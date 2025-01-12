use crate::openai; 


pub trait ReferencesTrait {
    fn query (question : &str) -> &str;
}

pub struct References {
    pub references : Vec<Reference>,
}

impl References {
    pub async fn new(url : &str) -> References {
        get_references(url).await
    }
}

pub struct Reference {
    pub link : String,
} 



impl openai::Processable for References {

    fn convert_data(&self) -> String {
        "".to_string()
    }

}

pub async fn get_references(url : &str) -> References {

    let response = reqwest::get(url).await.unwrap();
    let html_string = response.text().await.unwrap();

    let html = scraper::Html::parse_document(&html_string); 

    let references_selector = scraper::Selector::parse("ol.references").unwrap();

    let mut references_vec : Vec<Reference> = Vec::new(); 

    for reference in html.select(&references_selector) {
        // Extract the first hyperlink within each reference
        for r in reference.select(&scraper::Selector::parse("span.reference-text").unwrap()) {

            if let Some(link) = r 
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
            {
                references_vec.push(Reference {
                    link: link.to_owned(),
             });
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
        let _blah = get_references("https://en.wikipedia.org/wiki/Chocolate_chip_cookie").await;
    }

}
