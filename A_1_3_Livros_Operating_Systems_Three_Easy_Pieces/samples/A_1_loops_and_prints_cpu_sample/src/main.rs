use std::env;
use std::process;
use std::thread;
use std::time::Duration;

fn spin(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("{:?}", args);

    if args.len() != 2 {
        eprintln!("usage: cpu <string>");
        process::exit(1);
    }

    let input_str = &args[1];

    loop {
        spin(1);
        println!("{}", input_str);
    }
}
