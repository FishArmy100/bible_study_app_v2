// ---------------------- Bible Bindings ----------------------
export type Word = { text: string, italicized: boolean, red: boolean };

export type Verse = { words: Array<string> };
export type Chapter = { verses: Array<Verse> };

export type ChapterIndex = { book: number, number: number };
export type VerseRange = { start: number, end: number };
export type WordRange = { verseStart: number };
export type ReferenceLocation = { chapter: ChapterIndex, range: WordRange };
export type WordPosition = { book: number, chapter: number, verse: number}

export type BookView = { name: string, chapterCount: number };
export type ChapterView = { verses: Array<number> };

// ---------------------- Notes Bindings -----------------------
export type HighlightCategory = { color: Color, name: string, description: string, priority: number, id: string };
export type NoteData = { id: string, text: string };
export type WordAnnotations = { highlights: Array<string>, notes: Array<string> };

// ------------------------ Misc Bindings ----------------------------
export type Color = { r: number, g: number, b: number };
export type BibleSection = { book: number, chapter: number, verseRange: VerseRange | null };