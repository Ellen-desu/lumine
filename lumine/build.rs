use std::{fs, path::Path};

fn main() {
    let workspace_readme = Path::new("../README.md");
    let local_readme = Path::new("README.md");

    if !local_readme.exists() {
        fs::copy(workspace_readme, local_readme).expect("failed to copy README.md");
    }
}
