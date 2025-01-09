pub mod openai;
pub mod scraper;

use openai::*;
use scraper::*;

pub struct AppState {
    references : scraper::References,
    llmclient : openai::LLMClient,
}

impl AppState {
    pub async fn new(url : &str) -> AppState {

        let references = References::new(url); 

        let llmclient = LLMClient::new().await;

        AppState {references , llmclient}
    }
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn query() {
        // openai::process_data("best cookies", "https://en.wikipedia.org/wiki/Chocolate_chip_cookie");
    }

}
