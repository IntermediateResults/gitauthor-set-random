use std::process::{Command, Stdio, exit};

use anyhow::{Context, Result};
use bstr::ByteSlice;
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Args {
    name: String,

    #[arg(required = true)]
    email: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let output = Command::new("git")
        .args(["config", "get", "--global", "user.name"])
        .stderr(Stdio::inherit())
        .output()
        .context("getting user.name from git config")?;

    if !output.status.success() {
        exit(1)
    }

    if output.stdout.as_bstr().trim_ascii() == args.name.as_bytes() {
        let mut rng = rand::thread_rng();
        let emails = args.email;
        let i: usize = rng.gen_range(0..emails.len());
        let email = &emails[i];
        let status = Command::new("git")
            .args(["config", "set", "--global", "user.email", email])
            .status()
            .context("setting user.email of git config")?;
        if !status.success() {
            exit(1);
        }
    }

    Ok(())
}
