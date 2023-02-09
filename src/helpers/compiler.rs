use std::error::Error;
use std::fs::File;
use std::process::Command;

pub fn compile_to_pdf(file: &str, out: Option<&str>) -> Result<(), Box<dyn Error>>
where
{
    println!("Compiling {} to pdf...", file);
    let _ = File::open(file)?;
    let outdir: &str = out.unwrap_or("./output");
    let outarg = format!("--output-directory={}", outdir);
    let mut proc = Command::new("pdflatex")
        .args(vec!["--halt-on-error", outarg.as_str(), file])
        .spawn()
        .expect("Failed to spawn process");
    log::debug!("Running: {:?}", proc);
    let _ = proc.wait().map_err(|e| e.to_string())?;
    return Ok(());
}

pub fn compile_to_dvi(file: &str, out: Option<&str>) -> Result<(), Box<dyn Error>>
where
{
    println!("Compiling {} to dvi...", file);
    let _ = File::open(file)?;
    let mut proc = Command::new("latex")
        .args(vec![file, "--output-directory=", out.unwrap_or("output")])
        .spawn()
        .expect("Failed to spawn process");
    let _ = proc.wait().map_err(|e| e.to_string())?;
    return Ok(());
}
