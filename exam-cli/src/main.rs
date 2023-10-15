use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    match args.count {
        1..=7 => println!("Right"),
        _ => println!("Not matched")
    }
    

    let _t:u16;
    _t = 3;

    let v = vec![2, 3, 5];
    let v2 = v;
    println!("{}", v2[0]);
}
