fn main() {
  if let Ok("release") = std::env::var("PROFILE").as_ref().map(|s| &**s) {
    println!("cargo:rustc-cfg=feature=\"cache\"");
  }
}
