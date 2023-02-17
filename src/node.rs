use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn get_nodejs_url(version: &str, arch: &str) -> String {
    let mut platform = "win-x64".to_string();

    if arch == "x86" {
        platform = "win-x86".to_string();
    }

    // v0.x.x 版本中 x64在目录中 x86在 https://nodejs.org/dist/v0.x.x/ 没有存档
    // v4.5.0 之前的v4.x.x版本没有存档 node-v4.x.x-win-x86.zip、node-v4.x.x-win-x64.zip
    // v5.x.x 版本中 都没有存档
    // v6.2.1 之前的v6.x.x版本没有存档 node-v6.x.x-win-x86.zip、node-v6.x.x-win-x64.zip

    let url = format!(
        "https://nodejs.org/dist/v{}/node-v{}-{}.zip",
        version, version, platform
    );

    return url;
}

pub fn download(url: &str, version: &str, install_path: PathBuf) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::new(10, 0))
        .build()
        .unwrap();

    let mut response = client
        .get(url)
        .send()
        .expect(&format!("Failed to download nodejs version: {}", version));

    if response.status() == 404 {
        println!("Nodejs v{} not found.", version);
        std::fs::remove_dir_all(install_path).expect("Remove install path error");
        return;
    }

    let total_size: usize = response
        .headers()
        .get("Content-Length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    let mut writer_buf: Vec<u8> = Vec::with_capacity(total_size);

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
    )
    .unwrap()
    .progress_chars("#>-"));

    let mut tmp_buf = vec![0u8; 16 * 1024];
    loop {
        match response.read(&mut tmp_buf) {
            Ok(0) => break,
            Ok(n) => {
                for i in &tmp_buf[0..n] {
                    writer_buf.push(*i)
                }
                pb.inc(n as u64);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => println!("response.read err:{}", e),
        }
    }

    pb.finish_with_message("downloaded");

    print!("\n\nExtracting node and npm ...\n\n");

    let reader = std::io::Cursor::new(writer_buf);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut item = archive.by_index(i).unwrap();
        let file_path = item.mangled_name();
        let file_path = file_path.to_string_lossy();

        let mut new_path = install_path.to_owned();
        if let Some(index) = file_path.find('\\') {
            new_path.push(&file_path[index + 1..]);
        }

        if item.is_dir() && !new_path.exists() {
            std::fs::create_dir_all(&new_path)
                .unwrap_or_else(|_| panic!("Could not create new folder: {new_path:?}"));
        }

        if item.is_file() {
            let mut file = std::fs::File::create(&*new_path).unwrap();
            std::io::copy(&mut item, &mut file)
                .unwrap_or_else(|_| panic!("Couldn't write to {new_path:?}"));
        }
    }
}
