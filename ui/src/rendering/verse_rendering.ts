import { ChapterIndex, Word } from "../bindings.js";
import * as utils from "../utils.js"
import * as rendering from "./bible_rendering.js";
import * as bible from "../bible.js";
import * as highlighting from "../highlights.js"
import * as wp from "../word_popup.js";
import * as sp from "../side_popup.js";
import { ERASER_STATE_NAME } from "../save_states.js";

const HIGHLIGHT_CATAGORIES = await highlighting.get_catagories();

/**
 * Note: this function must be called only once on page reload
 */
export function init_highlighting(side_popup: HTMLElement, on_require_rerender: () => Promise<void>): void
{
    document.addEventListener('mouseup', _e => on_stop_dragging(side_popup, on_require_rerender));
}

export type VerseRenderArgs = {
    chapter: ChapterIndex,
    verse: number,
    word_popup: HTMLElement,
    side_popup: HTMLElement,
    side_popup_content: HTMLElement,
    bolded: string[] | null,
}

export async function render_verse(args: VerseRenderArgs): Promise<HTMLElement[]>
{
    let elements = [];

    let verse_data = await utils.invoke('get_verse', { book: args.chapter.book, chapter: args.chapter.number, verse: args.verse });
    let words: Word[] = verse_data.words;
    let offset = await bible.get_verse_word_offset(args.chapter.book, args.chapter.number, args.verse);
    let chapter_annotations = JSON.parse(await utils.invoke('get_chapter_annotations', { chapter: args.chapter}));
    
    let last_word_highlights = null;
    for(let i = 0; i < words.length; i++)
    {
        let word_annotations = chapter_annotations[offset + i];

        if(i != 0)
        {
            let space: HTMLElement = rendering.create_bible_space();

            if(word_annotations !== undefined && word_annotations !== null && word_annotations.highlights.length !== 0 && last_word_highlights !== null)
            {
                let overlap: any[] = utils.overlap(word_annotations.highlights, last_word_highlights);
                if(overlap.length !== 0)
                {
                    let space_highlight = rendering.get_highest_priority_highlight(overlap, HIGHLIGHT_CATAGORIES);
                    let space_color = HIGHLIGHT_CATAGORIES[space_highlight].color;
                    space = rendering.color(space, space_color);
                }
            }
            
            elements.push(space);
        }

        let color = null;
        if(word_annotations !== null && word_annotations !== undefined && word_annotations.highlights.length !== 0)
        {
            let id = rendering.get_highest_priority_highlight(word_annotations.highlights, HIGHLIGHT_CATAGORIES);
            color = HIGHLIGHT_CATAGORIES[id].color;
            last_word_highlights = word_annotations.highlights;
        }
        else 
        {
            last_word_highlights = null;
        }

        let word_node = rendering.render_word(words[i], args.bolded, color);
        if(word_annotations !== null && word_annotations !== undefined && word_annotations.highlights.length !== 0)
        {
            wp.display_on_div(word_node, word_annotations.highlights.map((h: string) => HIGHLIGHT_CATAGORIES[h].color), args.word_popup);

            let word = utils.trim_string(words[i].text);
            sp.display_on_div(word_node, word, word_annotations.highlights, HIGHLIGHT_CATAGORIES, args.side_popup, args.side_popup_content);
        }

        word_node.addEventListener('mousedown', e => {
            on_start_dragging(args.chapter, offset + i, word_node);
        });

        word_node.addEventListener('mouseover', e => {
            on_over_dragging(args.chapter, offset + i, word_node);
        });

        elements.push(word_node);
    }

    return elements;
}

let is_dragging = false;
function on_start_dragging(chapter: ChapterIndex, word_index: number, word_div: HTMLElement) 
{
    if(highlighting.get_selected_highlight() !== null)
    {
        is_dragging = true;
        update_word(chapter, word_index, word_div);
    }
}

function on_over_dragging(chapter: ChapterIndex, word_index: number, word_div: HTMLElement) 
{
    if(is_dragging && highlighting.get_selected_highlight() !== null)
    {
        update_word(chapter, word_index, word_div);
    }
}

function on_stop_dragging(word_popup: HTMLElement, on_require_rerender: () => Promise<void>) 
{
    if(is_dragging && highlighting.get_selected_highlight() !== null)
    {
        is_dragging = false;
        word_popup.classList.remove('show');

        let scroll = window.scrollY;

        on_require_rerender().then(() => {
            window.scrollTo(window.scrollX, scroll);
        });
    }
}

function update_word(chapter: ChapterIndex, word: number, div: HTMLElement)
{
    div.style.color = rendering.HIGHLIGHT_SELECTED_WORD_COLOR;
    let selected_highlight = highlighting.get_selected_highlight();

    if(selected_highlight === null) return;
    if(utils.get_toggle_value(ERASER_STATE_NAME) !== true)
    {
        highlighting.highlight_word(chapter, word, selected_highlight);
    }
    else 
    {
        highlighting.erase_highlight(chapter, word, selected_highlight);
    }
}