fn main() {
  println!("{}", anagrams("biro".to_string()).join(", "));
}

fn anagrams(input: String) -> Vec<String> {
  if input.len() == 1 {
    return vec![input];
  }

  let mut ret = Vec::new();
  let chars = input.chars();
  for (i, c) in chars.enumerate() {
    let mut remaining = input.clone();
    remaining.remove(i);
    let result = anagrams(remaining);
    for r in result {
      let mut s = c.to_string();
      s.push_str(&r);
      ret.push(s);
    }
  }
  return ret;

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_returns_the_string_itself_as_an_anagram() {
    let result = anagrams("biro".to_string());
    assert!(result.contains( &"biro".to_string() ));
  }

  #[test]
  fn it_returns_the_reverse_of_the_string_as_an_anagram() {
    let result = anagrams("biro".to_string());
    assert!(result.contains( &"orib".to_string() ));
  }

  #[test]
  fn it_returns_all_the_anagrams_starting_with_the_letter_b() {
    let result = anagrams("biro".to_string());
    let expected = vec!["biro".to_string(), "brio".to_string(), "boir".to_string(), "bori".to_string(), "bior".to_string(), "brio".to_string()];
    for e in expected {
      assert!(result.contains( &e ));
    }
  }

  #[test]
  fn it_returns_all_the_anagrams_starting_with_the_letter_i() {
    let result = anagrams("biro".to_string());
    let expected = vec!["ibro".to_string(), "irbo".to_string(), "ibor".to_string(), "iorb".to_string(), "irbo".to_string(), "iorb".to_string()];
    for e in expected {
      assert!(result.contains( &e ));
    }
  }

  #[test]
  fn it_returns_all_the_anagrams_starting_with_the_letter_r() {
    let result = anagrams("biro".to_string());
    let expected = vec!["ribo".to_string(), "riob".to_string(), "rbio".to_string(), "rboi".to_string(), "roib".to_string(), "robi".to_string()];
    assert!(expected.iter().all(|expected_anagram| result.contains(expected_anagram)));
  }

  #[test]
  fn it_returns_all_the_anagrams_starting_with_the_letter_o() {
    let result = anagrams("biro".to_string());
    let expected = vec!["oirb".to_string(), "oirb".to_string(), "orbi".to_string(), "orib".to_string(), "oibr".to_string(), "oirb".to_string()];
    assert!(expected.iter().all(|expected_anagram| result.contains(expected_anagram)));
  }

  #[test]
  fn it_does_not_return_any_duplicates_in_the_array() {
    let result = anagrams("biro".to_string());
    let mut result_clone = result.clone();
    result_clone.sort();
    result_clone.dedup();
    assert_eq!(result_clone.len(), result.len());
  }

}


