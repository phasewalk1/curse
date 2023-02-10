extern crate argh;
use argh::FromArgs;
use std::error::Error;

mod cmd;
mod helpers;
mod manif;
use cmd::{CurseCli, RunningMode, SubCommands};
use helpers::init;
use manif::Manifest as CurseManifest;
use manif::OutputFormat;

fn main() -> Result<(), Box<dyn Error>>
{
    pretty_env_logger::try_init()?;
    let cli: CurseCli = argh::from_env();

    let mode: RunningMode = cli.mode();
    match mode
    {
        RunningMode::Empty =>
        {
            cli.print_help();
        }
        RunningMode::New =>
        {
            crate::init::init_workspace(&cli.new().unwrap())?;
        }
        RunningMode::Build =>
        {
            let manif = CurseManifest::new("Curse.toml");
            crate::helpers::builder::batch_compile(&manif)?;
        }
        RunningMode::Check =>
        {
            let manif = CurseManifest::new("Curse.toml");
            let _ = match manif.check()
            {
                true => log::info!("Manifest is valid"),
                false => log::error!("Manifest is invalid"),
            };
        }
        RunningMode::Version =>
        {
            println!("Curse version {}", env!("CARGO_PKG_VERSION"));
        }
        RunningMode::Compile =>
        {
            let compile = cli.subcommand().unwrap();
            match compile
            {
                SubCommands::Compile(compile) =>
                {
                    match compile.output.unwrap_or(OutputFormat::default())
                    {
                        OutputFormat::PDF =>
                        {
                            crate::helpers::builder::compile_to_pdf(&compile.src, Some("output"))?;
                        }
                        OutputFormat::DVI =>
                        {
                            crate::helpers::builder::compile_to_dvi(&compile.src, Some("output"))?;
                        }
                    }
                }
                _ =>
                {}
            }
        }
        RunningMode::Clean =>
        {
            todo!()
        }
    }

    return Ok(());
}
