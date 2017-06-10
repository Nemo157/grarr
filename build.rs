extern crate include_dir;

use std::env;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::Command;

use include_dir::include_dir;

fn main() {
  if let Ok("release") = env::var("PROFILE").as_ref().map(|s| &**s) {
    println!("cargo:rustc-cfg=feature=\"cache\"");
  }

  let rev = if let Ok(output) = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output() {
    format!("Some(\"{}\")", String::from_utf8_lossy(&output.stdout).trim())
  } else {
    "None".to_owned()
  };

  let date = if let Ok(output) = Command::new("date").arg("+%F").output() {
    format!("Some(\"{}\")", String::from_utf8_lossy(&output.stdout).trim())
  } else {
    "None".to_owned()
  };

  let out_dir = env::var("OUT_DIR").unwrap();
  let version_path = Path::new(&out_dir).join("version.rs");
  let mut f = File::create(&version_path).unwrap();

  f.write_all(format!("
    static REVISION: Option<&'static str> = {};
    static DATE: Option<&'static str> = {};
  ", rev, date).as_bytes()).unwrap();

  let assets = Path::new(&out_dir).join("static.rs");
  include_dir("src/static").as_variable("FILES").to_file(assets).unwrap();
}
