use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};

#[cfg(unix)]
pub static EOL: &str = "\n";

#[cfg(windows)]
pub static EOL: &str = "\r\n";

pub fn start_thread_write(from_search: Receiver<String>, fic_out: &str) -> JoinHandle<()> {
    let name = String::from(fic_out);
    spawn(move || {
        let file = match File::create(name) {
            Err(e) => {
                println!("Error creating file {:?}", e);
                return;
            }
            Ok(fil) => fil,
        };
        let mut writer = BufWriter::new(file);
        for d in from_search {
            let data = format!("{}{}", d, EOL);
            if writer.write_all(data.as_bytes()).is_err() {
                println!("Error writing in file");
            }
        }
    })
}
