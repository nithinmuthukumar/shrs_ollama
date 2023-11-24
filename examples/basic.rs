use shrs::prelude::*;
use shrs_ollama::OllamaPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(OllamaPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
