use std::{fs::File, io::Read, path::PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};
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
    #[clap(long = "include")]
    context_include: Vec<ContextArgs>,
}
//What to include as extra info in the prompt, folder will include all the code in all the files in
//the prompt
#[derive(ValueEnum, Debug, Clone, Copy)]
enum ContextArgs {
    Ls,
    Envs,
    Folder,
    File,
    Directory,
}
#[derive(Debug, Subcommand)]
enum ModelCommands {}

#[derive(Debug, Subcommand)]
enum Commands {
    Reset,
    Model {
        #[command(subcommand)]
        command: Option<ModelCommands>,
    },
    Prompt {
        prompt: Vec<String>,
    },
    Debug {
        prompt: Vec<String>,
    },
    File {
        prompt: Vec<String>,
        path: PathBuf,
    },
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

        let output_capture = ctx.state.get::<OutputCaptureState>().unwrap();
        let last_output = output_capture.last_output.clone();
        let last_command = output_capture.last_command.clone();

        if let Some(state) = ctx.state.get_mut::<OllamaState>() {
            if let Some(command) = cli.command {
                match command {
                    Commands::Model { command } => match command {
                        Some(_) => todo!(),
                        None => ctx
                            .out
                            .println(format!("The current model is {}", state.model))?,
                    },
                    Commands::Prompt { prompt } => {
                        let response = state.client.generate(prompt.join(" "), state)?;
                        state.context = response.context;
                        ctx.out.println(response.response)?;
                    }
                    Commands::Reset => {
                        state.context.clear();
                    }
                    Commands::Debug { prompt } => {
                        let response = state.client.generate(
                            format!(
                                "Debug this bash where input is {} and the stdout was \"{}\" and stderr was \"{}\". Additional info: {}",
                                last_command,
                                last_output.stdout,
                                last_output.stderr,
                                prompt.join(" ")
                            ),
                            state,
                        )?;
                        state.context = response.context;
                        ctx.out.println(response.response)?;
                    }
                    Commands::File { prompt, path } => {
                        let mut file = File::open(path)?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;

                        let response = state.client.generate(prompt.join(" "), state)?;
                        state.context = response.context;
                        ctx.out.println(response.response)?;
                    }
                }
            }
        }

        Ok(CmdOutput::success())
    }
}
