use std::path::MAIN_SEPARATOR;

pub fn basename(input: &str) -> &str {
    let mut index = 0;
    for (id, character) in input.char_indices() {
        if character == MAIN_SEPARATOR {
            index = id;
        }
    }
    if index == 0 {
        input
    } else {
        &input[index + 1..]
    }
}

/// Removes the extension of a given input
pub fn remove_extension(input: &str) -> &str {
    let mut dir_index = 0;
    let mut ext_index = 0;

    for (id, character) in input.char_indices() {
        if character == MAIN_SEPARATOR {
            dir_index = id;
        }
        if character == '.' {
            ext_index = id;
        }
    }

    // Account for hidden files and directories
    if ext_index == 0 || dir_index + 2 > ext_index {
        input
    } else {
        &input[0..ext_index]
    }
}

pub fn dirname(input: &str) -> &str {
    let mut index = 0;
    for (id, character) in input.char_indices() {
        if character == MAIN_SEPARATOR {
            index = id;
        }
    }
    if index == 0 { "." } else { &input[0..index] }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn correct(input: &str) -> String {
        let mut sep = String::new();
        sep.push(MAIN_SEPARATOR);
        input.replace('/', &sep)
    }

    #[test]
    fn path_remove_ext_simple() {
        assert_eq!(remove_extension("foo.txt"), "foo");
    }

    #[test]
    fn path_remove_ext_dir() {
        assert_eq!(
            remove_extension(&correct("dir/foo.txt")),
            &correct("dir/foo")
        );
    }

    #[test]
    fn path_hidden() {
        assert_eq!(remove_extension(".foo"), ".foo")
    }

    #[test]
    fn path_remove_ext_utf8() {
        assert_eq!(remove_extension("💖.txt"), "💖");
    }

    #[test]
    fn path_remove_ext_empty() {
        assert_eq!(remove_extension(""), "");
    }

    #[test]
    fn path_basename_simple() {
        assert_eq!(basename("foo.txt"), "foo.txt");
    }

    #[test]
    fn path_basename_dir() {
        assert_eq!(basename(&correct("dir/foo.txt")), "foo.txt");
    }

    #[test]
    fn path_basename_empty() {
        assert_eq!(basename(""), "");
    }

    #[test]
    fn path_basename_utf8() {
        assert_eq!(basename(&correct("💖/foo.txt")), "foo.txt");
        assert_eq!(basename(&correct("dir/💖.txt")), "💖.txt");
    }

    #[test]
    fn path_dirname_simple() {
        assert_eq!(dirname("foo.txt"), ".");
    }

    #[test]
    fn path_dirname_dir() {
        assert_eq!(dirname(&correct("dir/foo.txt")), "dir");
    }

    #[test]
    fn path_dirname_utf8() {
        assert_eq!(dirname(&correct("💖/foo.txt")), "💖");
        assert_eq!(dirname(&correct("dir/💖.txt")), "dir");
    }

    #[test]
    fn path_dirname_empty() {
        assert_eq!(dirname(""), ".");
    }
}
