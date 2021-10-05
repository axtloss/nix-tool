use std::io::{Read, Write};
fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let args: std::vec::Vec<String> = std::env::args().collect();
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    write!(std::fs::File::create("newConfiguration.nix")?, "{}", format!("{}environment.systemPackages = with pkgs;\n  [\n{}\n{}",std::fs::read_to_string(&args[1]).expect("cant open new configuration").split("environment.systemPackages = with pkgs;\n  [").collect::<Vec<&str>>()[0],&buffer.replace(" ", "\n"),std::fs::read_to_string(&args[1]).expect("cant open new configuration").split("environment.systemPackages = with pkgs;\n  [").collect::<Vec<&str>>()[1])).expect("cant write into new configuration");
    Ok(()) }