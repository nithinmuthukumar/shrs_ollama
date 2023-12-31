use reqwest::blocking::{self, Client};
use serde::{Deserialize, Serialize};
use shrs::anyhow::Result;

use crate::OllamaState;
#[derive(Serialize, Deserialize)]
pub struct GenerateParams {
    model: String,
    prompt: String,
    stream: bool,
    context: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Options>,
}
impl GenerateParams {
    fn new(prompt: String, model: String, context: Vec<u32>) -> Self {
        Self {
            model,
            prompt,
            stream: false,
            context,
            options: None,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Options {
    num_predict: u32,
}
#[derive(Serialize, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub response: String,
    pub created_at: String,
    pub context: Vec<u32>,
    //total_duration,load_duration,prompt_eval_count, prompt_eval_duration,eval_count,eval_duration
}
#[derive(Debug)]
pub struct OllamaClient {
    client: Client,
    url: String,
}
impl OllamaClient {
    pub(crate) fn new() -> OllamaClient {
        OllamaClient {
            client: blocking::Client::new(),
            url: "http://127.0.0.1:11434/api".to_string(),
        }
    }
    pub fn generate(&self, prompt: String, state: &OllamaState) -> Result<GenerateResponse> {
        let url = format!("{}/generate", self.url);
        let body = serde_json::to_string(&GenerateParams::new(
            prompt,
            state.model.clone(),
            state.context.clone(),
        ))?;
        let res: GenerateResponse = self.client.post(url).body(body).send()?.json()?;

        Ok(res)
    }
}
