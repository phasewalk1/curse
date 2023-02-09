use std::error::Error;
use std::fs::File;
use std::process::Command;

pub fn compile_to_pdf(file: &str, out: Option<&str>) -> Result<(), Box<dyn Error>>
where
{
    println!("Compiling {} to pdf...", file);
    let _ = File::open(file)?;
    let mut proc = Command::new("pdflatex")
        .args(vec![file, "--output-directory", out.unwrap_or("output")])
        .spawn()
        .expect("Failed to spawn process");
    let _ = proc.wait().map_err(|e| e.to_string())?;
    return Ok(());
}
