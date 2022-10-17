use crate::utils::*;

/// Struct to manage initial configuration of the application
pub struct Config {
    template : String,
    path: String,
}

/// Implementation of the Config struct to manage initial configuration of the application
impl Config {
    /// Create a new Config struct, in case of error return the error
    ///
    /// # Arguments
    /// * `args` - A vector of `String` containing the arguments passed to the program
    ///
    /// # Example
    /// ```
    /// let config = Config::build(&args)
    /// ```
    ///
    /// # Panics
    /// This function will panic if the arguments are not valid
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if the user is asking for the program version
        if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
            show_version();
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
    ///
    /// # Example
    /// ```
    /// config.run();
    /// ```
    pub fn run(&self) {
        // Check if path is a directory or a file
        check_path(&self.path);

        // Check if the option is valid
        check_template(&self.template, &self.path);
    }
}

/// Display help message
///
/// # Examples
/// ```
/// help();
/// ```
///
/// # Output
/// ```text
/// Usage: lth [TEMPLATE] [PATH]
/// Options:
///    -h, --help      Display this help message
///    -l, --list      Display the available templates 
///    -v, --version   Display the version of the program
/// ```
pub fn handle_bad_usage() {
    println!("Usage: lth [TEMPLATE] [PATH]");

    // Display the options in the OPTIONS const map
    println!("Options:");
    for (option, description) in crate::OPTIONS {
        println!("\t{}\t\t{}", option, description);
    }
}

/// Display the templates in the TEMPLATES const map
/// and the description of each template 
///
/// # Examples
/// ```
/// list_templates();
/// ```
///
/// # Output
/// ```
/// Available templates:
///    math        Latex report, template focused on math
///    ieee        Basic IEEE template, using pandoc & markdown
/// ```
pub fn template_list() {
    println!("List of templates:");
    // Display the templates in the TEMPLATES const map
    for (template, description, _) in crate::TEMPLATES {
        println!("\t\x1b[4;34m{}\x1b[0m\t\t{}", template, description);
    }
}

/// Display the version of the program
///
/// # Examples
/// ```
/// show_version();
/// ```
///
/// # Output
/// ```text
/// lth 0.1.0
/// ```
pub fn show_version() {
    println!("Latex Template Handler [lth] by @Johanx22x");
    println!("Version: \x1b[4;34m{}\x1b[0m", env!("CARGO_PKG_VERSION"));
}
