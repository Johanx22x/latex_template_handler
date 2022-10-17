use std::env;

mod config;
mod templates;
mod utils;
mod git;

use config::*;
use templates::*;

// const map to store the option name an its description
const OPTIONS: &[(&str, &str)] = &[
    ("-h, --help", "Display this help message"),
    ("-l, --list", "Display the available templates"),
    ("-v, --version", "Display the version of the program"),
];

// const map to store the template name, its description and the function to execute
const TEMPLATES: &[(&str, &str, fn(&str))] = &[
    ("math", "Latex report, template focused on math", math), // Based on @gillescastel & @SirCharlieMars
    ("ieee", "Basic IEEE template, using pandoc & markdown", ieee),
    ("apa7tec", "Custom template for TEC papers, using pandoc & markdown", apa7tec), // Provided by
                                                                                    // @zSnails
];

/// Main function
fn main() {
    // Get the arguments
    let args: Vec<String> = env::args().collect();

    // Manage the error here
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Check if the error is help
        // If so, print the help message
        // If is list, print the list of template available
        // Otherwise, print the error message
        if err == "help" {
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
