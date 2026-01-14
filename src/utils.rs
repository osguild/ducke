use std::io::{self};
use std::process::Command;

pub fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn clone_extension_template(proj_name: String) -> Result<(), io::Error> {
    let mut res = Command::new("git")
        .args([
            "clone",
            "--recurse-submodules",
            "git@github.com:duckdb/extension-template.git",
            &proj_name,
        ])
        .spawn()
        .expect("failed to clone project");
    res.wait().expect("project failed to clone");
    let mut path: String = proj_name.clone().to_owned();

    path.push_str("/.git");

    Command::new("rm")
        .args(["-rf", &path])
        .spawn()
        .expect("Could not delete .git directory");

    Ok(())
}
