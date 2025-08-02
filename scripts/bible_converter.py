import json
import sys
import os
from typing import List, Tuple
from dataclasses import dataclass
from dacite import from_dict
import re

@dataclass
class SrcVerse:
    verse: int
    chapter: int
    name: str
    text: str

@dataclass
class SrcChapter:
    chapter: int
    name: str
    verses: List[SrcVerse]

@dataclass
class SrcBook:
    name: str
    chapters: List[SrcChapter]

@dataclass
class SrcBible:
    books: List[SrcBook]

class DestWord:
    red: bool | None
    italics: bool | None
    begin_punc: str | None
    end_punc: str | None
    text: str

    def __init__(self, text: str, red: bool | None, italics: bool | None, begin_punc: str | None, end_punc: str | None) -> None:
        self.text = text
        self.red = red
        self.italics = italics
        self.begin_punc = begin_punc
        self.end_punc = end_punc
        

    def to_json(self) -> str:
        s = "{ \"text\": \"" + self.text + "\""

        if self.red != None:
            s += ", \"red\": " + str(self.red)
        if self.italics != None:
            s += ", \"italics\": " + str(self.italics)

        if self.begin_punc != None:
            s += ", \"begin_punc\": \"" + self.begin_punc + "\""
        if self.end_punc != None:
            s += ", \"end_punc\": \"" + self.end_punc + "\""

        s += " }"
        return s

class DestVerse:
    words: List[DestWord]
    id: str

    def __init__(self, id: str, words: List[DestWord]) -> None:
        self.id = id
        self.words = words

    def to_json(self) -> str:
        words = ', '.join(map(lambda w : f"{w.to_json()}", self.words))
        return f"{{ \"id\": \"{self.id}\", \"words\": [{words}] }}"

class DestBible:
    verses: List[DestVerse]

    def __init__(self, verses: List[DestVerse]) -> None:
        self.verses = verses

    def to_jsonl(self) -> str:
        ret = ""
        for v in self.verses:
            ret += v.to_json() + "\n"

        return ret


if len(sys.argv) < 2:
    raise RuntimeError("You must pass a file path")
elif len(sys.argv) > 2:
    raise RuntimeError("More than 1 argument was supplied")

path = sys.argv[1]

if not os.path.isfile(path):
    raise RuntimeError(f"File path {path} is not a valid path")

_, ext = os.path.splitext(path)
if not ext == ".json":
    raise RuntimeError(f"File path {path} is not a json file")

with open(path, 'r') as f:
    bible = json.load(f)

bible = from_dict(SrcBible, bible)


bible_to_osis = {
    "Genesis": "Gen",
    "Exodus": "Exod",
    "Leviticus": "Lev",
    "Numbers": "Num",
    "Deuteronomy": "Deut",
    "Joshua": "Josh",
    "Judges": "Judg",
    "Ruth": "Ruth",
    "I Samuel": "1Sam",
    "II Samuel": "2Sam",
    "I Kings": "1Kgs",
    "II Kings": "2Kgs",
    "I Chronicles": "1Chr",
    "II Chronicles": "2Chr",
    "Ezra": "Ezra",
    "Nehemiah": "Neh",
    "Esther": "Esth",
    "Job": "Job",
    "Psalms": "Ps",
    "Proverbs": "Prov",
    "Ecclesiastes": "Eccl",
    "Song of Solomon": "Song",
    "Isaiah": "Isa",
    "Jeremiah": "Jer",
    "Lamentations": "Lam",
    "Ezekiel": "Ezek",
    "Daniel": "Dan",
    "Hosea": "Hos",
    "Joel": "Joel",
    "Amos": "Amos",
    "Obadiah": "Obad",
    "Jonah": "Jonah",
    "Micah": "Mic",
    "Nahum": "Nah",
    "Habakkuk": "Hab",
    "Zephaniah": "Zeph",
    "Haggai": "Hag",
    "Zechariah": "Zech",
    "Malachi": "Mal",
    "Matthew": "Matt",
    "Mark": "Mark",
    "Luke": "Luke",
    "John": "John",
    "Acts": "Acts",
    "Romans": "Rom",
    "I Corinthians": "1Cor",
    "II Corinthians": "2Cor",
    "Galatians": "Gal",
    "Ephesians": "Eph",
    "Philippians": "Phil",
    "Colossians": "Col",
    "I Thessalonians": "1Thess",
    "II Thessalonians": "2Thess",
    "I Timothy": "1Tim",
    "II Timothy": "2Tim",
    "Titus": "Titus",
    "Philemon": "Phlm",
    "Hebrews": "Heb",
    "James": "Jas",
    "I Peter": "1Pet",
    "II Peter": "2Pet",
    "I John": "1John",
    "II John": "2John",
    "III John": "3John",
    "Jude": "Jude",
    "Revelation": "Rev",
    "Revelation of John": "Rev"
}

def split_punctuated_word(text: str) -> Tuple[str | None, str, str | None]:
    # Match beginning punctuation, word text, and ending punctuation
    match = re.match(r'^(\W*)(\w+)(\W*)$', text)
    if match:
        begin_punc = match.group(1)
        word = match.group(2)
        end_punc = match.group(3)
    else:
        # If the text doesn't contain any word characters
        begin_punc, word, end_punc = '', text, ''
    
    if len(begin_punc) == 0:
        begin_punc = None
    
    if len(end_punc) == 0:
        end_punc = None

    return begin_punc, word, end_punc

verses: List[DestVerse] = []
for book in bible.books:
    for chapter in book.chapters:
        for verse in chapter.verses:
            book_name = bible_to_osis[book.name]
            id: str = f"{book_name}.{chapter.chapter}.{verse.verse}"

            text_words = verse.text.split()
            words: List[DestWord] = []
            for w in text_words:
                begin_punc, text, end_punc = split_punctuated_word(w)
                words.append(DestWord(text, None, None, begin_punc, end_punc))

            verses.append(DestVerse(id, words))
    print(f"Parsed {book.name}")

dest_bible = DestBible(verses)

print("Writing to file...")

with open('out.jsonl', 'w') as file:
    file.write(dest_bible.to_jsonl())

print("Done!")



            


