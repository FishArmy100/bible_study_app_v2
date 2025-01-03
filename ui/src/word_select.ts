import { ChapterIndex } from "./bindings";
import * as highlighting from "./highlights.js";
import { HIGHLIGHT_SELECTED_WORD_COLOR } from "./rendering/bible_rendering.js";
import * as utils from "./utils/index.js"
import * as notes from "./notes.js";
import * as bible from "./bible.js";
import * as view_states from "./view_states.js";

export function init_word_selection(side_popup: HTMLElement | null, on_require_rerender: () => Promise<void>): void
{
    document.addEventListener('mouseup', _e => on_stop_dragging(side_popup, on_require_rerender));
}

type WordRange = {
    parent: HTMLElement,
    words: HTMLElement[],
    chapter: ChapterIndex,
    word_offset: number,
}

let ranges: WordRange[] = [];

export function clear_selection_ranges()
{
    ranges = [];
}

export function push_selection_range(content: HTMLElement, chapter: ChapterIndex, word_offset: number)
{
    let word_nodes = content.getElementsByClassName('bible-word');

    for(let i = 0; i < word_nodes.length; i++)
    {
        let word_node = word_nodes[i] as HTMLElement;
        word_node.addEventListener('mousedown', e => {
            if(e.button !== utils.LEFT_MOUSE_BUTTON) return;
            on_start_dragging(chapter, word_offset + i, word_node);
        });

        word_node.addEventListener('mouseover', e => {
            if(e.button !== utils.LEFT_MOUSE_BUTTON) return;
            on_over_dragging(chapter, word_offset + i, word_node);
        });
    }

    ranges.push({
        words: [...word_nodes] as HTMLElement[],
        chapter: chapter,
        word_offset: word_offset,
        parent: content,
    });
}

type WordData = {
    html: HTMLElement,
    chapter: ChapterIndex,
    word_index: number,
}

type SelectingNoteListener =
{
    on_start: () => void,
    on_end: () => void,
}

let note_listeners: SelectingNoteListener[] = [];
export function add_note_listener(listener: SelectingNoteListener)
{
    note_listeners.push(listener);
}

export function remove_note_listener(listener: SelectingNoteListener)
{
    note_listeners.remove(listener);
}

function update_start_listeners()
{
    note_listeners.forEach(l => {
        l.on_start();
    });
}

function update_end_listeners()
{
    note_listeners.forEach(l => {
        l.on_end();
    })
}

let making_note = false;
export function begin_making_note()
{
    if(making_note || editing_note) return;
    making_note = true;
    update_words_for_selection();
    update_start_listeners();
}

let on_edit_fn: (() => void) | null = null;
let editing_note = false;
export function begin_editing_note(on_edit: (() => void) | null)
{
    if (making_note || editing_note) return;
    editing_note = true;
    on_edit_fn = on_edit;
    update_words_for_selection();
    update_start_listeners();
}

export function is_selecting(): boolean
{
    return editing_note || making_note || highlighting.SELECTED_HIGHLIGHT.get() !== null;
}

export function stop_selecting()
{
    begin = null;
    end = null;
    making_note = false;
    editing_note = false;

    if(highlighting.SELECTED_HIGHLIGHT.get() !== null)
    {
        highlighting.SELECTED_HIGHLIGHT.set(null);
        highlighting.ERASING_HIGHLIGHT.set(false);
    }

    update_words_for_selection();
    on_edit_fn = null;
    update_end_listeners();
}

let is_dragging = false;
let begin: WordData | null = null;
let end: WordData | null = null;

function on_start_dragging(chapter: ChapterIndex, word_index: number, word_div: HTMLElement) 
{
    if(highlighting.SELECTED_HIGHLIGHT.get() !== null)
    {
        is_dragging = true;
        update_highlight_words(chapter, word_index, word_div);
    }
    else if(making_note)
    {
        is_dragging = true;
        begin = { html: word_div, word_index: word_index, chapter: chapter };
        end = { html: word_div, word_index: word_index, chapter: chapter };
        update_selected_note_words();
    }
    else if(editing_note)
    {
        is_dragging = true;
        begin = { html: word_div, word_index: word_index, chapter: chapter };
        end = { html: word_div, word_index: word_index, chapter: chapter };
        update_selected_note_words();
    }
}

