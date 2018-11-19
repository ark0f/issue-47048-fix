#[cfg(all(target_os = "windows", target_env = "gnu"))]
extern crate cc;

fn main() {
    /*
     * Your build code
     */
}

fn issue_47048_fix() {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
        cc::Build::new()
        .file("issue_47048_fix.c")
        .compile("issue_47048_fix")
}
