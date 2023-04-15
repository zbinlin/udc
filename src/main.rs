use std::io::Write;
use std::{fs,io};
use toml;
use clap::Parser;
use udc::Udc;
use udc::Cli;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let conf = fs::read_to_string(&cli.conf)
        .with_context(|| format!("could not read file `{}`", cli.conf.display()))?;

    let _ = fs::read_dir(&cli.out)
        .with_context(|| format!("could not access directory `{}`", cli.out.display()))?;

    let n: Udc = toml::from_str(&conf)
        .with_context(|| format!("parse TOML format failure!"))?;

    for (file_name, v) in &n.0 {
        if v.len() == 0 {
            continue;
        }
        let out = &cli.out.join(file_name.to_owned() + ".conf");

        if out.exists() {
            let mut confirm = String::new();
            print!("The file `{}` already exists, are you override it? [yes/No] ", out.display());
            io::stdout().flush()?;
            io::stdin().read_line(&mut confirm)?;
            match confirm.trim().to_uppercase().as_str() {
                "Y" | "YES" => {
                    println!("[Warning]: override file `{}`", out.display());
                },
                _ => {
                    println!("Ignore override file `{}`", out.display());
                    continue;
                },
            }
        }

        let contents = v.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("\n\n");
        println!("Writing `{}`...", out.display());
        fs::write(out, contents)
            .with_context(|| format!("write `{}` failure", out.display()))?;
        println!("Write `{}` success!", out.display());
    }

    Ok(())
}
