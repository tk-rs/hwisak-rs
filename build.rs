use std::{env, process::Command, path::PathBuf};

fn main() {
    // Get the project directory (CARGO_MANIFEST_DIR is the root of the crate)
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get project directory");

    // Construct the path to the script
    let script_path = PathBuf::from(&project_dir).join("scripts");

    // Platform-specific script execution
    #[cfg(target_os = "windows")]
    {
        let script_file = script_path.join("build.bat");
        println!("Running script: {:?}", script_file);

        let status = Command::new("cmd")
            .args(["/C", script_file.to_str().unwrap()])
            .status()
            .expect("Failed to execute .bat script");

        if !status.success() {
            panic!("Script execution failed!");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let script_file = script_path.join("build.sh");
        println!("Running script: {:?}", script_file);

        let status = Command::new("sh")
            .arg(script_file.to_str().unwrap())
            .status()
            .expect("Failed to execute .sh script");

        if !status.success() {
            panic!("Script execution failed!");
        }
    }
}
