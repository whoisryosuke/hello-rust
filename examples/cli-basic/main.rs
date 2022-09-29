use std::env::{args, Args};

fn main() {
    let args: Args = args();

    // This would print "example"
    for flag in args {
        println!("{:?}", flag);
    }
}
