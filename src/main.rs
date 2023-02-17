use std::path::Path;

mod node;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let mut detail = String::from("");
    let arch = std::env::consts::ARCH.to_string();

    if args.len() > 2 {
        detail = args[2].clone();
    }

    match args[1].as_str() {
        "install" => install(&detail, &arch),
        "uninstall" => uninstall(),
        "use" => apply(),
        "list" => list(),
        "-V" => version(),
        "--version" => version(),
        _ => help(),
    }
}

fn install(version: &str, arch: &str) {
    let mut version = version.replace("v", "");
    version = version.replace("-", "");
    let nvm_home = std::env::var("NVM_HOME").unwrap();
    let install_path = Path::new(&nvm_home).join("v".to_string() + &version);

    let node_path = install_path.join("node.exe");

    if util::is_version_installed(&node_path) {
        println!("Version {} is already installed.", version);
        return;
    }

    if install_path.exists() {
        if !install_path.is_dir() {
            println!("Path: {:?} already exists and is a file.", install_path);
            return;
        }
    }

    let url = node::get_nodejs_url(&version, &arch);

    println!("\nDownloading nodejs version: v{} ({})", version, arch);

    node::download(&url, &version, install_path);

    println!(
        "Installation complete. If you want to use this version, type\n\nnvm use {}",
        version
    );
}

fn uninstall() {}

fn apply() {}

fn list() {}

fn version() {
    println!("nvm version: 0.1.0");
}

fn help() {
    println!("\nnvm version: {}", "0.1.0");
    println!("\nUsage:");
    println!(" ");
    println!("  nvm install <version>    : The version can be a specific version. (default: install system arch)");
    println!("  nvm list                 : List the node.js installations.");
    println!("  nvm uninstall <version>  : The version must be a specific version.");
    println!("  nvm use [version]        : Switch to use the specified version.");
    println!("  nvm -V, --version        : Show the current running version of nvm for Windows and exit.");
    println!(" ");
}
