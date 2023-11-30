use clap::{Parser, Subcommand};
use request::OllamaClient;
use reqwest::Client;
use shrs::{
    anyhow::{self, Result},
    prelude::{AfterCommandCtx, BuiltinCmd, CmdOutput, Context, Plugin, Prompt, Runtime, Shell},
};
use shrs_output_capture::{OutputCapturePlugin, OutputCaptureState};
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
}
#[derive(Debug, Subcommand)]
enum Commands {
    Reset,
    Model { model: String },
    Prompt { prompt: Vec<String> },
    Debug,
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

        let last_output = if let Some(output_capture) = ctx.state.get::<OutputCaptureState>() {
            Some(output_capture.last_output.clone())
        } else {
            None
        };

        if let Some(state) = ctx.state.get_mut::<OllamaState>() {
            if let Some(command) = cli.command {
                match command {
                    Commands::Model { model } => (),
                    Commands::Prompt { prompt } => {
                        let response = state.client.generate(
                            prompt.join(" "),
                            state.model.clone(),
                            state.context.clone(),
                        )?;
                        ctx.out.println(response.response)?;
                    }
                    Commands::Reset => {
                        state.context.clear();
                    }
                    Commands::Debug => {
                        if let Some(lo) = last_output {
                            println!("{:?}", lo);
                            let response = state.client.generate(
                                format!("Debug this {:?}", lo.stderr.clone()),
                                state.model.clone(),
                                state.context.clone(),
                            )?;
                            ctx.out.println(response.response)?;
                        }
                    }
                }
            }
        }

        Ok(CmdOutput::success())
    }
}
