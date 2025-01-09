use crate::openai; 


pub trait ReferencesTrait {
    fn query (question : &str) -> &str;
}

pub struct References {
    pub references : Vec<Reference>,
}

impl References {
    pub fn new(url : &str) -> References {
        get_references(url)
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

pub fn get_references(url : &str) -> References {

    let response = reqwest::blocking::get(url);
    let html_string = response.unwrap().text().unwrap();

    println!("{}", html_string);

    let html = scraper::Html::parse_document(&html_string); 

    let references_selector = scraper::Selector::parse("ol.references").unwrap();
    let references = html.select(&references_selector);

    let mut references_vec : Vec<Reference> = Vec::new(); 

    for reference in references {
        let link = reference.select(&scraper::Selector::parse("a").unwrap())
            .next().and_then(|a| a.value().attr("href")).map(|s| s.to_owned()).unwrap_or_default();

        let new_ref = Reference { link };
        references_vec.push(new_ref);
    }
    References {references : references_vec }

}

#[cfg(test)]
pub mod test {
    use super::*;

//    #[test]
    fn test_scraper() {
        get_references("https://en.wikipedia.org/wiki/Chocolate_chip_cookie");
    }

}
