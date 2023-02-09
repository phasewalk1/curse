#![allow(dead_code)]

use os_info::Type;
use std::error::Error;
use std::process::Command;

pub fn init_workspace(name: &str) -> Result<(), Box<dyn Error>>
where
{
    std::fs::create_dir(name)?;
    std::fs::create_dir(format!("{}/output", name))?;
    std::fs::create_dir(format!("{}/src", name))?;
    return Ok(());
}

pub fn macos_install_deps<'deps>() -> Result<(), Box<dyn Error>>
where
{
    let mut proc = Command::new("brew")
        .args(vec!["install", "--cask", "mactex"])
        .spawn()
        .expect("Failed to spawn process");
    let _ = proc.wait().map_err(|e| e.to_string())?;
    return Ok(());
}

fn determine_distribution() -> Result<os_info::Type, Box<dyn Error>>
where
{
    let os = os_info::get();
    return Ok(os.os_type());
}

pub fn linux_install_deps<'deps>() -> Result<(), Box<dyn Error>>
where
{
    let distro = determine_distribution()?;
    let pkgmgr: String;
    let pkg: String;
    let install_cmd: String;

    match distro
    {
        Type::Arch =>
        {
            pkgmgr = "pacman".to_string();
            pkg = "texlive-most".to_string();
            install_cmd = "-S".to_string();
        }
        Type::Debian =>
        {
            pkgmgr = "apt".to_string();
            pkg = "texlive-full".to_string();
            install_cmd = "install".to_string();
        }
        _ =>
        {
            return Err("Unsupported Linux distribution.".into());
        }
    }

    let mut proc = Command::new("sudo")
        .args(vec![pkgmgr, install_cmd, pkg])
        .spawn()
        .expect("Failed to spawn process");
    let _ = proc.wait().map_err(|e| e.to_string())?;
    return Ok(());
}
