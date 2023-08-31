use std::fs;

fn main() {
    let root_dir = "."; // root directory
    let mut table =
        String::from("| No. | Title | Public/Private | Link |\n| --- | ----- | --- | ---- |\n"); // markdown table header

    let mut count = 1;
    let exclude_dirs = vec!["src", "target", ".git"]; // exclude directories

    for entry in fs::read_dir(root_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() && !exclude_dirs.contains(&path.file_name().unwrap().to_str().unwrap()) {
            // exclude directories in exclude_dirs
            let title = path.file_name().unwrap().to_str().unwrap();
            let link = format!("[Link]({})", path.display());
            table.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                count, title, link, link
            ));
            count += 1;
        }
    }

    // write markdown table to README.md
    let readme_path = format!("{}/README.md", root_dir);
    let readme_content = fs::read_to_string(&readme_path).unwrap();
    let new_readme_content =
        readme_content.replace("## Projects", &format!("## Projects\n{}", table));
    fs::write(&readme_path, new_readme_content).unwrap();
}
