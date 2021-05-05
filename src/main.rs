use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    set: i32,
    challenge: i32,
}

fn main() {
    let args = Cli::from_args();

    let set = args.set;
    let challenge = args.challenge;

    println!("Running Set {} Challenge {}", set, challenge);

    match set {
        1 => {
            match challenge {
                _ => println!("Nothing available Set Challenge {}", challenge),
            }
        },
        _ => { println!("No challenges available from set {}", set); },
    }
}
