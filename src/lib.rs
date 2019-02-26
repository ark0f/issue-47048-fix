use std::{env, fs::File, io::Write, path::Path};

/// Compile C file with fix and add to your executable
///
/// `build.rs`:
/// ```
/// use issue_47048_fix::issue_47048_fix;
///
/// fn main() {
///     /* your build code */
///
///     issue_47048_fix();
/// }
/// ```
///
pub fn issue_47048_fix() {
    let path = env::var("OUT_DIR").expect("Failed to get 'OUT_DIR' environment variable");
    let path = Path::new(&path).join("issue_47048_fix.c");

    if !path.exists() {
        let mut out_file =
            File::create(path.clone()).expect("Failed to create file 'issue_47048_fix.c'");
        write!(
            &mut out_file,
            "{}",
            r#"
/*
 * Fix by Trevor Spiteri
 * https://github.com/tspiteri
 * Thank you!
 */

#define _CRTBLD
#include <stdio.h>

FILE *__cdecl __acrt_iob_func(unsigned index)
{
	return &(__iob_func()[index]);
}

typedef FILE *__cdecl (*_f__acrt_iob_func)(unsigned index);
_f__acrt_iob_func __MINGW_IMP_SYMBOL(__acrt_iob_func) = __acrt_iob_func;
"#
        )
        .expect("Failed to write content into file 'issue_47048_fix.c'");
    }

    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    cc::Build::new().file(path).compile("issue_47048_fix")
}
