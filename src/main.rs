extern crate argh;
use argh::FromArgs;
use std::error::Error;

mod helpers;
use helpers::init;

/// sexy command line interface
#[derive(FromArgs, Debug)]
struct Cli
{
    /// install dependencies
    #[argh(option, short = 'i')]
    new: Option<String>,
    /// compile file
    #[argh(option, short = 'c')]
    compile: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>>
{
    let cli: Cli = argh::from_env();
    println!("{:?}", cli);
    if cli.new != None
    {
        crate::init::init_workspace(&cli.new.unwrap())?;
    }
    if cli.compile != None
    {
        helpers::compiler::compile_to_pdf(&cli.compile.unwrap(), None)?;
    }

    return Ok(());
}
