use std::{array::TryFromSliceError, ops::Index};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Word {
    chars: [u8; 5]
}

impl Word {
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.chars)
    }

    pub fn positions_with_count(&self, character: u8) -> ([u8; 5], u8) {
        let mut positions = [255; 5];
        let mut count = 0;
        for (i, word_char) in self.chars.iter().enumerate()  {
            if *word_char == character {
                positions[count as usize] = i as u8;
                count += 1;
            }
        }
        (positions, count)
    }
}

impl Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl PartialEq<&str> for Word {
    fn eq(&self, other: &&str) -> bool {
        self.chars.as_slice() == other.as_bytes()
    }
}

impl PartialEq<Word> for &str {
    fn eq(&self, other: &Word) -> bool {
        self.as_bytes() == other.chars.as_slice()
    }
}

impl PartialEq<String> for Word {
    fn eq(&self, other: &String) -> bool {
        self.chars.as_slice() == other.as_bytes()
    }
}

impl PartialEq<Word> for String {
    fn eq(&self, other: &Word) -> bool {
        self.as_bytes() == other.chars.as_slice()
    }
}

impl TryFrom<String> for Word {
    type Error = TryFromSliceError;
    fn try_from(value: String) -> Result<Self, TryFromSliceError> {
        let chars = value.as_bytes().try_into()?;

        Ok(Self { chars: chars })
    }
}

impl TryFrom<&str> for Word {
    type Error = TryFromSliceError;
    fn try_from(value: &str) -> Result<Self, TryFromSliceError> {
        let chars = value.as_bytes().try_into()?;

        Ok(Self { chars: chars })
    }
}

impl TryFrom<&[u8; 5]> for Word {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8; 5]) -> Result<Self, TryFromSliceError> {
        Ok(Self { chars: *value })
    }
}

impl TryFrom<[u8; 5]> for Word {
    type Error = TryFromSliceError;
    fn try_from(value: [u8; 5]) -> Result<Self, TryFromSliceError> {
        Ok(Self { chars: value })
    }
}


impl Into<String> for Word {
    fn into(self) -> String {
        str::from_utf8(&self.chars).expect("").to_string()
    }
}

impl Into<String> for &Word {
    fn into(self) -> String {
        str::from_utf8(&self.chars).expect("").to_string()
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_word_creation() {
        let word = Word::try_from("hello").unwrap();

        assert_eq!(word.as_str().unwrap(), "hello");
        assert_eq!(word.to_string(), "hello");
        
        assert!(Word::try_from("hi").is_err());
        assert!(Word::try_from("hello there").is_err());
    }

    #[test]
    fn test_word_comparison() {
        let word = Word::try_from("hello").unwrap();

        assert_eq!(word, "hello");
        assert_eq!("hello", word);
        assert_ne!(word, "world");
    }
}