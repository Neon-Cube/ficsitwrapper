use std::process;

use ficsitwrapper::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error Running The Wrapper: {e}");
        process::exit(1);
    }
    println!("Done.");
    process::exit(0);
}
