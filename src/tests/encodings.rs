use tempfile::tempdir;

use crate::properties_of;

const SIMPLE_EDITORCONFIG: &str = "
root = true
[*.rs]
end_of_line = lf
";

// Happy path: UTF-8 files are still supported
#[test]
fn utf8_works() -> anyhow::Result<()> {
    // Create a temp dir
    let temp_dir = tempdir()?;
    let temp_root = temp_dir.path();

    // Create an editorconfig file with UTF-8 encoding in the temp dir
    let editorconfig_path = temp_root.join(".editorconfig");
    std::fs::write(editorconfig_path, SIMPLE_EDITORCONFIG)?;

    // Ask about a file in the temp dir (which doesn't need to exist)
    let subject = properties_of(temp_root.join("foo.rs"));

    // Clean Up temp dir before making assertions
    temp_dir.close()?;

    // Assert that we get back an expected property.
    let subject = subject.expect("Failed to parse .editorconfig file and get properties");
    let end_of_line = subject.get_raw_for_key("end_of_line").into_str();
    assert_eq!(end_of_line.to_owned(), "lf");
    Ok(())
}

// UTF-8 with BOM

// UTF-16-LE

// UTF-16-BE

// Error case: Reporting a not supported encoding error, rather than a parse error

// todo: UTF-32 is not supported in file-content-rs yet
