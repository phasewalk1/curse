use std::error::Error;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Error as IoErr;
use std::process::Command;

fn filter_src(x: &Result<DirEntry, IoErr>) -> bool
{
    let x = x.as_ref().unwrap();
    let path = x.path();
    if path.is_file()
    {
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "tex"
        {
            return true;
        }
        else
        {
            return false;
        }
    }
    else
    {
        return false;
    }
}

pub fn batch_compile(manif: &crate::CurseManifest) -> Result<(), Box<dyn Error>>
where
{
    log::debug!("Batch compiling with manifest: {:?}", manif);
    let outdir = manif.defaults.target_dir.clone();
    let srcdir = manif.get_src().unwrap_or("src".to_string());
    let _ = match manif.check()
    {
        true =>
        {}
        false => return Err("Manifest is invalid".to_string().into()),
    };
    let src_files: Vec<String> = std::fs::read_dir(srcdir)?
        .into_iter()
        .filter(|x| filter_src(&x))
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect();
    let _ = match src_files.len()
    {
        0 => return Err("No source files found!".into()),
        _ =>
        {
            for file in src_files
            {
                let _ = match manif.defaults.target.as_str()
                {
                    "dvi" =>
                    {
                        compile_to_dvi(&file, Some(&outdir))?;
                    }
                    _ =>
                    {
                        compile_to_pdf(&file, Some(&outdir))?;
                    }
                };
            }
        }
    };

    return Ok(());
}

pub fn single_compile(manif: &crate::CurseManifest) -> Result<(), Box<dyn Error>>
{
    let main = match manif.get_main()
    {
        Some(m) => m,
        None => return Err("No main file specified in manifest!".into()),
    };
    let _ = match manif.defaults.target.as_str()
    {
        "dvi" =>
        {
            compile_to_dvi(&main, None)?;
        }
        _ =>
        {
            compile_to_pdf(&main, None)?;
        }
    };
    return Ok(());
}

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
    let outdir: &str = out.unwrap_or("./output");
    let outarg = format!("--output-directory={}", outdir);
    let mut proc = Command::new("latex")
        .args(vec!["--halt-on-error", outarg.as_str(), file])
        .spawn()
        .expect("Failed to spawn process");
    let _ = proc.wait().map_err(|e| e.to_string())?;
    return Ok(());
}
