
use std::io::prelude::*;
use std::io;

use outparse;


fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let result = outparse::parse_log(io::BufReader::new(handle));
    println!("{}", result);

    for message in &result.messages {
        if let(outparse::Message::Error(inner)) = message {
            println!("{:?}", inner);
        }
    }
}
