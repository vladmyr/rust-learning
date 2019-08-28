use std::io;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<(), io::Error> {
  content
    .lines()
    .filter(|line| line.contains(pattern))
    .try_for_each(|line| writeln!(writer, "{}", line))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn find_matches_test_001() {
    let mut result = Vec::new();

    assert!(find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).is_ok());
    assert_eq!(result, b"lorem ipsum\n");
  }
}