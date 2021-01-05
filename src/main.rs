use rand::{self, Rng};
use shannon_entropy::shannon_entropy;
use std::fmt;
use clap::{App, AppSettings, Arg}; /* For command line arguments */

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789#@!$&*";

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
impl From<f32> for PasswordQuality {
    fn from(shannon_entropy: f32) -> Self {
        let x = shannon_entropy;
        match x  {
            _ if x <= 2.5 => PasswordQuality::Bad,
            _ if x <= 3.3 => PasswordQuality::Weak,
            _ if x <= 3.7 => PasswordQuality::Good,
            _             => PasswordQuality::Excellent,
        }    
    }
}

fn get_password(charset: &[u8], charset_length: usize, pw_length: u32) -> String {
    let mut rng = rand::thread_rng();

    (0..pw_length)
    .map(|_| {
        let idx = rng.gen_range(0..charset_length);
        charset[idx] as char
    })
    .collect()
}


fn main() {
    let matches = App::new("Shrimp")
        .version("0.1")
        .author("github.com/carmesim/shrimp")
        .about("Simple password generator in Rust.")
        .after_help("If no pool information is supplied, ")
        .settings(&[AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("uppercase")
                .long("--uppercase")
                .short("-u")
                .conflicts_with("pool")
                .help("Enable uppercase letters."),
        )
        .arg(
            Arg::with_name("lowercase")
                .long("--lowercase")
                .short("L")
                .conflicts_with("pool")
                .help("Enable lowercase letters."),
        )
        .arg(
            Arg::with_name("symbols")
                .long("--symbols")
                .short("s")
                .conflicts_with("pool")
                .help("Enable symbols like #, $ or @."),
        )
        .arg(
            Arg::with_name("silence")
                .long("--silence")
                .short("S")
                .help("Suppress all output bar the generated password."),
        )
        .arg(
            Arg::with_name("verbose")
                .long("--verbose")
                .short("v")
                .conflicts_with("silence")
                .multiple(true)
                .help("Activates verbose output. Can be set multiple times for greater info."),
        )
        .arg(
            Arg::with_name("numbers")
                .long("--numbers")
                .short("n")
                .conflicts_with("pool")
                .help("Enable whole numbers like 1, 2, or 8."),
        )
        .arg(
            Arg::with_name("pool")
                .long("--pool")
                .short("p")
                .value_name("POOL")
                .help("Sets the pool of characters from which the password can be made of."),
        ).arg(
            Arg::with_name("length")
                .long("--length")
                .short("l")
                .value_name("LENGTH")
                .help("Sets generated password's length. If not set, 15 will be used."),
        )
        .get_matches();


        let length: u32 = if matches.is_present("length") {
            matches.value_of("length")
                .unwrap()
                .parse()
                .unwrap()
        } else {
            15
        };

        let use_lowercase = matches.is_present("lowercase");
        let use_uppercase = matches.is_present("uppercase");
        let use_symbols   = matches.is_present("symbols");
        let use_numbers   = matches.is_present("numbers");
        let silent_output = matches.is_present("silence");
        let verbosity_level    = matches.occurrences_of("verbose");

        let password = if use_lowercase || use_uppercase || use_symbols || use_numbers 
        {
            // There's probably a better way to do this but I don't really want to think right now
            let mut charset: String = "".into();
            if use_uppercase {
                charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }
            if use_lowercase {
                charset.push_str("abcdefghijklmnopqrstuvwxyz");
            }
            if use_numbers {
                charset.push_str("0123456789");
            }
            if use_symbols {
                charset.push_str("#@!$&*");
            }
            get_password(&charset.as_bytes(), charset.len(), length)    
        } else {
            if matches.is_present("pool") {
                let pool = matches.value_of("pool").unwrap();
                let pool = pool.as_bytes();
                get_password(pool, pool.len(), length)    
            } else {
                get_password(CHARSET, CHARSET.len(), length)
            }
        };

        if silent_output {
            println!("{}", password);
            return;
        }

        let entropy = shannon_entropy(&password);
        let pw_quality = PasswordQuality::from(entropy);

        if verbosity_level > 0 {
            if verbosity_level > 1 {
                println!("Command-line argument information: {:?}", matches.args);
            }
            println!("Shannon entropy: {}", entropy);
        }

        println!("Password: {}\nQuality: {}", password, pw_quality);
        
}
