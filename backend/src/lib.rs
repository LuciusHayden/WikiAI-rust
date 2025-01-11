pub mod openai;
pub mod scraper;

use openai::*;
use scraper::*;


pub struct AppState {
    references : scraper::References,
    llmclient : openai::LLMClient,
}

impl AppState {
    pub async fn new(url : &str, options : LlmOptions) -> AppState {
        
        let references = References::new(url).await; 
        let llmclient = LLMClient::new(&references, options).await;
        AppState {references , llmclient}
    }

    pub async fn llm_query(&self, query : &str) -> String {
        self.llmclient.query(query).await
    }
}




#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn query() {
        let app_state = AppState::new("https://en.wikipedia.org/wiki/Chocolate_chip_cookie", LlmOptions::RAG).await;
        // let result = app_state.llm_query("what is your context?").await;
        println!("{}", app_state.references.references[0].link);
        // println!("{}", result);
    }

}
