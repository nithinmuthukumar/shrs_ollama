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

    prompt: Vec<String>,
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
enum Commands {
    Reset,
    Model { model: String },
    Prompt,
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

        let oc = ctx.state.get::<OutputCaptureState>().unwrap();
        let lo = oc.last_output.clone();
        let c = oc.last_command.clone();

        if let Some(state) = ctx.state.get_mut::<OllamaState>() {
            if let Some(command) = cli.command {
                match command {
                    Commands::Model { model } => (),
                    Commands::Prompt => {
                        let response = state.client.generate(
                            cli.prompt.join(" "),
                            state.model.clone(),
                            state.context.clone(),
                        )?;
                        ctx.out.println(response.response)?;
                    }
                    Commands::Reset => {
                        state.context.clear();
                    }
                    Commands::Debug => {
                        let response = state.client.generate(
                            format!(
                                "Debug this input:{} output:{:?} {}",
                                c,
                                lo,
                                cli.prompt.join(" ")
                            ),
                            state.model.clone(),
                            state.context.clone(),
                        )?;
                        ctx.out.println(response.response)?;
                    }
                }
            }
        }

        Ok(CmdOutput::success())
    }
}
