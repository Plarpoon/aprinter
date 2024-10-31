pub const VERSION: &str = "v1.0.1";
pub const GITHUB_LINK: &str = "https://github.com/plarpoon/aprinter";
pub const CHANGELOG: &str = "Added version parameter";

pub fn print_version_info() {
    println!("Version: {}", VERSION);
    println!("GitHub Repository: {}", GITHUB_LINK);
    println!("Changelog: {}", CHANGELOG);
}
