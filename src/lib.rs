/// Compile C file with fix and add to your executable
///
/// `build.rs`:
/// ```no_run
/// use issue_47048_fix::issue_47048_fix;
///
/// fn main() {
///     /* your build code */
///
///     issue_47048_fix();
/// }
/// ```
///
#[cfg(all(target_os = "windows", target_env = "gnu"))]
pub fn issue_47048_fix() {
    use std::{env, fs::File, io::Write, path::Path};

    let path = env::var("OUT_DIR").expect("Failed to get 'OUT_DIR' environment variable");
    let path = Path::new(&path).join("issue_47048_fix.c");

    if !path.exists() {
        let mut out_file =
            File::create(path.clone()).expect("Failed to create file 'issue_47048_fix.c'");
        write!(&mut out_file, "{}", include_str!("fix.c"))
            .expect("Failed to write content into file 'issue_47048_fix.c'");
    }

    cc::Build::new().file(path).compile("issue_47048_fix")
}

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
pub fn issue_47048_fix() {}
