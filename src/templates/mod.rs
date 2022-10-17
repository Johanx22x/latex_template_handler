use crate::git::*;
use crate::file::*;

/// Create a new latex report, template focused on math
/// Based on @gillescastel & @SirCharlieMars templates
///
/// # Arguments
/// * `path` - A string slice that holds the path to the new folder
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// math_template(path);
/// ```
///
/// # Output
/// ```
/// path/to/a/directory
/// ├── .git
/// ├── images
/// │   └── logo.png
/// ├── letterfonts.tex
/// ├── main.tex
/// ├── macros.tex 
/// ├── preamble.tex 
/// ├── README.md
/// └── src
///     └── chap01.tex
/// ```
pub fn math(path: &str) {
    // Create the new folder
    let new_path = create_dir_using_stdin(path);

    // Create .tex files
    create_file(&new_path, "main.tex", "");
    create_file(&new_path, "preamble.tex", "");
    create_file(&new_path, "macros.tex", "");
    create_file(&new_path, "letterfonts.tex", "");

    // Create the images folder
    create_folder(&new_path, "images");
    // Get an image about math from the internet and save it to the images folder 
    download_image("https://upload.wikimedia.org/wikipedia/commons/thumb/2/2a/Logarithmic_spiral.svg/1200px-Logarithmic_spiral.svg.png", &format!("{}/images", new_path));

    // Create the src folder
    create_folder(&new_path, "src");
    create_file(&format!("{}/src", new_path), "chap01.tex", "");

    // Ask the user if he wants to initialize a git repository
    let git = read_stdin("Do you want to initialize a git repository? (Y/n)".to_string()).trim().to_string();

    // Use an array to store the git answers
    let git_answers = ["y", "Y", "yes", "Yes", "YES", ""];

    // Check if the user wants to initialize a git repository
    if git_answers.contains(&git.as_str()) {
        // Initialize a new git repository
        init_git(&new_path);
    }

    println!("\x1b[34mCreated the new folder at {}\x1b[0m", new_path);

    // Use tree -C to print the new folder structure with colors
    let output = std::process::Command::new("tree")
        .arg("-C")
        .arg(new_path)
        .output()
        .expect("Failed to print the new folder structure");
}

pub fn ieee(path: &str) {
    println!("IEEE template at {}", path);
}

pub fn apa7tec(path: &str) {
    println!("APA7TEC template at {}", path);
}
