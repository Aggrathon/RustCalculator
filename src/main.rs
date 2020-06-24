use std::env;

fn main() {
    let combine: String = combine_args();
    if combine == "" {
        print_help();
    } else {
        calc::calculate_print(&combine);
    }
}

fn combine_args() -> String {
    let args: Vec<String> = env::args().collect();
    return args[1..].join(" ");
}

fn print_help() {
    let exe = env::args().next().unwrap();
    println!("Commandline calculator written in Rust.");
    println!("");
    println!("Try running it with something to calculate!");
    println!("Example:    {} 2 + 2 - cos pi", exe);
    println!("");
    println!("For more information see: https://github.com/Aggrathon/RustCalculator");
}
