#![allow(unused)]

use clap::Parser;
use std::{process::{Command, self}, fmt::format};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// type
  #[arg(short, long, default_value_t = String::from("web"))]
  typ: String,

  // Name of the repo
  #[arg(short, long)]
  name: String,

  /// package-manager
  #[arg(short, long, default_value_t = String::from("yarn"))]
  mgr: String,

  /// lang
  #[arg(short, long, default_value_t = String::from("ts"))]
  lang: String,

  /// lang
  #[arg(short, long, default_value_t = String::from("vanilla"))]
  fmk: String,
}

fn web(name: &String,framework: &String, lang: &String, mgr: &String) -> (){
  process::Command::new(mgr)
    .arg("create")
    .arg("vite")
    .arg(name)
    .arg(if mgr == "npm" {"--"} else {""})
    .arg("--template")
    .arg(format!("{}{}",framework, if lang == "ts" {"-ts"} else {""}))
    .status();
}

fn main() -> (){
  let args = Args::parse();

  web(&args.name, &args.fmk, &args.lang, &args.mgr)
}

