mod stak;

use stak::Stak;
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "stak")]
/// Command line based RPN calculator
struct Opts {
    #[structopt(name = "TOKEN")]
    /// Numbers and operators
    tokens: Vec<String>,
}

fn main() {
    // Parse command line arguments
    let opts = Opts::from_args();
    if opts.tokens.is_empty() {
        interactive();
    } else {
        oneshot(&opts.tokens)
    }
}

/// Start an interactive shell session
fn interactive() {
    let mut stack = Stak::new();
    loop {
        // Get input from user
        let mut tokens = String::new();
        print!("> ");
        let _ = std::io::stdout().flush();
        if let Err(e) = std::io::stdin().read_line(&mut tokens) {
            println!("Parsing error: {:?}", e);
            continue;
        }

        // Process each token
        for token in tokens.trim().split(' ') {
            if let Err(e) = stack.parse_token(token) {
                println!("Error: {}", e);
                break;
            }
        }

        // Print out the stack
        stack.print_stack()
    }
}

/// Executes the operations passed in as command line arguments, then exits
fn oneshot(tokens: &[String]) {
    let mut stack = Stak::new();
    for token in tokens {
        if let Err(e) = stack.parse_token(token) {
            println!("Error: {}", e);
            exit(1);
        }
    }
    stack.print_stack();
}
