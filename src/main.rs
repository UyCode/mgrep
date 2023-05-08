
use std::{env, process};
use mgrep::Config;

fn main() {

    //let args = env::args();
    //dbg!(args);

    let begin_time = std::time::Instant::now();

    let help = env::args().any(|arg| arg == "--help" || arg == "-h" || arg == "help");
    if help {
        println!("Usage: mgrep [OPTIONS] [<query>] <file_path>");
        println!("OPTIONS:");
        println!("\t-i\t\tignore case");
        println!("\t-c\t\tprint total words count in file");
        println!("\t-h\t\tprint this help menu");
        // example
        println!("\nExample:");
        println!("\tmgrep -i \"hello world\" test.txt");
        println!("\tmgrep -c  test.txt");
        process::exit(0);
    }

    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });
    println!("searching {} in {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // estimate time in milliseconds
    println!("spend time: {:?}", begin_time.elapsed());
}

