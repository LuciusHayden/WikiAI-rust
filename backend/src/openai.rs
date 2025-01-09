use openai_api_rust::{*, chat::*};
use std::io::Cursor;
use crate::scraper;

use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_placeholder, fmt_template,
    language_models::llm::LLM,
    llm::openai::{OpenAI, OpenAIModel},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
    document_loaders::HtmlLoader,
    document_loaders::Loader,
    embedding::openai::openai_embedder::OpenAiEmbedder,
    schemas::Document,
    vectorstore::qdrant::{Qdrant, StoreBuilder},
    vectorstore::VectorStore,
    url,
};

use std::io::Write;
use url::Url;
use scraper::ReferencesTrait;
use futures_util::StreamExt;

pub struct LLMClient {
    vector_storage : Box<dyn VectorStore>,
    chain : Box<dyn Chain>,
}

impl LLMClient {
    pub async fn new() -> LLMClient {
        let embedder = OpenAiEmbedder::default();

        let client = Qdrant::from_url("https://localhost:6334").build().unwrap();

        let store = StoreBuilder::new().client(client).embedder(embedder).collection_name("wikiAI").build().await.unwrap();

        let open_ai = OpenAI::default().with_model(OpenAIModel::Gpt35.to_string());

        let prompt = message_formatter![fmt_message!(Message::new_system_message(
            "You are a helpful assistant"
            )),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!("{input}", "input")))];

        let chain = LLMChainBuilder::new().prompt(prompt).llm(open_ai.clone()).build().unwrap();

        LLMClient {vector_storage : Box::new(store), chain : Box::new(chain)}
    }
}

async fn store_documents(references : scraper::References, storage : Box<dyn VectorStore>) {
    use langchain_rust::vectorstore::VecStoreOptions;

    for reference in references.references {
        let documents = convert_reference_to_docs(reference).await;

        storage.add_documents(&documents, &VecStoreOptions::default()).await.unwrap(); 
        
    }
}


async fn convert_reference_to_docs(reference : scraper::Reference)-> Vec<Document >{

    let url : &str = &reference.link;

    let response = reqwest::blocking::get(url);
    let html : String = response.unwrap().text().unwrap();

    let html_cursor = Cursor::new(html);
  
    let html_loader = HtmlLoader::new(
        html_cursor,
        Url::parse(url).unwrap(),
        );


    let document = html_loader
        .load()
        .await
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .await;

    document
}


async fn openai_test(query : &str) {


    use langchain_rust::vectorstore::VecStoreOptions;

    let embedder = OpenAiEmbedder::default();


    let client = Qdrant::from_url("https://localhost:6334").build().unwrap();

    let store = StoreBuilder::new().client(client).embedder(embedder).collection_name("wikiAI").build().await.unwrap();


    let open_ai = OpenAI::default().with_model(OpenAIModel::Gpt35.to_string());

    let prompt = message_formatter![fmt_message!(Message::new_system_message(
            "You are a helpful assistant"
            )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!("{input}", "input")))];

    let chain = LLMChainBuilder::new().prompt(prompt).llm(open_ai.clone()).build().unwrap();

    //let html_loader = HtmlLoader::from_path()
    
    match chain.invoke(prompt_args! {
        "input" => query, 
        }).await
    {
        Ok(result) => {
            println!("Result: {}", result);
        }
        Err(e) => {
            panic!("Error {e}");
        }
    }
}

pub trait Processable {

    fn convert_data(&self) -> String; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        println!("Entering");
        openai_test("Testing").await;
        
    }
}
