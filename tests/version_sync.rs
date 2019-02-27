use version_sync::assert_contains_regex;

#[test]
fn readme() {
    assert_contains_regex!(
        "README.md",
        r#"https://docs\.rs/{name}/badge\.svg\?version={version}"#
    );
    assert_contains_regex!("README.md", r#"https://docs\.rs/{name}/{version}"#);
}
