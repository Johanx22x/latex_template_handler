/// Initialize a new git repository in the given path 
/// and add the new files to the repository
///
/// # Arguments
/// * `path` - The path to the new git repository
///
/// # Example
/// ```
/// let path = "path/to/a/directory";
/// init_git(path);
/// ```
///
/// # Panics
/// This function will panic if the git repository can't be initialized
/// or if the files can't be added to the repository
/// or if the repository can't be commited
pub fn init_git(path: &str) {
    // Initialize a new git repository
    let repo = git2::Repository::init(path).expect("Failed to initialize the git repository");

    // Add all the files to the repository
    let mut index = repo.index().expect("Failed to add the files to the repository");

    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).expect("Failed to add the files to the repository");

    index.write().expect("Failed to add the files to the repository");

    // Commit the files to the repository
    let tree_id = index.write_tree().expect("Failed to commit the files to the repository");

    let tree = repo.find_tree(tree_id).expect("Failed to commit the files to the repository");

    let sig = repo.signature().expect("Failed to commit the files to the repository");

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).expect("Failed to commit the files to the repository");

    println!("\x1b[34mInitialized a new git repository at {}\x1b[0m", path);

    println!("\x1b[34mAdded the files to the repository\x1b[0m");

    println!("\x1b[34mCommitted the files to the repository\x1b[0m");
}
