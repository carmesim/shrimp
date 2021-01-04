use clap::{App, AppSettings, Arg}; /* For command line arguments */

fn main() {
    let matches = App::new("Shrimp")
        .version("0.1")
        .author("Vini And Merazi")
        .about("Rusty password generator.")
        .settings(&[AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("uppercase")
                .long("--uppercase")
                .short("-u")
                .help("Enable uppercase letters."),
        )
        .arg(
            Arg::with_name("lowercase")
                .long("--nolow")
                .short("-l")
                .help("Enable lowercase letters. (Enabled by default)"),
        )
        .arg(
            Arg::with_name("symbols")
                .long("--symbols")
                .short("-s")
                .help("Enable symbols like #, $ or @."),
        )
        .arg(
            Arg::with_name("numbers")
                .long("--numbers")
                .short("-n")
                .help("Enable whole numbers like 1, 2, or 8."),
        )
        .arg(
            Arg::with_name("language")
                .long("--lang")
                .short("-L")
                .value_name("Language")
                .help("Set a base string to take the characters from."),
        )
        .get_matches();
    println!("Hello, world!");
}
