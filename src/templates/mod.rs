use crate::git::*;
use crate::utils::*;

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

    // Create main.tex file
    create_file(&new_path, "main.tex", "");

    // Create lib folder 
    create_folder(&new_path, "lib");

    // Create .text files inside lib folder 
    create_file(&new_path, "lib/macros.tex", "");
    create_file(&new_path, "lib/preamble.tex", "");
    create_file(&new_path, "lib/letterfonts.tex", "");

    // Create the images folder
    create_folder(&new_path, "images");
    // Get an image about math from the internet and save it to the images folder 
    // download_file("https://upload.wikimedia.org/wikipedia/commons/thumb/2/2a/Logarithmic_spiral.svg/1200px-Logarithmic_spiral.svg.png", &format!("{}/images", new_path));

    // Create the src folder
    create_folder(&new_path, "src");
    create_file(&format!("{}/src", new_path), "chap01.tex", "");

    // Ask the user if he wants to initialize a git repository
    let git = read_stdin("Do you want to initialize a git repository? (Y/n)".to_string()).trim().to_string();

    // Use an array to store the git answers
    let answers = ["y", "Y", "yes", "Yes", "YES", ""];

    // Check if the user wants to initialize a git repository
    if answers.contains(&git.as_str()) {
        // Initialize a new git repository
        init_git(&new_path);
    }

    // Ask the user if he wants to create a README.md file 
    let readme = read_stdin("Do you want to create a README.md file? (Y/n)".to_string()).trim().to_string();

    // Check if the user wants to create a README.md file 
    if answers.contains(&readme.as_str()) {
        // Create a README.md file 
        create_file(&new_path, "README.md", "# Math Template

This is a template oriented for a math report, based on [@gillescastel](https://github.com/gillescastel/) & [@SirCharlieMars](https://github.com/SirCharlieMars) templates. 

Also can be used for other reports, just change the `main.tex` file and the `lib` folder to customize it.

## Dependencies

A LateX compiler like [vimtex](https://github.com/lervag/vimtex) or [latexmk](https://www.ctan.org/pkg/latexmk/) is required to compile this template.

## How to use it 

1. Configure the `main.tex` file to your needs.
2. Create and manage your chapters in the `src` folder.
3. Add your images and figures in the `images` folder.
4. Customize the `lib` folder to your more specific needs.
5. Enjoy your report.

## Contributing 

If you want to contribute to this template or the Latex Template Handler project, please visit the [GitHub repository](https://github.com/Johanx22x/latex_template_handler).
");
    }

    println!("\x1b[34mCreated the new folder at {}\x1b[0m", new_path);

    // Use tree -C to print the new folder structure with colors
    let output = std::process::Command::new("tree")
        .arg("-C")
        .arg(&new_path)
        .output()
        .expect("Failed to execute command");
    
    // Print the new folder structure
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn ieee(path: &str) {
    println!("IEEE template at {}", path);
}

pub fn apa7tec(path: &str) {
    println!("APA7TEC template at {}", path);
}
