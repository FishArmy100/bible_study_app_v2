use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::num::NonZeroU32;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum RefId 
{
    Single(Atom),
    Range { from: Atom, to: Atom },
}

impl RefId
{
    pub fn is_valid(&self) -> bool
    {
        match self 
        {
            Self::Single(_) => true,
            Self::Range { from, to } => match (from, to)
            {
                (Atom::Book { book: _ }, Atom::Book { book: _ }) => true,
                (Atom::Chapter { book: _, chapter: _ }, Atom::Chapter { book: _, chapter: _ }) => true,
                (Atom::Verse { book: _, chapter: _, verse: _ }, Atom::Verse { book: _, chapter: _, verse: _ }) => true,
                (Atom::Word { book: _, chapter: _, verse: _, word: _ }, Atom::Word { book: _, chapter: _, verse: _, word: _ }) => true,
                _ => false,
            }
        }
    }

    pub fn get_verse_components(&self) -> Option<(&str, u32, u32)>
    {
        if let Self::Single(Atom::Verse { book, chapter, verse }) = self 
        {
            Some((book, chapter.get(), verse.get()))
        }
        else 
        {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Atom 
{
    Book { book: String },
    Chapter { book: String, chapter: NonZeroU32 },
    Verse { book: String, chapter: NonZeroU32, verse: NonZeroU32 },
    Word { book: String, chapter: NonZeroU32, verse: NonZeroU32, word: NonZeroU32 },
}

impl fmt::Display for Atom 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self {
            Atom::Book { book } => write!(f, "{}", book),
            Atom::Chapter { book, chapter } => write!(f, "{}.{}", book, chapter),
            Atom::Verse { book, chapter, verse } => write!(f, "{}.{}.{}", book, chapter, verse),
            Atom::Word { book, chapter, verse, word } => {
                write!(f, "{}.{}.{}#{}", book, chapter, verse, word)
            }
        }
    }
}

impl fmt::Display for RefId 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self {
            RefId::Single(atom) => write!(f, "{}", atom),
            RefId::Range { from, to } => write!(f, "{}-{}", from, to),
        }
    }
}

impl FromStr for Atom 
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        let (main, word_opt) = if let Some((main, word)) = s.split_once('#') {
            (main, Some(word))
        } 
        else 
        {
            (s, None)
        };

        let parts: Vec<&str> = main.split('.').collect();
        match (parts.len(), word_opt) {
            (1, None) => Ok(Atom::Book {
                book: parts[0].to_string(),
            }),
            (2, None) => Ok(Atom::Chapter {
                book: parts[0].to_string(),
                chapter: parts[1].parse().map_err(|_| "Invalid chapter")?,
            }),
            (3, None) => Ok(Atom::Verse {
                book: parts[0].to_string(),
                chapter: parts[1].parse().map_err(|_| "Invalid chapter")?,
                verse: parts[2].parse().map_err(|_| "Invalid verse")?,
            }),
            (3, Some(word)) => Ok(Atom::Word {
                book: parts[0].to_string(),
                chapter: parts[1].parse().map_err(|_| "Invalid chapter")?,
                verse: parts[2].parse().map_err(|_| "Invalid verse")?,
                word: word.parse().map_err(|_| "Invalid word")?,
            }),
            _ => Err(format!("Unrecognized Atom format: {}", s)),
        }
    }
}

impl FromStr for RefId 
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        if let Some((from, to)) = s.split_once('-') {
            Ok(RefId::Range {
                from: Atom::from_str(from.trim())?,
                to: Atom::from_str(to.trim())?,
            })
        } 
        else 
        {
            Ok(RefId::Single(Atom::from_str(s.trim())?))
        }
    }
}

// --- Custom Serde Support for Ref ---

impl Serialize for RefId 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer 
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RefId 
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> 
    {
        let s = String::deserialize(deserializer)?;
        RefId::from_str(&s).map_err(serde::de::Error::custom)
    }
}
