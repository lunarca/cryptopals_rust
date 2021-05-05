use structopt::StructOpt;

pub mod challenges;
pub mod utils;

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
                1 => challenges::stage1::challenge1::run(),
                2 => challenges::stage1::challenge2::run(),
                _ => println!("Nothing available Set Challenge {}", challenge),
            }
        },
        _ => { println!("No challenges available from set {}", set); },
    }
}
