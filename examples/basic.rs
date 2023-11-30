use shrs::prelude::*;
use shrs_mux::MuxPlugin;
use shrs_ollama::OllamaPlugin;
use shrs_output_capture::OutputCapturePlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(OllamaPlugin::new())
        .with_plugin(OutputCapturePlugin)
        .with_plugin(MuxPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
