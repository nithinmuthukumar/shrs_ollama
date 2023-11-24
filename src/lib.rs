use clap::{Parser, Subcommand};
use request::OllamaClient;
use reqwest::Client;
use shrs::{
    anyhow::{self, Result},
    prelude::{AfterCommandCtx, BuiltinCmd, CmdOutput, Context, Plugin, Runtime, Shell},
};
use shrs_output_capture::OutputCapturePlugin;
mod request;

pub struct OllamaPlugin {}
impl Plugin for OllamaPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("gpt", OllamaBuiltin {});
        shell.state.insert(OllamaState::new());
        // shell.hooks.register(save_output);
        Ok(())
    }
}
impl OllamaPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug)]
pub struct OllamaState {
    pub client: OllamaClient,

    pub model: String,
    pub context: Vec<u32>,
}
impl OllamaState {
    pub fn new() -> Self {
        Self {
            model: "codellama".to_string(),
            context: Vec::new(),
            client: OllamaClient::new(),
        }
    }
}

//gpt debug - debugs last
//gpt --reset
//gpt --model model
//gpt "prompt"
#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    prompt: Option<Vec<String>>,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Reset,
    Model { model: String },
}

pub struct OllamaBuiltin {}
impl BuiltinCmd for OllamaBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;
        if let Some(prompt) = cli.prompt {
            if let Some(state) = ctx.state.get_mut::<OllamaState>() {
                let response = state.client.generate(
                    prompt.join(" "),
                    state.model.clone(),
                    state.context.clone(),
                )?;
                ctx.out.println(response.response)?;
            }
        }

        Ok(CmdOutput::success())
    }
}
