use std::env;

use latex_template_handler::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Manage the error here
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Check if the error is help
        // If so, print the help message
        // If is list, print the list of template available
        // Otherwise, print the error message
        if err == "help" {
            eprintln!("Help message");
            handle_bad_usage();
            std::process::exit(0);
        } else if err == "list" {
            template_list();
            std::process::exit(0);
        } else {
            eprintln!("Problem parsing arguments: \x1b[31m{}\x1b[0m", err);
            handle_bad_usage();
            std::process::exit(1);
        }
    });

    config.run();
}
