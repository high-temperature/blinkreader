use std::fs::File;
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let path = "C:\\Users\\kyoch\\Downloads\\令和3章1節_生産性の動向と課題.txt"; // ファイルのパスを指定
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(ip) = line {
            for chunk in ip.as_bytes().chunks(30) {
                let display_string = String::from_utf8_lossy(chunk);
                println!("{}", display_string);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    Ok(())
}

