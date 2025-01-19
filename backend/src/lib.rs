pub mod openai;
pub mod scraper;
pub mod api;

use openai::*;
use scraper::*;


pub struct AppState {
    main_reference : Option<Reference>,
    references : scraper::References,
    llmclient : openai::LLMClient,
}

impl AppState {
    pub async fn new(url : &str, options : LlmOptions) -> AppState {
        
        let references = References::new(url).await; 
        let llmclient = LLMClient::new(&references, options).await;
        AppState {main_reference : Some(Reference { link : url.to_string() } ), references , llmclient}
    }

    pub async fn new_empty() -> AppState {
        let references = References::new_empty().await;
        let llmclient = LLMClient::new(&references, LlmOptions::BASE).await;

        AppState {main_reference : None, references, llmclient }
    }

    pub async fn get_references(&self) -> Vec<Reference>{
        self.references.references.clone()
    }

    pub async fn set_references(&mut self, url : &str) {
        let references =  References::new(url).await;
        self.references = references;
        self.reload_llmclient(LlmOptions::RAG).await;
    }

    async fn reload_llmclient(&mut self, options: LlmOptions) {
        let llmclient = LLMClient::new(&self.references, options).await;
        self.llmclient = llmclient;
    }

    pub async fn llm_query(&self, query : &str) -> String {
        self.llmclient.query(query).await
    }

    pub async fn get_main_reference(&self) -> Option<Reference> {
        self.main_reference.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn query() {
        let app_state = AppState::new("https://en.wikipedia.org/wiki/Chocolate_chip_cookie", LlmOptions::RAG).await;
        let result = app_state.llm_query("what is rust?").await;
        // println!("{}", app_state.references.references[0].link);
        println!("{}", result);
    }

}
