use crate::git::*;
use crate::utils::*;

// Use a const array to store the git answers
const GIT_ANSWERS: [&str; 6] = [
    "Y",
    "y",
    "Yes",
    "YES",
    "yes",
    "",
];


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

    // Check if the user wants to create a README.md file 
    if GIT_ANSWERS.contains(&readme.as_str()) {
        // Create a README.md file 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/README.md 
        let readme = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/math/README.md");
        create_file(&new_path, "README.md", readme.as_str());
    }

    // Check if the user wants to initialize a git repository
    if GIT_ANSWERS.contains(&git.as_str()) {
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

/// Create a new latex report, IEEE template 
///
/// # Arguments
/// * `path` - A string slice that holds the path to the new folder
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// ieee_template(path);
/// ```
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
    create_file(&new_path, "lib/bibliography.csl", bibliography.as_str());

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
    get_image("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/images/markdown.png", &format!("{}/images", new_path), "markdown.png");

    // Create the src folder
    create_folder(&new_path, "src");

    // Download the 01.md file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/src/01.md
    let first = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/src/01.md");
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

    // Check if the user wants to create a README.md file 
    if GIT_ANSWERS.contains(&readme.as_str()) {
        // Create a README.md file 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/README.md 
        let readme = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieee/README.md");
        create_file(&new_path, "README.md", readme.as_str());
    }

    // Check if the user wants to initialize a git repository
    if GIT_ANSWERS.contains(&git.as_str()) {
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

/// Create a new latex project, APA template 
///
/// # Arguments
/// * `path` - A string slice that holds the path to the new folder 
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// apa_template(path);
/// ```
///
/// # Panics
/// This function panics if the path is not valid
pub fn apa7tec(path: &str) {
    // Create the new folder 
    let new_path = create_dir_using_stdin(path);

    // Create the metadata.yaml file
    // Download the metadata.yaml file from the repo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/metadata.yaml
    let main = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/metadata.yaml");
    create_file(&new_path, "metadata.yaml", main.as_str());

    // Create lib folder 
    create_folder(&new_path, "lib");

    // Download the necessary files from the repo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/lib/bibliography.bib
    let bibliography = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/lib/bibliography.bib");
    create_file(&new_path, "lib/bibliography.bib", bibliography.as_str());

    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/lib/template.tex 
    let template = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/lib/apa7tec.cls");
    create_file(&new_path, "lib/apa7tec.cls", template.as_str());

    // Create the src folder
    create_folder(&new_path, "src");

    // Download the 01.md file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/src/01.md 
    let first = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/src/01.md");
    create_file(&new_path, "src/01.md", first.as_str());

    // Download the Makefile 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/Makefile 
    let makefile = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/Makefile"); 
    create_file(&new_path, "Makefile", makefile.as_str()); 

    // Create images folder 
    create_folder(&new_path, "images");

    // Download the logo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/images/logo.png
    get_image("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/images/logo.png", &format!("{}/images", new_path), "logo.png");

    // Make a new directory called build 
    create_folder(&new_path, "build"); 

    // Ask the user if he wants to initialize a git repository
    let git = read_stdin("Do you want to initialize a git repository? (Y/n)".to_string()).trim().to_string(); 

    // Ask the user if he wants to create a README.md file 
    let readme = read_stdin("Do you want to create a README.md file? (Y/n)".to_string()).trim().to_string(); 

    // Check if the user wants to create a README.md file 
    if GIT_ANSWERS.contains(&readme.as_str()) {
        // Create a README.md file 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/README.md 
        let readme = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/README.md"); 
        create_file(&new_path, "README.md", readme.as_str()); 
    }

    // Check if the user wants to initialize a git repository 
    if GIT_ANSWERS.contains(&git.as_str()) {
        // Create a .gitignore file 
        // Download the .gitignore file from the repo 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/.gitignore 
        let gitignore = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/apa7tec/.gitignore"); 
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

/// Create a new latex project, IEEE template for TEC 
///
/// # Arguments
/// * `path` - A string slice that holds the path to the new folder 
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// ieee_tec(path);
/// ```
///
/// # Note
/// This template is based on the IEEEtran.cls class file 
/// https://www.ctan.org/pkg/ieeetran 
pub fn ieee_tec(path: &str) {
    // Create the new folder 
    let new_path = create_dir_using_stdin(path);

    // Download the main.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/main.tex
    let main = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/main.tex");
    create_file(&new_path, "main.tex", main.as_str());

    // Create the lib folder 
    create_folder(&new_path, "lib"); 

    // Create the src folder 
    create_folder(&new_path, "src");

    // Download the IEEEtran.cls file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/lib/IEEEtran.cls 
    let template = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/lib/IEEEtran.cls");
    create_file(&new_path, "lib/IEEEtran.cls", template.as_str());

    // Download the preamble.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/lib/preamble.tex 
    let preamble = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/lib/preamble.tex");
    create_file(&new_path, "lib/preamble.tex", preamble.as_str());

    // Download the header.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/lib/header.tex 
    let header = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/lib/header.tex");
    create_file(&new_path, "lib/header.tex", header.as_str());

    // Download the 01.md file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/src/01.md 
    let first = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/src/01.tex");
    create_file(&new_path, "src/01.tex", first.as_str());

    // Download the abstract.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/src/abstract.tex 
    let abstract_ = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/src/abstract.tex");
    create_file(&new_path, "src/abstract.tex", abstract_.as_str());

    // Download the title.tex file 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/src/title.tex 
    let title = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/src/title.tex");
    create_file(&new_path, "src/title.tex", title.as_str());

    // Create figs folder 
    create_folder(&new_path, "figs"); 

    // Download the logo 
    // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/figs/logo.png
    get_image("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/figs/logo.png", &format!("{}/figs", new_path), "logo.png");

    // Create the build folder
    create_folder(&new_path, "build");

    // Ask the user if he wants to initialize a git repository
    let git = read_stdin("Do you want to initialize a git repository? (Y/n)".to_string()).trim().to_string();

    // Ask the user if he wants to create a README.md file 
    let readme = read_stdin("Do you want to create a README.md file? (Y/n)".to_string()).trim().to_string();

    // Check if the user wants to create a README.md file 
    if GIT_ANSWERS.contains(&readme.as_str()) {
        // Create a README.md file 
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/README.md 
        let readme = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/README.md");
        create_file(&new_path, "README.md", readme.as_str());
    }

    // Check if the user wants to initialize a git repository
    if GIT_ANSWERS.contains(&git.as_str()) {
        // Create a .gitignore file 
        // Download the .gitignore file from the repo
        // https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/.gitignore
        let gitignore = get_file("https://raw.githubusercontent.com/Johanx22x/latex-templates/main/ieeetec/.gitignore");
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
