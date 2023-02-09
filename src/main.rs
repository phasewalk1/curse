extern crate argh;
use argh::FromArgs;
use std::error::Error;

mod helpers;
mod manif;
use helpers::init;
use manif::Manifest as CurseManifest;

/// sexy command line interface
#[derive(FromArgs, Debug)]
struct Cli
{
    /// install dependencies
    #[argh(option, short = 'i')]
    new: Option<String>,
    /// subcommands
    #[argh(subcommand)]
    subcommand: Option<SubCommands>,
}

// creat a subcommand for compiling options
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands
{
    Compile(CompileCmd),
}

#[derive(Debug, PartialEq)]
enum OutputFormat
{
    PDF,
    DVI,
}

impl std::str::FromStr for OutputFormat
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "pdf" => Ok(OutputFormat::PDF),
            "dvi" => Ok(OutputFormat::DVI),
            _ => Err(format!("{} is not a valid output format", s)),
        }
    }
}

impl Default for OutputFormat
{
    fn default() -> Self { OutputFormat::PDF }
}

/// compilation
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "compile")]
struct CompileCmd
{
    /// compile file
    #[argh(option, short = 'c')]
    src: String,
    /// output format
    #[argh(option, short = 'o')]
    output: Option<OutputFormat>,
}

fn main() -> Result<(), Box<dyn Error>>
{
    pretty_env_logger::try_init()?;
    let cli: Cli = argh::from_env();
    println!("{:?}", cli);
    if cli.new != None
    {
        let name = cli.new.unwrap();
        crate::init::init_workspace(&name.clone())?;
        log::info!("Initialized workspace at {}", name);
        return Ok(());
    }
    match cli.subcommand.unwrap()
    {
        SubCommands::Compile(compile) => match compile.output.unwrap_or(OutputFormat::default())
        {
            OutputFormat::PDF =>
            {
                crate::helpers::compiler::compile_to_pdf(&compile.src, Some("output"))?;
            }
            OutputFormat::DVI =>
            {
                crate::helpers::compiler::compile_to_dvi(&compile.src, Some("output"))?;
            }
        },
    }

    return Ok(());
}
