use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use tauri::{path::BaseDirectory, Manager, Runtime, State};
use uuid::Uuid;

use crate::{
    app_state::{self, AppState, ViewState}, audio::reader_behavior::ReaderBehavior, bible::{ChapterIndex, ReferenceLocation, Verse}, notes::{HighlightCategory, NoteSourceType, WordAnnotations}, searching::{self, *}, settings::Settings, utils::{self, Color}
};

#[tauri::command(rename_all = "snake_case")]
pub fn is_initialized(app_state: State<'_, AppState>) -> bool
{
    app_state.is_initialized()
}

#[tauri::command(rename_all = "snake_case")]
pub fn debug_print(message: &str) {
    println!("Debug from JS: `{}`", message);
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_bible_view(app_state: State<'_, AppState>) -> String {
    let view = app_state.get().as_ref().unwrap().get_current_bible().get_view();
    serde_json::to_string(&view.books).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_current_view_state(app_state: State<'_, AppState>) -> ViewState {
    let app_state = app_state.get_ref();

    let index = app_state.get_view_state_index();
    app_state.read_view_states(|states| states[index].clone())
}

#[tauri::command(rename_all = "snake_case")]
pub fn push_view_state(app_state: State<'_, AppState>, view_state: ViewState) 
{
    let index = get_view_state_index(app_state.clone());
    app_state.get_ref().read_view_states(|states| {
        if states.last().is_some_and(|s| *s == view_state) {
            return;
        }

        let next = (index + 1) as usize;
        if next != states.len() {
            states.drain(next..states.len());
        }

        states.push(view_state.clone());
    });

    to_next_view_state(app_state);
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_view_state_count(app_state: State<'_, AppState>) -> u32 
{
    app_state.get_ref().read_view_states(|states| states.len() as u32)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_view_state_index(app_state: State<'_, AppState>) -> u32 
{
    app_state.get_ref().get_view_state_index() as u32
}

#[tauri::command(rename_all = "snake_case")]
pub fn to_next_view_state(app_state: State<'_, AppState>) 
{
    let current = get_view_state_index(app_state.clone());
    let max = get_view_state_count(app_state.clone()) - 1;

    if current < max 
    {
        app_state.get_ref().set_view_state_index(current as usize + 1);
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn to_previous_view_state(app_state: State<'_, AppState>) 
{
    let current = get_view_state_index(app_state.clone());
    if current > 0 {
        app_state.get_ref().set_view_state_index(current as usize - 1);
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn clear_view_states(app_state: State<'_, AppState>)
{
    let app_state = app_state.get_ref();
    let current = app_state.get_view_state_index();

    app_state.read_view_states(|view_states| {
        let last = view_states[current].clone();
        view_states.clear();
        view_states.push(last);
    });

    app_state.set_view_state_index(0);
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_chapter_text(app_state: State<'_, AppState>, chapter: ChapterIndex) -> String 
{
    let app_state = app_state.get_ref();
    let chapter = &app_state.get_current_bible().books[chapter.book as usize].chapters[chapter.number as usize];
    serde_json::to_string(chapter).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_verse(app_state: State<'_, AppState>, book: u32, chapter: u32, verse: u32) -> Verse {
    app_state.get_ref().get_current_bible().books[book as usize].chapters[chapter as usize].verses[verse as usize]
        .clone()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_book_name(app_state: State<'_, AppState>, book: u32) -> String {
    app_state.get_ref().get_current_bible().books[book as usize].name.clone()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_chapter_view(app_state: State<'_, AppState>, chapter: ChapterIndex) -> String {
    let view = app_state.get_ref().get_current_bible().books[chapter.book as usize].chapters[chapter.number as usize]
        .get_view();
    serde_json::to_string(&view).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_highlight_categories(app_state: State<'_, AppState>) -> String {
    app_state.get_ref()
        .read_notes(|notebook| serde_json::to_string(&notebook.highlight_categories).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_highlight_category(app_state: State<'_, AppState>, color: &str, name: &str, description: &str, source_type: NoteSourceType, priority: &str) {
    app_state.get_ref().read_notes(|notebook| {
        let color = Color::from_hex(color).unwrap();
        let name = name.to_string();
        let description = description.to_string();
        let priority: u32 = priority.parse().unwrap();
        let id = uuid::Uuid::new_v4().to_string();

        let category = HighlightCategory {
            color,
            name,
            source_type,
            description,
            priority,
            id: id.clone(),
        };

        notebook.highlight_categories.insert(id, category);
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_highlight_category(app_state: State<'_, AppState>, id: &str) {
    app_state.get_ref().read_notes(|notebook| {
        notebook.highlight_categories.remove(&id.to_string());
        notebook.refresh_highlights();
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_highlight_category(
    app_state: State<'_, AppState>,
    id: &str,
    color: &str,
    name: &str,
    description: &str,
    source_type: NoteSourceType,
    priority: &str,
) {
    app_state.get_ref().read_notes(|notebook| {
        let color = Color::from_hex(color).unwrap();
        let name = name.to_string();
        let description = description.to_string();
        let priority: u32 = priority.parse().unwrap();

        let category = HighlightCategory {
            color,
            name,
            description,
            priority,
            source_type,
            id: id.to_string(),
        };

        notebook
            .highlight_categories
            .insert(id.to_string(), category);
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_chapter_annotations(app_state: State<'_, AppState>, chapter: ChapterIndex) -> String {
    app_state.get_ref().read_notes(|notebook| {
        if let Some(highlights) = notebook.annotations.get(&chapter) {
            serde_json::to_string(highlights).unwrap()
        } else {
            serde_json::to_string("").unwrap()
        }
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn highlight_word(app_state: State<'_, AppState>, chapter: ChapterIndex, word_position: u32, highlight_id: &str) {
    app_state.get_ref().read_notes(|notebook| {
        let chapter_annotations = match notebook.annotations.get_mut(&chapter) {
            Some(highlights) => highlights,
            None => {
                notebook.annotations.insert(chapter.clone(), HashMap::new());
                notebook.annotations.get_mut(&chapter).unwrap()
            }
        };

        let word_notes = match chapter_annotations.get_mut(&word_position) {
            Some(word_highlights) => word_highlights,
            None => {
                chapter_annotations.insert(
                    word_position,
                    WordAnnotations {
                        highlights: vec![],
                        notes: vec![],
                    },
                );
                chapter_annotations.get_mut(&word_position).unwrap()
            }
        };

        let highlight_id = highlight_id.to_string();
        if !word_notes.highlights.contains(&highlight_id) {
            word_notes.highlights.push(highlight_id);
        }
    });
}

#[tauri::command(rename_all = "snake_case")]
pub fn erase_highlight(app_state: State<'_, AppState>, chapter: ChapterIndex, word_position: u32, highlight_id: &str) {
    app_state.get_ref().read_notes(|notebook| {
        let Some(chapter_highlights) = notebook.annotations.get_mut(&chapter) else {
            return;
        };

        let Some(word_notes) = chapter_highlights.get_mut(&word_position) else {
            return;
        };

        word_notes.highlights.retain(|h| h != highlight_id);
    });
}

#[tauri::command(rename_all = "snake_case")]
pub fn parse_bible_search(app_state: State<'_, AppState>, text: &str) -> ParsedSearchResult {
    let app_state = app_state.get_ref();
    let bible = app_state.get_current_bible();
    parse_search(text, bible)
}

#[tauri::command(rename_all = "snake_case")]
pub fn run_word_search(app_state: State<'_, AppState>, words: Vec<String>) -> Vec<WordSearchResult> {
    let app_state = app_state.get_ref();
    let bible = app_state.get_current_bible();
    let words = words.iter().map(|w| w.as_str()).collect_vec();
    search_bible(&words, bible)
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_note(app_state: State<'_, AppState>, text: String, locations: Vec<ReferenceLocation>, source_type: NoteSourceType) -> String {
    let app_state = app_state.get_ref();
    let current_bible = app_state.get_current_bible();

    app_state.read_notes(move |notebook| {
        let id = uuid::Uuid::new_v4().to_string();
        notebook.add_note(
            current_bible,
            id.clone(),
            text.to_owned(),
            locations.to_owned(),
            source_type,
        );
        id
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_note(app_state: State<'_, AppState>, id: &str) {
    app_state.get_ref().read_notes(|notebook| {
        notebook.remove_note(id);
    });
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_note(app_state: State<'_, AppState>, id: &str) -> String {
    app_state.get_ref().read_notes(|notebook| {
        let note = notebook.get_note(id);
        serde_json::to_string(note).unwrap()
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_editing_note(app_state: State<'_, AppState>) -> Option<String> {
    app_state.get_ref().read_editing_note(|editing_note| editing_note.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_editing_note(app_state: State<'_, AppState>, note: Option<String>) {
    app_state.get_ref().read_editing_note(|editing_note| *editing_note = note.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_note(app_state: State<'_, AppState>, id: String, locations: Vec<ReferenceLocation>, text: String, source_type: NoteSourceType) {
    let app_state = app_state.get_ref();
    let current_bible = app_state.get_current_bible();

    app_state.read_notes(move |notebook| {
        notebook.remove_note(&id);
        notebook.add_note(
            current_bible,
            id.clone(),
            text.clone(),
            locations.clone(),
            source_type,
        );
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_settings(app_state: State<'_, AppState>) -> Settings
{
    app_state.get_ref().read_settings(|settings| {
        settings.clone()
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_settings(app_state: State<'_, AppState>, settings: Option<Settings>)
{
    app_state.get_ref().read_settings(|old| {

        match &settings
        {
            Some(s) => *old = s.clone(),
            None => *old = Settings::default()
        };
    });
}

#[tauri::command(rename_all = "snake_case")]
pub fn should_display_migration(app_state: State<'_, AppState>) -> bool 
{
    app_state.get_ref().should_display_migration()
}

#[tauri::command(rename_all = "snake_case")]
pub fn should_display_no_save(app_state: State<'_, AppState>) -> bool 
{
    app_state.get_ref().should_display_no_save()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_book_from_name(app_state: State<'_, AppState>, prefix: Option<u32>, name: &str) -> Option<BookTitleData>
{
    searching::get_book_from_name(prefix, name, &app_state.get_ref().get_current_bible()).ok()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_selected_reading(app_state: State<'_, AppState>) -> u32 
{
    app_state.get_ref().read_selected_reading(|selected_reading| {
        *selected_reading
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_selected_reading(app_state: State<'_, AppState>, selected_reading: u32)
{
    app_state.get_ref().read_selected_reading(|sr| {
        *sr = selected_reading;
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn open(path: &str)
{
    match utils::open(path)
    {
        Ok(_) => {},
        Err(err) => println!("ERROR: {}", err),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_save_in_file_explorer<R>(app: tauri::AppHandle<R>) -> Option<String>
    where R : Runtime
{
    let has_save = app.path().resolve(app_state::SAVE_NAME, BaseDirectory::Resource).unwrap().as_path().exists();
    if !has_save 
    {
        return Some("App save has not been created".into())
    }

    let path = app.path().resolve("", BaseDirectory::Resource).unwrap();
    let path_str = path.to_str().unwrap();
    utils::open(path_str).err().map(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_current_bible_version(app_state: State<'_, AppState>) -> String
{
    app_state.get_ref().get_current_bible_version()
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_current_bible_version(app_state: State<'_, AppState>, version: String)
{
    app_state.get_ref().set_current_bible_version(version);
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_bible_versions(app_state: State<'_, AppState>) -> Vec<String>
{
    app_state.get_ref().get_bibles()
        .map(|v| v.clone())
        .collect()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_reader_behavior(app_state: State<'_, AppState>) -> ReaderBehavior
{
    app_state.get_ref().read_reader_behavior(|b| b.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_reader_behavior(app_state: State<'_, AppState>, reader_behavior: ReaderBehavior)
{
    app_state.get_ref().read_reader_behavior(|b| *b = reader_behavior)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_recent_highlights(app_state: State<'_, AppState>) -> Vec<String>
{
    app_state.get_ref().read_recent_highlights(|r| {
        r.iter().map(|id| id.to_string()).collect_vec()
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_recent_highlights(app_state: State<'_, AppState>, recent_highlights: Vec<String>)
{
    app_state.get_ref().read_recent_highlights(move |r| {
        *r = recent_highlights.iter()
            .map(|id| Uuid::from_str(id).unwrap())
            .collect_vec()
    })
}