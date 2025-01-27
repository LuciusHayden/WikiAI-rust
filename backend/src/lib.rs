pub mod openai;
pub mod scraper;
pub mod api;

use openai::*;
use scraper::*;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    main_reference : Option<Reference>,
    references : scraper::References,
    llmclient : Arc<Mutex<openai::LLMClient>>,
}

impl AppState {
    pub async fn new(url : &str, options : LlmOptions) -> AppState {
        
        let references = References::new(url).await; 
        let llmclient = LLMClient::new(&references, options).await;
        let llmclient = Arc::new(Mutex::new(llmclient));
        AppState {main_reference : Some(Reference { link : url.to_string(), id : 0} ), references , llmclient }
    }

    pub async fn new_empty() -> AppState {
        let references = References::new_empty().await;
        let llmclient = LLMClient::new(&references, LlmOptions::BASE).await;

        AppState {main_reference : None, references, llmclient : Arc::new(Mutex::new(llmclient)) }
    }

    pub async fn get_references(&self) -> &Vec<Reference>{
        &self.references.references
    }

    pub async fn set_references(&mut self, url : &str) {
        self.main_reference = Some(Reference {link : url.to_string(), id: 0});
        let references =  References::new(url).await;
        self.references = references;
        self.reload_llmclient(LlmOptions::RAG).await;
    }

    async fn reload_llmclient(&mut self, options: LlmOptions) {
        let llmclient = LLMClient::new(&self.references, options).await;
        let mut client = self.llmclient.lock().await;
        *client = llmclient;
    }

    pub async fn clear_references(&mut self) {
        self.references = References::new_empty().await;
    }

    pub async fn llm_query(&self, query : &str) -> openai::QueryResult {
        self.llmclient.lock().await.query(query).await
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
        // let app_state = AppState::new("https://en.wikipedia.org/wiki/Chocolate_chip_cookie", LlmOptions::RAG).await;
        let app_state = AppState::new_empty().await;
        app_state.llmclient.lock().await.query("What is the context").await;
        // println!("{}", app_state.references.references[0].link);
    }

}
