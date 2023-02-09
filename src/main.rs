extern crate argh;
use argh::FromArgs;
use std::error::Error;

mod init;

/// sexy command line interface
#[derive(FromArgs, Debug)]
struct Cli
{
    /// install dependencies
    #[argh(switch, short = 'i')]
    init: bool,
}

fn main() -> Result<(), Box<dyn Error>>
{
    let cli: Cli = argh::from_env();
    match cli.init
    {
        true =>
        {
            let de = crate::init::deps_env()?;
            crate::init::install_deps(de)?;
        }
        false =>
        {
            println!("No command given.");
        }
    }
    return Ok(());
}
