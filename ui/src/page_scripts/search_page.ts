import * as utils from "../utils/index.js";
import * as pages from "./pages.js";
import { SearchSection } from "../bindings.js";
import * as search from "../rendering/word_search.js";
import { PanelData } from "../popups/side_popup.js";
import * as bible from "../bible.js";

export function run()
{
    let section = utils.decode_from_url(window.location.href) as SearchSection;
    utils.init_format_copy_event_listener();

    bible.add_version_changed_listener(_ => {
        utils.scrolling.save_scroll(null);
    })

    Promise.all([
        pages.init_header(),
        display_search(section),
    ]).then(() => {
        document.body.style.visibility = 'visible';
        utils.scrolling.load_scroll();
    })
}

export async function display_search(section: SearchSection): Promise<void>
{
    const word_popup = document.getElementById('word-popup');
    const side_popup = document.getElementById('popup-panel');
    const side_popup_content = document.getElementById('popup-panel-content');
    
    if(word_popup === null) return;

    let side_popup_data: PanelData | null = null;
    if(side_popup && side_popup_content)
    {
        side_popup_data = {
            popup_panel: side_popup,
            popup_panel_content: side_popup_content,
        }
    }
    
    let search_result = await utils.invoke('run_word_search', { words: section.words });
    
    utils.set_value('search-input', section.words.join(" "));

    await search.render_search_result(search_result, section.words, 'word-search-content', word_popup, side_popup_data, section.display_index, 
        () => {
            pages.update_nav_buttons_opacity();
        }, 
        (msg: string) => {
            utils.set_value('search-input', msg);
            document.getElementById('search-btn')?.click();
        }
    );
}