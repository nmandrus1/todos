mod args;
use args::Args;
use structopt::StructOpt;

use std::path::Path;
use std::process;
use std::ffi::OsString;

const EXTS: [&str; 2] = ["rs", "cpp"];

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();

    if let Some(file) = args.file {
        if !file.exists() {
            eprintln!("File does not exist");
            return Ok(())
        }
        get_todos(&file);
        return Ok(())
    }

    let cwd = std::env::current_dir()?;

    if !is_root_dir(&cwd) {
        eprintln!("Error: Not in root directory");
        process::exit(0)
    }

    traverse_directory(&cwd)?;

    Ok(())
}

fn get_todos(path: &Path) {
    use std::fs;
    let file_string = fs::read_to_string(path).unwrap();

    file_string
        .lines()
        .enumerate()
        .filter(|(_, l)| l.contains("TODO") && l.trim().starts_with("//"))
        .map(|(i, _)| i)
        .enumerate()
        .for_each(|(count, i)| {
            println!("======== {:?} Todo {} ========", path.file_name().unwrap(), count);
            file_string
                .lines()
                .skip(i)
                .take_while(|line| line.trim().starts_with("//"))
                .enumerate()
                .for_each(|(line_num_offset, line)| {
                    let line = line.trim()[2..].trim();
                    if line == "TODO" {
                        return
                    } else if line.starts_with("TODO") {
                        println!("{} - {}", i + &line_num_offset + 1, &line[4..].trim())
                    } else {
                        println!("{} - {}", i + &line_num_offset + 1, &line)
                    }
                });
            // TODO add the line number to each 
            // println statement
            println!()
           }) 
}

fn traverse_directory(path: &Path) -> anyhow::Result<()> {
    let iter = std::fs::read_dir(path)?;

    for entry in iter {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            traverse_directory(&path)?;
        }

        if let Some(extension) = path.extension() {
            if EXTS.iter().any(|ext| ext.eq(&extension.to_str().unwrap())) {
                get_todos(&path)
            }
        }
    }

    Ok(())
}

fn is_root_dir(cwd: &Path) -> bool {
    match std::fs::read_dir(cwd) {
        Ok(mut iter) => {
            iter
            .any(|e| {
                let e = match e {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        process::exit(0);
                    }
                };

                e.file_name().eq(&OsString::from("Cargo.toml"))
                || e.file_name().eq(&OsString::from(".git"))
            })
        },

        Err(e) =>{ 
            eprintln!("Error reading directory:\n\n{}", e.to_string());
            process::exit(0)
        }
    }
}
