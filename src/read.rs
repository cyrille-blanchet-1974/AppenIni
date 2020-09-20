use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_read(to_search: Sender<String>, fic: &str) -> JoinHandle<()> {
    let file = String::from(fic);
    spawn(move || {
        let input = File::open(&file);
        match input {
            Err(e) => {
                println!("Error reading file {} => {}", &file, e);
            }
            Ok(f) => {
                let buffered = BufReader::new(f);
                for line in buffered.lines() {
                    if let Ok(l) = line {
                        if to_search.send(l).is_err() {
                            println!("error sending to search");
                            return;
                        }
                    }
                }
            }
        }
    })
}
