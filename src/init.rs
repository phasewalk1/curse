#![allow(dead_code)]

use os_info::Type;
use std::error::Error;
use std::process::Command;

type DepsEnv<'deps> = (bool, String, Vec<&'deps str>);

pub fn deps_env<'deps>() -> Result<DepsEnv<'deps>, Box<dyn Error>>
where
{
    pretty_env_logger::try_init().ok();
    let info = os_info::get();
    let os = info.os_type();
    let mut needs_sudo: bool = false;
    let pkgcmd: String;
    let cmdargs: Vec<&str>;

    match os
    {
        Type::Arch =>
        {
            needs_sudo = true;
            pkgcmd = "pacman".to_string();
            cmdargs = vec!["-S", "--noconfirm", "texlive-most"];
        }
        Type::Debian =>
        {
            needs_sudo = true;
            pkgcmd = "apt-get".to_string();
            cmdargs = vec!["install", "-y", "texlive-full"];
        }
        Type::Gentoo =>
        {
            todo!()
        }
        Type::Macos =>
        {
            pkgcmd = "brew".to_string();
            cmdargs = vec!["install", "--cask", "mactex"];
        }
        _ =>
        {
            println!("Your OS is not supported yet.");
            println!("Please install the following packages manually:");
            println!("texlive-most (Arch Linux)");
            println!("texlive-full (Debian)");
            println!("mactex (MacOS)");
            println!("texlive (Gentoo)");
            return Err("Unsupported OS".into());
        }
    }

    return Ok((needs_sudo, pkgcmd, cmdargs));
}

pub fn install_deps<'deps>(depenv: DepsEnv<'deps>) -> Result<(), Box<dyn Error>>
where
{
    let (needs_sudo, pkgcmd, cmdargs) = depenv;
    match needs_sudo
    {
        true =>
        {
            let after_sudo = vec![pkgcmd, cmdargs.join(" ")];
            let mut proc = Command::new("sudo")
                .args(after_sudo)
                .spawn()
                .expect("Failed to spawn process");
            let _ = proc.wait().map_err(|e| e.to_string())?;
        }
        false =>
        {
            let mut proc = Command::new(pkgcmd)
                .args(cmdargs)
                .spawn()
                .expect("Failed to spawn process");
            let _ = proc.wait().map_err(|e| e.to_string())?;
        }
    }
    return Ok(());
}
