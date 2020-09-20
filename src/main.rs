mod paramcli;
mod read;
mod replace;
mod write;

use paramcli::*;
use read::*;
use replace::*;
use std::fs::*;
use std::sync::mpsc::channel;
use write::*;

pub fn traitement(p: &Paramcli) {
    let mut fic_out = String::from(&p.fic);
    fic_out.push_str(".chg");

    //MPSC chanels
    let (to_search, from_read) = channel();
    let (to_write, from_search) = channel();

    let hread = start_thread_read(to_search, &p.fic);
    let hsearch = start_thread_search(
        from_read,
        to_write,
        &p.section,
        &p.key,
        &p.value,
        &p.separator,
    );
    let hwrite = start_thread_write(from_search, &fic_out);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if hsearch.join().is_err() {
        println!("Thread search finished with error");
    }
    if hwrite.join().is_err() {
        println!("Thread write finished with error");
    }

    if p.keep_old {
        let mut fic_old = String::from(&p.fic);
        fic_old.push_str(".old");
        if rename(&p.fic, &fic_old).is_err() {
            println!("error renaming {} to {} aborting", &p.fic, fic_old);
            return;
        }
    } else if remove_file(&p.fic).is_err() {
        println!("error removing {} aborting", &p.fic);
        return;
    }
    if rename(&fic_out, &p.fic).is_err() {
        println!("error renaming {} to {} aborting", fic_out, p.fic);
        return;
    }
}

fn main() {
    println!("appendini 1.0 (2020)");
    let param = Paramcli::new();
    traitement(&param);
}
