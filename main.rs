// src/main.rs
mod token;
mod lexer;
mod parser;
mod htmlgen;
mod scope;

use parser::FrontEnd;
use std::{
    env, fs,
    path::PathBuf,
    
};


// From the handout’s “Compiler” trait (simplified mapping)
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

// Minimal shim to satisfy the shape; our Parser already drives tokens internally.
#[allow(dead_code)]
struct Driver;
impl Compiler for Driver {
    fn compile(&mut self, _source: &str) {}
    fn next_token(&mut self) -> String { String::new() }
    fn parse(&mut self) {}
    fn current_token(&self) -> String { String::new() }
    fn set_current_token(&mut self, _tok: String) {}
}

/* ---------- Browser helpers ---------- */

#[cfg(not(target_os = "windows"))]

fn file_url_from_path(p: &Path) -> String {
    let abs = std::fs::canonicalize(p).unwrap_or_else(|e| {
        eprintln!("Failed to canonicalize '{}': {}", p.display(), e);
        std::process::exit(1);
    });
    format!("file:///{}", abs.to_string_lossy().replace('\\', "/"))
}



#[cfg(target_os = "windows")]
fn open_in_chrome(html_path: &std::path::Path) {
    use std::process::Command;

    // Get absolute path like C:\Users\rvasa\lolcompiler\test\sample.html
    let abs = std::fs::canonicalize(html_path).unwrap_or_else(|e| {
        eprintln!("Failed to canonicalize '{}': {}", html_path.display(), e);
        std::process::exit(1);
    });
    let abs_str = abs.to_string_lossy().to_string();

    // Open with Windows' URL handler (default browser). This reliably opens local files.
    let _ = Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", &abs_str])
        .spawn();

    // Fallback: START with absolute path (also uses default handler)
    let _ = Command::new("cmd")
        .args(["/C", "start", "", &abs_str])
        .spawn();
}


#[cfg(target_os = "macos")]
fn open_in_chrome(html_path: &Path) {
    let url = file_url_from_path(html_path);
    // Try Chrome; fallback to the default handler
    let _ = Command::new("open")
        .args(["-a", "Google Chrome", &url])
        .spawn();
    let _ = Command::new("open").arg(&url).spawn();
}

#[cfg(all(unix, not(target_os = "macos")))]
fn open_in_chrome(html_path: &Path) {
    let url = file_url_from_path(html_path);
    // Try common Chrome binaries; then fallback to xdg-open
    let _ = Command::new("google-chrome").arg(&url).spawn()
        .or_else(|_| Command::new("chromium").arg(&url).spawn())
        .or_else(|_| Command::new("xdg-open").arg(&url).spawn());
}

/* ---------- Main ---------- */

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    if !input_path.to_ascii_lowercase().ends_with(".lol") {
        eprintln!("Error: input must have .lol extension (per project spec).");
        std::process::exit(1);
    }

    let source = fs::read_to_string(input_path).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", input_path, e);
        std::process::exit(1);
    });

    // Compile
    let fe = FrontEnd::new(&source, input_path);
    let html = fe.run();

    // Write output .html next to input
    let mut out = PathBuf::from(input_path);
    out.set_extension("html");
    fs::write(&out, html).unwrap_or_else(|e| {
        eprintln!("Failed to write '{}': {}", out.display(), e);
        std::process::exit(1);
    });

    println!("Wrote {}", out.display());

    // Open the result in a browser (Chrome preferred on Windows/macOS)
    open_in_chrome(&out);
}
