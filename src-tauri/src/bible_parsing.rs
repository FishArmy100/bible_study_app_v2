use itertools::Itertools;
use regex::Regex;

use crate::bible::*;

// format: Book Name 10:5 rest of the verse text

pub fn parse_verse(text: &str) -> Verse {
    let mut italicized = false;
    let mut word = String::new();
    let mut words = vec![];
    for i in 0..text.chars().count() {
        let c = text.chars().nth(i).unwrap();
        if c == '[' {
            italicized = true;
            continue;
        }
        if c == ']' {
            italicized = false;
            continue;
        }

        if c.is_whitespace() {
            if word.len() > 0 {
                words.push(Word {
                    text: std::mem::replace(&mut word, String::new()),
                    italicized,
                    red: false,
                });
            }
        }

        word.push(c);
    }

    if word.len() > 0 {
        words.push(Word {
            text: std::mem::replace(&mut word, String::new()),
            italicized,
            red: false,
        });
    }

    Verse { words }
}

pub fn parse_bible(text: &str) -> Result<Bible, String> {
    let pattern = Regex::new(r"\s*(?<book>[1-2]?\s*[a-zA-Z]+(?:\s+[a-zA-Z]+)*)\s+(?<chapter>\d+):(?<verse>\d+)(?:(?<text>[^\n]*))").unwrap();

    let mut lines = text.lines();
    let Some(version) = lines.next() else {
        return Err("Missing Bible version".into());
    };

    let Some(description) = lines.next() else {
        return Err("Missing Bible description".into());
    };

    let rest = lines.join("\n");

    let mut books: Vec<Book> = vec![];
    let mut last_chapter_number = 0;
    for cap in pattern.captures_iter(&rest)
    {
        let book_name = cap.name("book").unwrap().as_str();
        let chapter_str = cap.name("chapter").unwrap().as_str();
        let text = cap.name("text").map(|m| m.as_str()).unwrap_or("");
        let chapter_number = chapter_str.parse().unwrap();

        if books.is_empty() || books.last().unwrap().name != book_name {
            books.push(Book {
                name: book_name.into(),
                chapters: vec![],
            });
        }

        let book = books.last_mut().unwrap();

        if book.chapters.is_empty() || last_chapter_number != chapter_number {
            book.chapters.push(Chapter { verses: vec![] });
            last_chapter_number = chapter_number;
        }

        let chapter = book.chapters.last_mut().unwrap();

        chapter.verses.push(parse_verse(text));
    }

    Ok(Bible {
        name: version.into(),
        desc: description.into(),
        books,
    })
}
