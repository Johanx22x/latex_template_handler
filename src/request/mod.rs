use std::io::Write;

/// Download a template from a given raw url of a github repository
///
/// # Arguments
/// * `url` - The url to the template 
///
/// # Example
/// ```
/// let url = "github.com/Johanx22x/latex-template/math";
/// ```
/// 
/// # Panics
/// This function will panic if the template can't be downloaded
///
/// # Output 
/// ```
/// "Template downloaded"
/// ```
fn download_template(url: &str, path: &str) {
    // Print the message
    println!("\x1b[34mDownloading the template...\x1b[0m");

    // Download the template
    let mut response = reqwest::blocking::get(url).expect("Failed to download the template");

    // Create a new file
    let mut file = std::fs::File::create(path).expect("Failed to create the new file");

    // Write the content of the template to the new file
    response.copy_to(&mut file).expect("Failed to write the content of the template to the new file");

    // Close the file
    file.flush().expect("Failed to close the new file");

    // Print the message
    println!("\x1b[34mTemplate downloaded\x1b[0m");
}

/// Download an image from a given url
/// and save it to a given path
///
/// # Arguments
/// * `url` - The url to the image
/// * `path` - The path to the new image
/// 
/// # Example
/// ```
/// let url = "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
/// let path = "path/to/a/directory";
/// download_image(url, path);
/// ```
///
/// # Panics
/// This function will panic if the image can't be downloaded
/// or if the image can't be saved
/// or if the image can't be closed
fn download_image(url: &str, path: &str) {
    // Download the image
    let mut response = reqwest::blocking::get(url).expect("Failed to download the image");

    // Create a new file
    let mut file = std::fs::File::create(path).expect("Failed to create the new file");

    // Write the content of the image to the new file
    response.copy_to(&mut file).expect("Failed to write the content of the image to the new file");

    // Close the file
    file.flush().expect("Failed to close the new file");

    println!("\x1b[34mDownloaded the image to {}\x1b[0m", path);
}
