use std::path::Path;

// Create a const map to store the option name an its description
const OPTIONS: &[(&str, &str)] = &[
    ("-h, --help", "Display this help message"),
    ("-l, --list", "Display the available templates"),
    ("-v, --version", "Display the version of the program"),
];

// Create a const map to store the template name, its description and the anonymous function to execute
const TEMPLATES: &[(&str, &str, fn())] = &[
    ("math", "Latex report, template focused on math", math), // Based on @gillescastel & @SirCharlieMars
    ("ieee", "Basic IEEE template, using pandoc & markdown", ieee),
    ("apa7tec", "Custom template for TEC papers, using pandoc & markdown", apa7tec), // Provided by
                                                                                    // @zSnails
];

/// Struct to manage initial configuration of the application
pub struct Config {
    template : String,
    path: String,
}

/// Implementation of the Config struct to manage initial configuration of the application
impl Config {
    /// Create a new Config struct, in case of error return the error
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if the user is asking for the program version
        if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
            println!("Latex Template Handler by @Johanx22x");
            println!("Version: \x1b[4;34m{}\x1b[0m", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        // Check if the user is asking for the program help
        if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
            return Err("help");
        }

        // Check if the user is asking for the available templates
        if args.contains(&String::from("-l")) || args.contains(&String::from("--list")) {
            return Err("list");
        }

        // Check if the user do a correct call to the program
        if args.len() != 3 {
            return Err("bad usage");
        }

        let template = args[1].clone();
        let path = args[2].clone();

        Ok(Config { template , path })
    }

    /// Run the application
    pub fn run(&self) {
        // Check if path is a directory or a file
        check_path(&self.path);

        // Check if the option is valid
        check_template(&self.template);
    }
}

/// Check the template name according to the available templates 
fn check_template(option: &str) {
    // Check if the option is valid
    // If it is, execute the anonymous function
    // If it is not, return an error
    // Check if the option wasn't found
    // If it wasn't, return an error
    match TEMPLATES.iter().find(|(name, _, _)| name == &option) {
        Some((_, _, func)) => {
            println!("\x1b[34mCreating the file...\x1b[0m");
            func();
        },
        None => { 
            eprintln!("\x1b[31mInvalid template name!\x1b[0m\n\
                      Use -l or --list to see the available templates");
            std::process::exit(1);
        },
    }
}

/// Check if path is a directory or a file
fn check_path(path: &str) {
    let path = Path::new(path).is_dir();

    if !path {
        eprintln!("{}, is not a directory", path);
        std::process::exit(1);
    }
}

/// Display help message
pub fn handle_bad_usage() {
    println!("Usage: latex-template-handler [TEMPLATE] [PATH]");
    // Display the options in the OPTIONS const map
    println!("Options:");
    for (option, description) in OPTIONS {
        println!("\t{}\t\t{}", option, description);
    }
}

/// Display available templates
pub fn template_list() {
    println!("List of templates:");
    // Display the templates in the TEMPLATES const map
    for (template, description, _) in TEMPLATES {
        println!("\t\x1b[4;34m{}\x1b[0m\t\t{}", template, description);
    }
}

fn math() {
    println!("Math template");
}

fn ieee() {
    println!("IEEE template");
}

fn apa7tec() {
    println!("APA7TEC template");
}
