use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_search(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    section: &str,
    key: &str,
    value: &str,
    separator: &str,
) -> JoinHandle<()> {
    let mut str_section = String::new();
    str_section.push('[');
    str_section.push_str(section);
    str_section.push(']');
    let mut str_key = String::from(key);
    str_key.push('=');
    let str_value = String::from(value);
    let str_separator = String::from(separator);
    spawn(move || {
        let mut section_found = false;
        let mut key_maj = false;
        for l in from_read {
            let mut res = String::new();
            if key_maj {
                //already done -> simply pass lines
                res.push_str(&l);
            } else if l.to_uppercase().starts_with(&str_section.to_uppercase()) {
                //we are on line section
                section_found = true;
                //write it directly
                res.push_str(&l);
            } else if section_found {
                //section already pass
                if l.starts_with('[') {
                    //other section
                    //-> add key , then
                    let mut res2 = String::new();
                    res2.push_str(&str_key);
                    res2.push_str(&str_value);
                    if to_write.send(res2).is_err() {
                        println!("error sending to write");
                        return;
                    }
                    key_maj = true;
                    //then write line l
                    res.push_str(&l);
                } else if l.to_uppercase().starts_with(&str_key.to_uppercase()) {
                    //key found in section -> append value

                    res.push_str(&l);
                    res.push_str(&str_separator);
                    res.push_str(&str_value);
                    key_maj = true;
                } else {
                    res.push_str(&l);
                }
            } else {
                res.push_str(&l);
            }
            if to_write.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }
        if section_found {
            if !key_maj {
                //section found but key not updated so section must be at end of file
                //just add key
                let mut res = String::new();
                res.push_str(&str_key);
                res.push_str(&str_value);
                if to_write.send(res).is_err() {
                    println!("error sending to write");
                    return;
                }
            }
        } else {
            //add section and key
            if to_write.send(str_section).is_err() {
                println!("error sending to write");
                return;
            }
            let mut res = String::new();
            res.push_str(&str_key);
            res.push_str(&str_value);
            if to_write.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}
