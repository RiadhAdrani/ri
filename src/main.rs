#![allow(unused)]

mod eslint;
mod git;
mod jest;
mod licence;
mod npm;
mod readme;
mod typescript;

use eslint::{use_eslint, use_eslint_ignore};
use git::use_git_ignore;
use jest::use_jest_config;
use licence::use_licence;
use npm::{use_npm_ignore, use_pkg_json};
use readme::use_readme;
use typescript::use_ts_config;

use chrono::{self, Datelike};
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::{
    fmt::format,
    process::{self, Command},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// type
    #[arg(short, long, default_value_t = false)]
    lib: bool,

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

fn web(name: &String, framework: &String, lang: &String, mgr: &String) -> () {
    process::Command::new(mgr)
        .arg("create")
        .arg("vite")
        .arg(name)
        .arg(if mgr == "npm" { "--" } else { "" })
        .arg("--template")
        .arg(format!(
            "{}{}",
            framework,
            if lang == "ts" { "-ts" } else { "" }
        ))
        .status();
}

fn lib(name: &String, mgr: &String) -> std::io::Result<()> {
    let ts_config = use_ts_config();
    let eslint_ignore = use_eslint_ignore();
    let eslint_rc = use_eslint();
    let git_ignore = use_git_ignore();
    let npm_ignore = use_npm_ignore();
    let licence_f = use_licence();
    let readme_f = use_readme(name);
    let jest_config = use_jest_config();

    let package_json = use_pkg_json(name);

    let idx = String::from("export {};");

    fs::create_dir(name);
    fs::create_dir(format!("{}/src", name));

    let mut tsconfig =
        File::create(format!("{}/tsconfig.json", name))?.write_all(use_ts_config().as_bytes())?;
    let mut eslintignore =
        File::create(format!("{}/.eslintignore", name))?.write_all(eslint_ignore.as_bytes())?;
    let mut eslintrc =
        File::create(format!("{}/.eslintrc", name))?.write_all(eslint_rc.as_bytes())?;
    let mut gitignore =
        File::create(format!("{}/.gitignore", name))?.write_all(git_ignore.as_bytes())?;
    let mut npmignore =
        File::create(format!("{}/.npmignore", name))?.write_all(npm_ignore.as_bytes())?;
    let mut licence = File::create(format!("{}/LICENSE", name))?.write_all(licence_f.as_bytes());
    let mut readme = File::create(format!("{}/README.md", name))?.write_all(readme_f.as_bytes());
    let mut jestconfig =
        File::create(format!("{}/jest.config.js", name))?.write_all(jest_config.as_bytes());
    let mut packagejson =
        File::create(format!("{}/package.json", name))?.write_all(package_json.as_bytes());
    let mut index = File::create(format!("{}/src/index.ts", name))?.write_all(idx.as_bytes());

    Ok(())
}

fn main() -> () {
    let args = Args::parse();

    if (args.lib) {
        lib(&args.name, &args.mgr);
    } else {
        web(&args.name, &args.fmk, &args.lang, &args.mgr);
    }
}
