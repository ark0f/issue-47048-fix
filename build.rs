#[cfg(all(target_os = "windows", target_env = "gnu"))]
extern crate cc;

fn main() {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    cc::Build::new()
        .file("fix.c")
        .compile("issue_47048_fix")
}
