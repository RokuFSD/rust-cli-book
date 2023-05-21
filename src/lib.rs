use std::io::Write;

pub fn find_match(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_match("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
