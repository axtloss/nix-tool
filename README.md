# nix-tool
Add packages to your configuration.nix </br>
Except there are two issues:
- The code is so "optimized" its barely even readble anymore
- The packages are only accepted via stdin

## Installation
```sh
git clone https://github.com/axtloss/nix-tool.git
cd nix-tool
rustc -C debuginfo=0 -C embed-bitcode=no -C force-frame-pointers=y -C opt-level=z -C target-cpu=native main.rs -o nix-tool
# or
rustc main.rs -o nix-tool
```

### Usage
```sh
echo "<pkgs>" | nix-tool path/to/configuration.nix
```
The modified configuration will be saved as `newConfiguration.nix` in the directory where you ran the tool

## How it works
The code is so small i can dissect it line-by-line and explain it here:
```rust
use std::io::{Read, Write};
```
This Part just imports `std::io::Read` and `std::io::Write` <br>
Those were actually the only libraries I couldnt figure out how to use without importing them!<br>
<br>
```rust
fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
```
this just is the header of `main` which return some Results of commands<br>
<br>
```rust
let args: std::vec::Vec<String> = std::env::args().collect();
```
Here we just read the args the user passed, only the first one is needed but I couldnt figure out how to only import the first arg <br>
<br>
```rust
let mut buffer = String::new();
```
We create a new String in which the stdin input will be saved <br>
<br>
```rust
std::io::stdin().read_to_string(&mut buffer)?;
```
The stdin input gets read into `buffer` which is also just used once, but I cant figure out how to use it without creating a variable<br>
<br>
```rust
write!(std::fs::File::create("newConfiguration.nix")?, "{}", format!("{}environment.systemPackages = with pkgs;\n  [\n{}\n{}",std::fs::read_to_string(&args[1]).expect("cant open new configuration").split("environment.systemPackages = with pkgs;\n  [").collect::<Vec<&str>>()[0],&buffer.replace(" ", "\n"),std::fs::read_to_string(&args[1]).expect("cant open new configuration").split("environment.systemPackages = with pkgs;\n  [").collect::<Vec<&str>>()[1])).expect("cant write into new configuration");
```
This is the fun part!
so, we use write!() to write the new, modified configuration into `newConfiguraiton.nix` this file is created in here immediatly with `std::fs::File::creat("newconfiguration.nix)` <br>
The whole contents are also created on the fly inside of `write!()` with `format!()`<br>
First we read the user specified file into a string, immediatly split it at `environment.systemPackages = with pkgs;\n  [` and write the first part of this new, splitted string <br>
then we add `environment.systemPackages = with pkgs;\n  [` again, since the `.split()` seems to remove that part<br>
after that we add the `buffer` variable, which contains all the packages, but replace spaces with "\n" so that each package is in its own line <br>
then we split the user specified configuartion again, but this time we add the second part of the splitted file<br>
<br>
```rust
Ok(()) }
```
To be honest, I dont really know what `Ok(())` does, all I know is that its needed when the function is expected to return status stuff
