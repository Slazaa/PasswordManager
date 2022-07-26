use std::env;
use std::usize;

fn main() {
    const CHARS: &str = "\
        abcdefghijklmnopqrstuvwxyz\
        ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        0123456789\
        ~!@#$%^&*_+-=?.\
    ";

    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    if args.is_empty() {
        println!("You need to provide a key");
        return;
    }

    if args.len() > 1 {
        println!("Too many arguments were given");
        return;
    }

    let hash = blake3::hash(args[0].as_bytes());
    let hash = format!("{}", hash);
    let subs = hash.as_bytes()
        .chunks(4)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let mut password = String::new();

    for sub in subs.iter() {
        let sub = usize::from_str_radix(sub, 16).unwrap();
        password.push(CHARS.chars().nth(sub % CHARS.chars().count()).unwrap());
    }

    println!("{}", password);
}
