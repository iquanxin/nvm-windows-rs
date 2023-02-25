use std::{io::ErrorKind, path::Path};

mod node;
mod util;

static VERSION: &str = "0.1.0";

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
        "uninstall" => uninstall(&detail),
        "use" => use_fn(&detail),
        "list" => list(),
        "-V" => version(),
        "version" => version(),
        _ => help(),
    }
}

fn install(version: &str, arch: &str) {
    let nvm_home = std::env::var("NVM_HOME").unwrap();

    let mut version = version.replace("v", "");
    version = version.replace("-", "");

    let install_path = Path::new(&nvm_home).join("v".to_string() + &version);

    let node_path = install_path.join("node.exe");

    if util::is_version_installed(&node_path) {
        println!("\nVersion {} is already installed.", version);
        return;
    }

    let url = node::get_nodejs_url(&version, &arch);

    println!("\nDownloading nodejs version: v{} ({})", version, arch);

    node::download(&url, &version, install_path);

    println!(
        "Installation complete. If you want to use this version, type\n\nnvm use {}",
        version
    );
}

fn uninstall(version: &str) {
    let nvm_home = std::env::var("NVM_HOME").unwrap();

    let mut version = version.replace("v", "");
    version = version.replace("-", "");

    let install_path = Path::new(&nvm_home).join("v".to_string() + &version);

    if !install_path.exists() {
        println!("\nNodejs version v{} is not installed.", version);
        return;
    }

    std::fs::remove_dir_all(install_path).expect("Please try to use administrative privileges");

    println!("\nUninstalling nodejs v{version}... done");
}

fn use_fn(version: &str) {
    let nvm_home = std::env::var("NVM_HOME").unwrap();

    let mut version = version.replace("v", "");
    version = version.replace("-", "");

    let install_path = Path::new(&nvm_home).join("v".to_string() + &version);

    let node_path = install_path.join("node.exe");

    if !util::is_version_installed(&node_path) {
        println!("\nNodejs version {} is not installed.", version);
        return;
    }

    let symlink = std::env::var("NVM_SYMLINK").unwrap();

    match std::fs::metadata(&symlink) {
        Ok(val) => {
            if val.is_dir() {
                std::fs::remove_dir_all(&symlink).expect("Delete NVM_SYMLINK dir all failed");
            }
        }
        Err(err) => {
            if err.kind() != ErrorKind::NotFound {
                println!("Get NVM_SYMLINK metadata err:{:?}", err);
            }
        }
    }

    if let Err(_) = std::os::windows::fs::symlink_dir(&install_path, &symlink) {
        if let Err(err) = std::process::Command::new("elevate.cmd")
            .arg("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/D")
            .arg(symlink)
            .arg(install_path)
            .stderr(std::process::Stdio::null())
            .spawn()
        {
            println!("failed to execute process. err:{}", err);
            return;
        }
    };

    println!("\nNow using nodejs v{}", version);
}

fn list() {
    let nvm_home = std::env::var("NVM_HOME").unwrap();

    let nvm_home_dir_list = std::fs::read_dir(nvm_home).unwrap();
    let mut dir_list = nvm_home_dir_list
        .filter(|x| x.as_ref().unwrap().path().is_dir())
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    dir_list.sort();

    let cmd_node_v = std::process::Command::new("node").arg("-v").output();
    let mut current_use_node_version = String::from("");
    if let Ok(val) = cmd_node_v {
        current_use_node_version = String::from_utf8_lossy(&val.stdout).to_string();
    }
    current_use_node_version = current_use_node_version.replace("\r\n", "");

    println!("");

    for pb in dir_list {
        let node_version = pb.file_name().unwrap().to_string_lossy().to_string();
        let mut output = String::from("");
        if node_version.starts_with("v") {
            if node_version.eq(&current_use_node_version) {
                output += "  * ";
                output = output + &node_version + " (Currently using executable)";
            } else {
                output += "    ";
                output += &node_version;
            }

            println!("{output}");
        }
    }
}

fn version() {
    println!("{VERSION}");
}

fn help() {
    println!("\nnvm version: {VERSION}");
    println!("\nUsage:");
    println!(" ");
    println!("  nvm install <version>    : The version can be a specific version. (default: install system arch)");
    println!("  nvm list                 : List the node.js installations.");
    println!("  nvm uninstall <version>  : The version must be a specific version.");
    println!("  nvm use [version]        : Switch to use the specified version.");
    println!("  nvm -V, --version        : Show the current running version of nvm for Windows and exit.");
    println!(" ");
}
