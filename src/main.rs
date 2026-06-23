mod tasks;
mod cli;

fn main() {
    let test = cli::read_args();
    println!("{:?}",test);
}
