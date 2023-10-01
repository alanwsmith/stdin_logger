use std::fs::OpenOptions;
use std::io::Write;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("STARTING");
    let mut stdin = io::stdin();
    loop {
        let mut buffer = [0; 1];
        stdin.read_exact(&mut buffer).await?;
        append_file("output.txt", String::from_utf8(vec![buffer[0]]).unwrap());
    }
}

fn append_file(path: &str, text: String) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        if let Err(e) = write!(file, "{}", text) {
            eprintln!("Couldn't append: {}", e);
        }
    }
}
