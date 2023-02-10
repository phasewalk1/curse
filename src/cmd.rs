use crate::manif::OutputFormat;
use argh::FromArgs;

#[derive(Debug, Clone, PartialEq)]
pub enum RunningMode
{
    Empty,
    New,
    Version,
    Check,
    Compile,
    Build,
    Clean,
}

/// compile .tex source
#[derive(FromArgs, Debug, Clone)]
#[argh(subcommand, name = "compile")]
pub struct CompileCmd
{
    /// compile file
    #[argh(option, short = 'c')]
    pub src: String,
    /// output format
    #[argh(option, short = 'o')]
    pub output: Option<OutputFormat>,
}

/// compile workspace
#[derive(FromArgs, Debug, Clone)]
#[argh(subcommand, name = "build")]
pub struct BuildCmd {}

/// check manifest for validity
#[derive(FromArgs, Debug, Clone)]
#[argh(subcommand, name = "check")]
pub struct CheckCmd {}

// creat a subcommand for compiling options
#[derive(FromArgs, Debug, Clone)]
#[argh(subcommand)]
pub enum SubCommands
{
    Compile(CompileCmd),
    Check(CheckCmd),
    Build(BuildCmd),
}

/// sexy command line interface
#[derive(FromArgs, Debug)]
pub struct CurseCli
{
    /// install dependencies
    #[argh(option, short = 'i')]
    new: Option<String>,
    /// check version
    #[argh(switch, short = 'v')]
    version: Option<bool>,
    /// subcommands
    #[argh(subcommand)]
    subcommand: Option<SubCommands>,
}

impl From<&CurseCli> for RunningMode
{
    fn from(cli: &CurseCli) -> Self
    {
        if cli.new.is_some()
        {
            return RunningMode::New;
        }
        else if cli.version.is_some()
        {
            return RunningMode::Version;
        }
        else if cli.subcommand.is_some()
        {
            match cli.subcommand.as_ref().unwrap()
            {
                SubCommands::Compile(_) => return RunningMode::Compile,
                SubCommands::Check(_) => return RunningMode::Check,
                SubCommands::Build(_) => return RunningMode::Build,
            }
        }
        else
        {
            return RunningMode::Empty;
        }
    }
}

impl CurseCli
{
    pub fn print_help(&self) -> ()
    {
        let help = "Usage: curse [-i <new>] [-v] [<command>] [<args>]

Options:
-i, --new         install dependencies
-v, --version     check version
--help            display usage information

Commands:
compile           compile .tex source
check             check manifest for validity
build             compile workspace
";
        println!("{}", help);
    }
    pub fn new(&self) -> Option<String> { return self.new.clone(); }
    pub fn version(&self) -> Option<bool> { return self.version; }
    pub fn subcommand(&self) -> Option<SubCommands> { return self.subcommand.clone(); }
    pub fn empty(&self) -> bool
    {
        return self.new().is_none() && self.version().is_none() && self.subcommand().is_none();
    }
    pub fn mode(&self) -> RunningMode { return RunningMode::from(self.clone()); }
}
