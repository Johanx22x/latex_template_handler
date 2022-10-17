use std::io::Write;
use std::path::Path;
use http::StatusCode;

/// Read user input and return the input
///
/// # Examples
/// ```
/// let input = read_input();
/// ```
///
/// # Panics
/// This function will panic if the user input is empty
/// or if the user input is not a valid UTF-8 string
pub fn read_stdin(message: String) -> String {
    // Print the message withouth a new line at the end
    print!("{}: ", message);
    // Flush the stdout buffer
    // This is needed to print the message before the user input
    // without a new line at the end
    // If we don't flush the buffer, the message will be printed
    // after the user input
    std::io::stdout().flush().unwrap();

    // Read the folder name from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    input
}

/// Check the template name according to the available templates 
/// and execute the function associated to the template
///
/// # Arguments
/// * `template` - A `String` containing the template name
/// * `path` - A `String` containing the path to the file or directory
///
/// # Example
/// ```
/// check_template(&self.template, &self.path);
/// ```
///
/// # Panics
/// This function will panic if the template is not valid
pub fn check_template(option: &str, path: &str) {
    // Check if the option is valid
    // If it is, execute the anonymous function
    // If it is not, return an error
    // Check if the option wasn't found
    // If it wasn't, return an error
    match crate::TEMPLATES.iter().find(|(name, _, _)| name == &option) {
        Some((_, _, func)) => {
            println!("\x1b[34mCreating the new template at {}\x1b[0m", path);
            // Execute the function
            func(path);
        },
        None => { 
            eprintln!("\x1b[31mInvalid template name!\x1b[0m\n\
                      Use -l or --list to see the available templates");
            std::process::exit(1);
        },
    }
}

/// Check if path is a directory or a file
/// If it is not a directory, exit the program
/// If it is a directory, do nothing and continue
///
/// # Arguments
/// * `path` - A string slice that holds the path to check 
///
/// # Panics
/// If the path is not a directory, the program will exit 
/// with the error code 1 
///
/// # Examples
/// ```
/// check_path("path/to/file");
/// ```
/// This will exit the program with the error code 1
/// ```
/// check_path("path/to/directory");
/// ```
/// This will not exit the program
pub fn check_path(path: &str) {
    // Check if the path is a directory
    let path = Path::new(path).is_dir();

    // If it is not a directory, exit the program
    if !path {
        eprintln!("{}, is not a directory", path);
        std::process::exit(1);
    }
}

/// Create a new folder and format the path to the new folder
///
/// # Arguments
/// * `path` - The path to the new folder
/// * `name` - The name of the new folder
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// let name = "new_folder";
/// create_folder(path, name);
/// ```
pub fn create_folder(path: &str, name: &str) {
    // Create the new folder
    std::fs::create_dir(format!("{}/{}", path, name)).expect("Failed to create the new folder");
}

/// Create a new folder according to the user input and format the path to the new folder
///
/// # Arguments
/// * `path` - The path to the new folder
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// let new_path = format_path(path);
/// ```
///
/// # Output
/// ```
/// "Enter the name of the new folder: "
/// ```
pub fn create_dir_using_stdin(path: &str) -> String {
    // Read the folder name from stdin
    let name = read_stdin("Enter the name of the new folder".to_string()).trim().to_string();

    // Create the new folder
    std::fs::create_dir(format!("{}/{}", path, name)).expect("Failed to create the new folder");

    // Return the path to the new folder 
    return format!("{}/{}", path, name);
}

/// Create a new file using a given path and name
/// Also, write the content of the file
///
/// # Arguments 
/// * `path` - The path to the new File
/// * `name` - The name of the new File 
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// let name = "new_file";
/// create_file(path, name);
/// ```
///
/// # Panics
/// This function will panic if the file can't be created
/// or if the file can't be written
/// or if the file can't be closed
pub fn create_file(path: &str, name: &str, content: &str) {
    // Create the new file
    let mut file = std::fs::File::create(format!("{}/{}", path, name)).expect("Failed to create the new file");

    // Write the content to the new file
    file.write_all(content.as_bytes()).expect("Failed to write to the new file");

    // Close the file
    file.flush().expect("Failed to close the new file");

    println!("\x1b[34mCreated the new file at {}/{}\x1b[0m", path, name);
}

/// Download a file from a given github raw url 
///
/// # Arguments
/// * `url` - A string slice that holds the url to download the file
///
/// # Example
/// ```
/// get_file("https://raw.githubusercontent.com/username/repo/main/file");
/// ```
/// This will download the file from the given url
/// and return a `String` containing the content of the file 
///
/// # Panics
/// This function will panic if the file can't be downloaded
pub fn get_file(url: &str) -> String {
    // Download the file from the given url
    let response = reqwest::blocking::get(url).expect("Failed to download the file");

    // Check if the response is successful
    // If it is, return the content of the file
    // If it is not, return an error
    match response.status() {
        StatusCode::OK => {
            // Return the content of the file
            response.text().unwrap()
        },
        _ => {
            eprintln!("\x1b[31mFailed to download the file!\x1b[0m");
            std::process::exit(1);
        },
    }
}
