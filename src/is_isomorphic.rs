use std::collections::HashMap;
use std::collections::HashSet;

/**
Determines if two strings are isomorphic.

Two strings are isomorphic if the characters in the first string can be 
replaced to get the second string, with a one-to-one correspondence between
the characters.

# Arguments

* `s` - The first input string to check for isomorphism.
* `t` - The second input string to check for isomorphism.

# Returns

* A boolean value - `true` if the strings are isomorphic, and `false`
  otherwise.

# Examples

```
# use your_crate::is_isomorphic;
assert_eq!(is_isomorphic(String::from("egg"), String::from("add")), true);
assert_eq!(is_isomorphic(String::from("foo"), String::from("bar")), false);
```
*/
pub fn is_isomorphic(s: String, t: String) -> bool {
  let mut s_to_t:HashMap<char, char> = HashMap::new();
  let mut seen_t_chars:HashSet<char> = HashSet::new();
  
  for (i, s_char) in s.char_indices() {
    let t_char = t.as_bytes()[i] as char;

    if let Some(map_t_char) = s_to_t.insert(s_char, t_char) {
      if map_t_char != t_char {
        return false;
      }
    } else if !seen_t_chars.insert(t_char) {
      return false;
    } 
  }

  true
}

#[cfg(test)]
mod tests {
    use super::is_isomorphic;

    #[test]
    fn test_identical_strings() {
        assert!(is_isomorphic("hello".to_string(), "hello".to_string()));
    }

    #[test]
    fn test_isomorphic_strings() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
    }

    #[test]
    fn test_non_isomorphic_strings() {
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
    }

    #[test]
    fn test_single_char_strings() {
        assert!(is_isomorphic("a".to_string(), "a".to_string()));
    }

    #[test]
    fn test_isomorphic_single_char_strings() {
        assert!(is_isomorphic("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_isomorphic_with_space() {
        assert!(is_isomorphic(" ".to_string(), "b".to_string()));
    }

    #[test]
    fn test_non_isomorphic_with_repeated_chars() {
        assert!(!is_isomorphic("aba".to_string(), "baa".to_string()));
    }
}
