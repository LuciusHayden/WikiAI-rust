use crate::scraper::*;

use langchain_rust::{
    chain::{Chain, LLMChainBuilder, ConversationalRetrieverChainBuilder},
    fmt_message, fmt_template,
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
    vectorstore::{VectorStore, Retriever},
    url,
    memory::SimpleMemory,
};

use url::Url;
use futures_util::StreamExt;
use serde::Serialize;

pub struct LLMClient {
    chain : Box<dyn Chain>,
}

pub enum LlmOptions {
    RAG,
    BASE,
}

impl LLMClient {
    pub async fn new(references : &References, option : LlmOptions) -> LLMClient {
        let embedder = OpenAiEmbedder::default();

        let client = Qdrant::from_url("http://127.0.0.1:6334").build().unwrap();

        let store = StoreBuilder::new().embedder(embedder).client(client).collection_name("wikiAI").build().await.unwrap();

        let open_ai = OpenAI::default().with_model(OpenAIModel::Gpt35.to_string());

        let prompt = message_formatter![fmt_message!(Message::new_system_message(
            "You are a helpful assistant"
            )),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!("{question}", "question")))];

        let chain :Box<dyn Chain> = match option {
            LlmOptions::BASE => Box::new(LLMChainBuilder::new().prompt(prompt).llm(open_ai.clone()).build().unwrap()),
            LlmOptions::RAG => {
                use langchain_rust::{
                    fmt_template, message_formatter, prompt::HumanMessagePromptTemplate,
                    template_jinja2,
                };
                
                let prompt= message_formatter![
                    fmt_template!(HumanMessagePromptTemplate::new(
                    template_jinja2!("
            Use the following pieces of context to answer the question at the end. If the context doesnt contain the answer, just say that you don't know, don't try to make up an answer.
            If differnt sources say different things then say that. 

            {{context}}

            Question:{{question}}
            Helpful Answer:

        ",
                    "context","question")))

                ];

                let store = store_documents(references, Box::new(store)).await;

                Box::new(ConversationalRetrieverChainBuilder::new()
                    .llm(open_ai.clone()).rephrase_question(false).memory(SimpleMemory::new().into())
                    .retriever(Retriever::new(store, 3)).prompt(prompt).build().expect("Error building ConversationalRetriever"))
            }
        };

        LLMClient { chain }
    }

    pub async fn query(&self, query : &str) -> QueryResult {
        let results = self.chain.execute(prompt_args!{"question" => query}).await.unwrap();

        let mut sources: Vec<Reference> = Vec::new();
        let response = results.get("output").unwrap().to_string();

        if let Some(source) = results.get("source_documents") {
            let items = source.as_array().unwrap().iter().map(|object| object.get("metadata").unwrap().get("source").unwrap());
            for source in items {
                let reference = Reference {link : source.to_string()  ,id: 0 };
                let _ = &sources.push(reference);
            }
        }

        let sources = References { references : sources };
        QueryResult { response, sources }
    }
}

#[derive(Serialize)]
pub struct QueryResult {
    response : String,
    sources : References,
}

async fn store_documents(references : &References, storage : Box<dyn VectorStore>) ->  Box<dyn VectorStore>{
    use langchain_rust::vectorstore::VecStoreOptions;

    for reference in references.references.iter() {
        if reference.link.contains("https") {
            let documents = convert_reference_to_docs(reference).await;
            if let Err(_) = storage.add_documents(&documents, &VecStoreOptions::default()).await {
                println!("vectorstorage filled");
                break;
            }
        }
    }
    storage
}

async fn convert_reference_to_docs(reference : &Reference)-> Vec<Document >{

    let url : &str = &reference.link;

    let response = reqwest::get(url).await;

    let html  = response.unwrap().text().await.unwrap();

    let html_loader = HtmlLoader::from_string(html, Url::parse(url).unwrap());

    let document = html_loader
        .load()
        .await
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .await;

    document
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test() {

        
    }
}
