use rand::{self, Rng};
use shannon_entropy::shannon_entropy;
use std::fmt;
use clap::{App, AppSettings, Arg}; /* For command line arguments */

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz";
const PASSWORD_LEN: usize = 20;

#[derive(Eq, PartialEq)]
enum PasswordQuality {
    Bad,
    Weak,
    Good,
    Excellent,
}

impl fmt::Display for PasswordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            PasswordQuality::Bad => "Bad",
            PasswordQuality::Weak => "Weak",
            PasswordQuality::Good => "Good",
            PasswordQuality::Excellent => "Excellent"
        };
        write!(f, "{}", txt)
    }
}

// I'm guesstimating here
fn password_quality(shannon_entropy: f32) -> PasswordQuality {
    let x = shannon_entropy;
    match x  {
        _ if x <= 2.5 => PasswordQuality::Bad,
        _ if x <= 3.3 => PasswordQuality::Weak,
        _ if x <= 3.7 => PasswordQuality::Good,
        _             => PasswordQuality::Excellent,
    }
}


fn main() {
    let _matches = App::new("Shrimp")
        .version("0.1")
        .author("github.com/carmesim/shrimp")
        .about("Simple password generator in Rust.")
        .settings(&[AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("uppercase")
                .long("--uppercase")
                .short("-u")
                .help("Enable uppercase letters."),
        )
        .arg(
            Arg::with_name("lowercase")
                .long("--lowercase")
                .short("-l")
                .help("Enable lowercase letters."),
        )
        .arg(
            Arg::with_name("symbols")
                .long("--symbols")
                .short("-s")
                .help("Enable symbols like #, $ or @."),
        )
        .arg(
            Arg::with_name("silence")
                .long("--silence")
                .short("-S")
                .help("Suppress all output bar the generated password."),
        )
        .arg(
            Arg::with_name("numbers")
                .long("--numbers")
                .short("-n")
                .help("Enable whole numbers like 1, 2, or 8."),
        )
        .arg(
            Arg::with_name("pool")
                .long("--pool")
                .short("-p")
                .value_name("POOL")
                .help("Sets the pool of characters from which the password can be made of."),
        )
        .get_matches();

        let mut rng = rand::thread_rng();

        let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

        let pw_quality = password_quality(shannon_entropy(&password));

        println!("Password: {}\nQuality:{}", password, pw_quality);
        
}
