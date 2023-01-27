#![allow(unused)]

use clap::Parser;
use std::{process::{Command, self}, fmt::format};
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use chrono::{self, Datelike};

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

fn lib(name: &String, mgr: &String) -> std::io::Result<()>{
  

  let ts_config  = String::from("") + 
  "{\n" +
    "\"exclude\": [\"node_modules\", \"src/**/*.test.ts\", \"./jest.config.js\"],\n" +
    "\"compilerOptions\": {\n"+
    "\"target\": \"es2016\",\n"+
    "\"lib\": [\"es6\", \"dom\"],\n"+
    "\"module\": \"commonjs\",\n"+
    "\"rootDir\": \".\",\n"+
    "\"resolveJsonModule\": true,\n"+
    "\"allowJs\": true,\n"+
    "\"declaration\": true,\n"+
    "\"outDir\": \"build\",\n"+
      "\"esModuleInterop\": true,\n"+
      "\"forceConsistentCasingInFileNames\": true,\n"+
      "\"strict\": true,\n"+
      "\"noImplicitAny\": true,\n"+
      "\"skipLibCheck\": true\n"+
    "}\n"+
  "}";

  let eslint_ignore = String::from("") + 
    "node_modules\n" + 
    "dist\n" + 
    "build\n";

  let eslint_rc = String::from("") + 
  "{\n"+
    "\"root\": true,"+
    "\"parser\": \"@typescript-eslint/parser\","+
    "\"plugins\": [\"@typescript-eslint\"],\n"+
    "\"extends\": [\n"+
      "\"eslint:recommended\",\n"+
      "\"plugin:@typescript-eslint/eslint-recommended\",\n"+
      "\"plugin:@typescript-eslint/recommended\""+
    "]\n"+
  "}";

  let git_ignore = String::from("") + 
  "node_modules\n" + 
  "dist\n" + 
  "build\n" + 
  "./docs/.vitepress/dist";

  let npm_ignore = String::from("") + 
  ".github\n" + 
  "vscode\n" + 
  "docs\n" + 
  "README.md\n" + 
  "src/**/*.test.js\n" + 
  "src/**/*.test.ts\n" + 
  ".eslintignore\n" + 
  ".eslintrc\n" + 
  ".gitignore\n" + 
  "jest.config.js\n" + 
  "tsconfig.json\n" + 
  "package-lock.json\n";

  let licence_f = format!("MIT License\n\nCopyright (c) {} Riadh Adrani\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: \n\nThe above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.",chrono::Utc::now().year().to_string());

  let readme_f = format!("# {}",name);

  let jest_config = String::from("")
  +"/** @type {import('ts-jest').JestConfigWithTsJest} */\n"
  +"module.exports = {\n"
  +"\tpreset: \"ts-jest\",\n"
  +"};"; 

  let package_json = String::from("") + 
  "{"+
  "\"name\":\""+ name + "\",\n"+
  "\"version\": \"0.0.0\",\n"+
  "\"description\": \"\",\n"+
  "\"types\": \"build/index.d.ts\",\n"+
  "\"main\": \"build/index.js\",\n"+
  "\"files\": [\"build/**/*\"],\n"+
  "\"author\": \"riadh-adrani\",\n"+
  "\"licence\": \"mit\",\n"+
  "\"keywords\": [],\n"+
  "\"scripts\": {\n"+
    "\"test\":\"jest\",\n" +
    "\"build\":\"rm -rf build && tsc\",\n" +
  "},\n"+
  "\"devDependencies\": {\n"+
    "\"@types/node\":\"^18.11.9\",\n" +
    "\"@typescript-eslint/eslint-plugin\":\"^5.42.1\",\n" +
    "\"@typescript-eslint/parser\":\"^5.42.1\",\n" +
    "\"eslint\":\"^8.27.0\",\n" +
    "\"jest\":\"^29.0.0\",\n" +
    "\"ts-jest\":\"^29.0.3\",\n" +
    "\"typescript\":\"^4.9.3\",\n" +
  "},\n"+
  "}";

  let idx = String::from("export {};");

  fs::create_dir(name);
  fs::create_dir(format!("{}/src",name));

  let mut tsconfig = File::create(format!("{}/tsconfig.json",name))?.write_all(ts_config.as_bytes())?;
  let mut eslintignore = File::create(format!("{}/.eslintignore",name))?.write_all(eslint_ignore.as_bytes())?;
  let mut eslintrc = File::create(format!("{}/.eslintrc",name))?.write_all(eslint_rc.as_bytes())?;
  let mut gitignore = File::create(format!("{}/.gitignore",name))?.write_all(git_ignore.as_bytes())?;  
  let mut npmignore = File::create(format!("{}/.npmignore",name))?.write_all(npm_ignore.as_bytes())?;
  let mut licence = File::create(format!("{}/LICENSE",name))?.write_all(licence_f.as_bytes());
  let mut readme = File::create(format!("{}/README.md",name))?.write_all(readme_f.as_bytes());
  let mut jestconfig = File::create(format!("{}/jest.config.js",name))?.write_all(jest_config.as_bytes());
  let mut packagejson = File::create(format!("{}/package.json",name))?.write_all(package_json.as_bytes());
  let mut index = File::create(format!("{}/src/index.ts",name))?.write_all(idx.as_bytes());

  Ok(())
}

fn main() -> (){
  let args = Args::parse();

  if (args.lib){
    lib(&args.name, &args.mgr);
  }else {
    web(&args.name, &args.fmk, &args.lang, &args.mgr);
  }

}

