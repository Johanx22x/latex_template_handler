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
pub fn math(path: &str) {
    // Create the new folder
    let new_path = create_dir_using_stdin(path);

    // Create main.tex file
    // Download the main.tex file from the repo 
    // https://raw.githubusercontent.com/TeXample/templates/master/math/main.tex
    let main = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/main.tex");
    create_file(&new_path, "main.tex", main.as_str());

    // Create lib folder 
    create_folder(&new_path, "lib");

    // Create .text files inside lib folder 

    // Download the preamble.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/preamble.tex
    let preamble = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/preamble.tex");
    create_file(&new_path, "lib/preamble.tex", preamble.as_str());

    // Download the macros.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/macros.tex 
    let macros = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/macros.tex");
    create_file(&new_path, "lib/macros.tex", macros.as_str());

    // Download the letterfonts.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/letterfonts.tex 
    let letterfonts = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/letterfonts.tex");
    create_file(&new_path, "lib/letterfonts.tex", letterfonts.as_str());

    // Create the images folder
    create_folder(&new_path, "images");
    // Get an image about math from the internet and save it to the images folder 
    // download_file("https://upload.wikimedia.org/wikipedia/commons/thumb/2/2a/Logarithmic_spiral.svg/1200px-Logarithmic_spiral.svg.png", &format!("{}/images", new_path));

    // Create the src folder
    create_folder(&new_path, "src");
    create_file(&format!("{}/src", new_path), "chap01.tex", "");

    // Ask the user if he wants to initialize a git repository
    let git = read_stdin("Do you want to initialize a git repository? (Y/n)".to_string()).trim().to_string();

    // Ask the user if he wants to create a README.md file 
    let readme = read_stdin("Do you want to create a README.md file? (Y/n)".to_string()).trim().to_string();

    // Use an array to store the git answers
    let answers = ["y", "Y", "yes", "Yes", "YES", ""];

    // Check if the user wants to create a README.md file 
    if answers.contains(&readme.as_str()) {
        // Create a README.md file 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/README.md 
        let readme = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/README.md");
        create_file(&new_path, "README.md", readme.as_str());
    }

    // Check if the user wants to initialize a git repository
    if answers.contains(&git.as_str()) {
        // Create a .gitignore file 
        // Download the .gitignore file from the repo
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/.gitignore 
        let gitignore = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/.gitignore");
        create_file(&new_path, ".gitignore", gitignore.as_str());

        // Initialize a new git repository
        init_git(&new_path);
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
    // Create the new folder 
    let new_path = create_dir_using_stdin(path);

    // Create the metadata.yaml file
    // Download the metadata.yaml file from the repo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/metadata.yaml
    let main = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/metadata.yaml");
    create_file(&new_path, "metadata.yaml", main.as_str());

    // Create lib folder 
    create_folder(&new_path, "lib");

    // Download the necessary files from the repo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/bibliography.csl
    let bibliography = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/bibliography.csl");
    create_file(&new_path, "lib/bibliography.cls", bibliography.as_str());

    // Download the necessary files from the repo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/bibliography.bib
    let bibliography = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/bibliography.bib");
    create_file(&new_path, "lib/bibliography.bib", bibliography.as_str());

    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/template.tex 
    let template = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/template.tex");
    create_file(&new_path, "lib/template.tex", template.as_str());

    // Create the images folder
    create_folder(&new_path, "images");
    // Get an image from the repo and save it to the images folder 
    let image = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/images/markdown.png");
    create_file(&new_path, "images/markdown.png", image.as_str());

    // Create the src folder
    create_folder(&new_path, "src");
    create_file(&format!("{}/src", new_path), "chap01.tex", "");

    // Download the 01.md file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/01.md
    let first = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/lib/01.md");
    create_file(&new_path, "src/01.md", first.as_str());

    // Download the Makefile
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/Makefile
    let makefile = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/Makefile");
    create_file(&new_path, "Makefile", makefile.as_str());

    // Make a new directory called build 
    create_folder(&new_path, "build");

    // Ask the user if he wants to initialize a git repository
    let git = read_stdin("Do you want to initialize a git repository? (Y/n)".to_string()).trim().to_string();

    // Ask the user if he wants to create a README.md file 
    let readme = read_stdin("Do you want to create a README.md file? (Y/n)".to_string()).trim().to_string();

    // Use an array to store the git answers
    let answers = ["y", "Y", "yes", "Yes", "YES", ""];

    // Check if the user wants to create a README.md file 
    if answers.contains(&readme.as_str()) {
        // Create a README.md file 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/README.md 
        let readme = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/README.md");
        create_file(&new_path, "README.md", readme.as_str());
    }

    // Check if the user wants to initialize a git repository
    if answers.contains(&git.as_str()) {
        // Create a .gitignore file 
        // Download the .gitignore file from the repo
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/.gitignore
        let gitignore = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/.gitignore");
        create_file(&new_path, ".gitignore", gitignore.as_str());

        // Initialize a new git repository
        init_git(&new_path);
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

pub fn apa7tec(path: &str) {
    println!("APA7TEC template at {}", path);
}
