import * as utils from "../utils.js";
import * as bible from "../bible.js";
import * as bible_renderer from "../bible_render.js";
import { BibleSection, ChapterIndex, VerseRange } from "../bindings.js";
import * as pages from "./pages.js";
import * as view_states from "../view_states.js";
import { ERASER_STATE_NAME } from "../save_states.js";

const CONTENT_ID: string = "chapter-text-content";
const CHAPTER_NAME_ID: string = "chapter-name"

const NEXT_CHAPTER_BUTTON_ID: string = "next-chapter-btn";
const PREVIOUS_CHAPTER_BUTTON_ID: string = "previous-chapter-btn";

export async function run()
{
    let data = utils.decode_from_url(window.location.href) as BibleSection;
    utils.init_format_copy_event_listener();
    
    Promise.all([
        pages.init_chapter_selection_dropdown(),
        pages.init_highlight_selection(null),
        pages.init_nav_buttons(),
        pages.init_highlight_editor_button(),
        pages.update_nav_buttons_opacity(),
        pages.init_search_bar(),
        utils.init_toggle('erase-highlight-toggle', ERASER_STATE_NAME, _ => {}),

        init_chapter_buttons(),
        display_chapter({book: data.book, number: data.chapter}, data.verse_range),
    ]).then(_ => {
        document.body.style.visibility = 'visible';
    });
}

async function display_chapter(chapter: ChapterIndex, verse_range: VerseRange | null)
{
    let chapter_view = await bible.load_view();
    
    let name = chapter_view[chapter.book].name;
    let number = chapter.number + 1;
    
    utils.set_value(pages.SEARCH_INPUT_ID, `${name} ${number}`);
    utils.set_html(CHAPTER_NAME_ID, `${name} ${number}`);
    bible_renderer.render_current_chapter(CONTENT_ID, pages.WORD_POPUP_ID, pages.POPUP_PANEL_ID, pages.update_word_selection).then(() => {
        if(verse_range !== null)
        {
            let start = verse_range.start;
            let element = document.getElementById(CONTENT_ID)?.getElementsByClassName(`verse-index-${start}`)[0];
            if (element !== undefined)
            {
                element.scrollIntoView();
                window.scrollBy(0, -40);
            }
        }

        pages.update_word_selection();
    });
}

async function init_chapter_buttons()
{
    utils.on_click(PREVIOUS_CHAPTER_BUTTON_ID, e => {
        bible.to_previous_chapter().then(() => {
            utils.reset_scroll();
            view_states.goto_current_view_state();
        })
    });

    utils.on_click(NEXT_CHAPTER_BUTTON_ID, e => {
        bible.to_next_chapter().then(() => {
            utils.reset_scroll();
            view_states.goto_current_view_state();
        })
    })
}