use std::{env, fs, process, ffi::OsStr};
use anyhow::Result;
use process::Command;

fn exec_cargo_clean(dir: &str)-> Result<()> {
    println!("cargo cleanup dir: [{}]", dir);
    let out = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(dir)
            .args(&["/C", "cargo clean"])
            .output()?
    }
    else {
        Command::new("sh")
            .current_dir(dir)
            .arg("-c")
            .arg("cargo clean")
            .output()?
    };
    println!("res => [{}]", String::from_utf8_lossy(out.stdout.as_slice()));
    Ok(())
}

fn exec_dotnet_clean(dir: &str)-> Result<()> {
    println!("dotnet clean dir: [{}]", dir);
    let out = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(dir)
            .args(&["/C", "dotnet clean"])
            .output()?
    }
    else {
        Command::new("sh")
            .current_dir(dir)
            .arg("-c")
            .arg("dotnet clean")
            .output()?
    };
    println!("res => [{}]", String::from_utf8_lossy(out.stdout.as_slice()));
    Ok(())
}

fn walk_dir_recur(dir_path: &str)-> Result<bool> {
    let mut toml_exists = false;
    let mut csporj_exists = false;

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let mut path = entry.path();
        let meta = entry.metadata();
        if meta.is_err() {
            continue;
        }
        let meta = meta.unwrap();

        println!("path: {}, is_dir: {:?}", path.as_path().to_string_lossy(), meta.is_dir());
        if meta.is_dir() {
            let target_dir = path.to_string_lossy().to_string();
            let _ = walk_dir_recur(&target_dir)?;
        }
        else {
            if !toml_exists {
                if path.file_name().unwrap().to_string_lossy() == "Cargo.toml" {
                    toml_exists = true;
                    if path.pop() {
                        let target_dir = path.to_string_lossy().to_string();
                        exec_cargo_clean(&target_dir)?;
                    }
                }
            }
            if !csporj_exists {
                if path.extension().unwrap_or(OsStr::new("")).to_string_lossy() == "csporj" {
                    csporj_exists = true;
                    if path.pop() {
                        let target_dir = path.to_string_lossy().to_string();
                        exec_dotnet_clean(&target_dir)?;
                    }
                }
            }
        }
    }

    Ok(toml_exists)
}

fn main()-> Result<()> {
    let current_dir = env::current_dir()?.to_string_lossy().to_string();
    let start_dir: Vec<String> = std::env::args().skip(1).collect();
    let dir: &str = if start_dir.len() > 0 { &start_dir[0] } else { &current_dir };
    println!("entry point dir: [{}]", dir);
    if walk_dir_recur(dir)? {
        exec_cargo_clean(dir)?;
    }

    Ok(())
}
