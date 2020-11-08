use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub fic: String,
    pub section: String,
    pub key: String,
    pub value: String,
    pub keep_old: bool,
    pub separator: String,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut section = String::new();
        let mut key = String::new();
        let mut value = String::new();
        let mut keep_old = false;
        let mut separator = String::new();

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("appendini"));
        println!("{} 1.0 (2020)", name);
        if args.is_empty() {
            help(&name);
        }
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if let Some(n) = get_param(&arg, String::from("/fic:")) {
                fic = n;
                continue;
            }
            if let Some(n) = get_param(&arg, String::from("/section:")) {
                section = n;
                continue;
            }
            if let Some(n) = get_param(&arg, String::from("/key:")) {
                key = n;
                continue;
            }
            if let Some(n) = get_param(&arg, String::from("/value:")) {
                value = n;
                continue;
            }
            if get_param(&arg, String::from("/keep_old")).is_some() {
                keep_old = true;
                continue;
            }
            if let Some(n) = get_param(&arg, String::from("/separator:")) {
                separator = n;
                continue;
            }
        }
        //checks
        if fic.is_empty() {
            println!("ERROR! no file to work with!");
            println!("--------------------------------------------------");
            help(&name);
        }
        if section.is_empty() {
            println!("ERROR! section is missing!");
            println!("--------------------------------------------------");
            help(&name);
        }
        if key.is_empty() {
            println!("ERROR! key is missing!");
            println!("--------------------------------------------------");
            help(&name);
        }
        //check if file exists
        if File::open(&fic).is_err() {
            println!("Error file {} doesn't exists or unereadable", &fic);
            help(&name);
        };
        Paramcli {
            fic,
            section,
            key,
            value,
            keep_old,
            separator,
        }
    }
}

fn get_param(arg: &str, switch: String) -> Option<String> {
    if arg.to_lowercase().starts_with(&switch) {
        let mut a = String::from(arg);
        return Some(a.split_off(switch.len()));
    }
    None
}

fn help(name: &str) {
    println!(
        "syntax : {} /fic:file /section:SSSS /key:KKKK [/value:VVVV] [/keep_old] [/separator:P]",
        name
    );
    println!("paramerters between [] are optionnals");
    println!("------------------------------------");
    println!("fic: ini file");
    println!("SSSS: Section in ini file where put the key");
    println!("KKKK: Key to add/update");
    println!("VVVV: value to put. If /value parameter is not set key will be removed");
    println!("/keep_old: do a .old copy of original file");
    println!("P: if key already exist, put P between existing value and VVVV");
    println!("Section and key searched without cases sensitivity");
    std::process::exit(0);
}
