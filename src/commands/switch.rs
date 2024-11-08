use crate::utils::yaml_layouts::InitLayout;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

pub fn switch(branch_name: &String) {
    let vault_folder: &Path = Path::new(".vault");
    let init_file: PathBuf = vault_folder.join("init.yaml");
    let content_bytes: Vec<u8> = fs::read(&init_file).unwrap();
    let content: Cow<'_, str> = String::from_utf8_lossy(&content_bytes);
    let mut init_content: InitLayout = serde_yaml::from_str(&content).unwrap();
    if init_content.branches.contains(branch_name) {
        init_content.current_branch = branch_name.to_string();
        println!("Branch switched to: {}", branch_name);
        let yaml_string = serde_yaml::to_string(&init_content).unwrap();
        fs::write(init_file, yaml_string).unwrap();
    } else {
        println!(
            "Invalid branch name: {} Please check for spell error",
            branch_name
        );
    };
}
