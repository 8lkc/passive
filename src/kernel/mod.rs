use std::env;

pub struct Manual;

impl Manual {
    pub fn launch() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
            Self::show_help();
            return;
        }
        println!("(i)   Run with --help to see available options.");
    }

    fn show_help() {
        println!("\nWelcome to passive v1.0.0");
        println!("\nOPTIONS:");
        println!("    -fn\tSearch with full-name");
        println!("    -ip\tSearch with ip address");
        println!("    -u\tSearch with username\n");
    }
}
