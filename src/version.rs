pub const VERSION: &str = "v1.1.0";
pub const GITHUB_LINK: &str = "https://github.com/plarpoon/aprinter";
pub const CHANGELOG: &str = "OpenSSL is now statically linked";

pub fn print_version_info() {
    println!("Version: {}", VERSION);
    println!("GitHub Repository: {}", GITHUB_LINK);
    println!("Changelog: {}", CHANGELOG);
}
