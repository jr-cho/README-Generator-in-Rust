use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("Enter a Title for your project:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter a brief description for your project:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    println!("Enter the main usage instructions:");
    let mut usage = String::new();
    io::stdin()
        .read_line(&mut usage)
        .expect("Failed to read line");

    println!("Enter installation instructions:");
    let mut installation = String::new();
    io::stdin()
        .read_line(&mut installation)
        .expect("Failed to read line");

    println!("Enter your github username:");
    let mut more = String::new();
    io::stdin()
        .read_line(&mut more)
        .expect("Failed to read line");

    let readme_content = format!(
        "# {}\n\n## Description\n{}\n\n## Installation\n```\n{}\n```\n\n## Usage\n```\n{}\n```\n\n## More\nYou can find more of my work at [{}](https://github.com/{})",
        title.trim(),
        description.trim(),
        installation.trim(),
        usage.trim(),
        more.trim(),
        more.trim()
    );

    let mut file = File::create("README.md").expect("Could not create README.md file");
    file.write_all(readme_content.as_bytes())
        .expect("Failed to write to README.md");

    println!("README.md has been generated successfully!");
}
