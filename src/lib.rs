use shrs::{
    anyhow::{self, Result},
    prelude::{AfterCommandCtx, BuiltinCmd, Context, Plugin, Runtime, Shell},
};

pub struct OllamaPlugin {}
pub struct OllamaState {
    last_output: String,
}

impl Plugin for OllamaPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("gpt", OllamaBuiltin {});
        shell.hooks.register(save_output);
        todo!()
    }
}
pub struct OllamaBuiltin {}
impl BuiltinCmd for OllamaBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<shrs::prelude::CmdOutput> {
        todo!()
    }
}
pub fn save_output(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    cmd_ctx: &AfterCommandCtx,
) -> Result<()> {
    if let Some(state) = ctx.state.get_mut::<OllamaState>() {
        state.last_output = cmd_ctx.cmd_output.stderr.clone();
    }
    Ok(())
}