function on_over_dragging(chapter: ChapterIndex, word_index: number, word_div: HTMLElement) 
{
    if(is_dragging && highlighting.SELECTED_HIGHLIGHT.get() !== null)
    {
        update_highlight_words(chapter, word_index, word_div);
    }
    else if(is_dragging && (making_note || editing_note) && begin !== null)
    {
        let parent = ranges.find(r => r.words.includes(word_div))?.parent
        let begin_parent = ranges.find(r => {
            if (begin === null) return false;
            return r.words.includes(begin.html);
        })?.parent;

        if(parent && begin_parent && parent === begin_parent)
        {
            end = { html: word_div, chapter: chapter, word_index: word_index };
            update_selected_note_words();
        }
    }
}

async function on_stop_dragging(word_popup: HTMLElement | null, on_require_rerender: () => Promise<void>) 
{
    if(is_dragging && highlighting.SELECTED_HIGHLIGHT.get() !== null)
    {
        is_dragging = false;
        word_popup?.classList.remove('show');

        let scroll = window.scrollY;

        on_require_rerender().then(() => {
            window.scrollTo(window.scrollX, scroll);
        });
    }
    else if(is_dragging && (making_note || editing_note) && begin && end)
    {
        update_words_for_selection();
        is_dragging = false;
        let scroll = window.scrollY;

        let range = ranges.find(r => {
            if (begin === null) return false;
            return r.words.includes(begin?.html);
        });

        if(range)
        {
            for(let i = 0; i < range.words.length; i++)
            {
                range.words[i].style.color = '';
            }
        }

        let view = await bible.get_chapter_view(begin.chapter);
        let [verse_start, word_start] = bible.expand_word_index(view, begin.word_index);
        let [verse_end, word_end] = bible.expand_word_index(view, end.word_index);

        let verse_range = {
            verse_start: verse_start,
            verse_end: verse_end,
            word_start: word_start,
            word_end: word_end,
        };

        if(making_note)
        {
            notes.create_note({
                chapter: begin.chapter,
                range: verse_range,
            }).then(r => {
                if (r !== null) // if created a new note, set it as the currently editing note
                {
                    notes.set_editing_note(r).then(_ => {
                        view_states.goto_current_view_state();
                    });
                }
            });
        }
        else if(editing_note)
        {
            let editing_id = await notes.get_editing_note();
            if(editing_id !== null)
            {
                let editing_note = await notes.get_note(editing_id);
                editing_note.locations.push({
                    chapter: begin.chapter,
                    range: verse_range,
                });

                await notes.update_note(editing_note.id, editing_note.locations, editing_note.text);
            }

            if(on_edit_fn !== null)
            {
                on_edit_fn();
            }
        }

        stop_selecting();

        on_require_rerender().then(() => {
            window.scrollTo(window.scrollX, scroll);
        });
    }
}

function update_selected_note_words()
{
    let range = ranges.find(r => {
        if (begin === null) return false;
        return r.words.includes(begin?.html);
    });

    if(!range || !begin || !end) return;
    for(let i = 0; i < range.words.length; i++)
    {
        let word_node = range.words[i];
        word_node.style.color = '';
    }

    let a = range.words.indexOf(begin.html);
    let b = range.words.indexOf(end.html);

    let begin_index = Math.min(a, b);
    let end_index = Math.max(a, b);

    for(let i = begin_index; i < end_index + 1; i++)
    {
        range.words[i].style.color = HIGHLIGHT_SELECTED_WORD_COLOR;
    }
}

function update_highlight_words(chapter: ChapterIndex, word: number, div: HTMLElement)
{
    div.style.color = HIGHLIGHT_SELECTED_WORD_COLOR;
    let selected_highlight = highlighting.SELECTED_HIGHLIGHT.get();

    if(selected_highlight === null) return;
    if(highlighting.ERASING_HIGHLIGHT.get() !== true)
    {
        highlighting.highlight_word(chapter, word, selected_highlight);
    }
    else 
    {
        highlighting.erase_highlight(chapter, word, selected_highlight);
    }
}

export function update_words_for_selection()
{
    if(making_note || editing_note || highlighting.SELECTED_HIGHLIGHT.get() !== null)
    {
        document.querySelectorAll('.bible-word, .bible-space').forEach(w => {
            (w as HTMLElement).style.userSelect = 'none';
            (w as HTMLElement).style.cursor = 'pointer';
        });
    }
    else 
    {
        document.querySelectorAll('.bible-word, .bible-space').forEach(w => {
            (w as HTMLElement).style.userSelect = 'text';
            (w as HTMLElement).style.cursor = 'default';
        });
    }
}