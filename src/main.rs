
use std::{env, process};
use mgrep::Config;

fn main() {

    //let args = env::args();
    //dbg!(args);

    let begin_time = std::time::Instant::now();

    let help = env::args().any(|arg| arg == "--help" || arg == "-h");
    let help_message ="Usage: mgrep [OPTIONS] [<query>] <file_path> \
                        \nOPTIONS: \
                        \n\t-i\t\tignore case \
                        \n\t-c\t\tprint total words count in file \
                        \n\t-h\t\tprint this help menu \
                        \nExample: \
                        \n\tmgrep \"hello world\" test.txt \
                        \n\tmgrep -i \"hello world\" test.txt \
                        \n\tmgrep -c  test.txt";
    if help {
        println!("{}", help_message);
        process::exit(0);
    }

    let args: Vec<String> = env::args().collect();
    if args.contains(&"-i".to_string()) && args.contains(&"-c".to_string())
        || args.contains(&"-c".to_string()) && args.contains(&"-i".to_string())
    {
        println!("{}", help_message);
        eprintln!("Error: -i and -c can't be used together");
        process::exit(1);
    }

    let config = Config::new(env::args()).unwrap_or_else(|e| {
        println!("{}", help_message);
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });
    println!("searching {} in {}", config.query, config.file_path);

    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // estimate time in milliseconds
    println!("spend time: {:?}", begin_time.elapsed());
}

