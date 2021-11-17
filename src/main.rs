mod stak;

use stak::Stak;
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;

const LONG_HELP: &str = r#"Operators:
+   add the top two values
-   subtract the top two values
*   multiply the top two values
/   divide the top two values
%   modulo the top two values
**  raise the second value to the top values
||  perform a parallel sum on all values
inv     take 1 divided by the top value
sqrt    perform a square root on the top value
log2    perform a base-2 log on the top value
floor   floor the top value
ceil    ceiling the top value
abs     take the absolute value
sum     add all values together
prod    multiply all values together

Stack Management:
.   drop the top value of the stack
..  clear the stack
~   duplicate the top value
~#  duplicate the value at index # (ex. `~2`)
<>  swap the top two values
<<  rotate the stack to the left
<<# rotate the stack # times to the left (ex. `<<3`)
>>  rotate the stack to the right
>># rotate the stack # times to the right (ex. `>>3`)
"#;

#[derive(StructOpt, Debug)]
#[structopt(name = "stak")]
/// Command line based RPN calculator
///
/// Run with no arguments to enter the interactive shell
struct Opts {
    #[structopt(name = "TOKEN", long_help = LONG_HELP)]
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

        // Check for interactive-only commands
        match tokens.trim() {
            "?" => {
                println!("{}", LONG_HELP);
                continue;
            }
            "help" => {
                println!("{}", LONG_HELP);
                continue;
            }
            "exit" => exit(0),
            "quit" => exit(0),
            "q" => exit(0),
            _ => {}
        }

        // Process each token
        for token in tokens.trim().split(' ') {
            if !token.is_empty() {
                if let Err(e) = stack.parse_token(token) {
                    println!("Error: {}", e);
                    break;
                }
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
        for t in token.trim().split(' ') {
            if let Err(e) = stack.parse_token(t) {
                println!("Error: {}", e);
                exit(1);
            }
        }
    }
    stack.print_stack();
}
