import * as utils from "../utils/index.js";
import { init_settings_page_header } from "./menu_header.js";
import * as pages from "./pages.js";
import * as settings from "../settings.js"

export type HelpPageData = {
    old_path: string,
}

export function run()
{
    let data = utils.decode_from_url(window.location.href) as HelpPageData;
    init_settings_page_header({
        middle: [],
        on_back_clicked: () => {
            window.location.href = data.old_path;
        },
        old_path: data.old_path,
    });
    settings.init_less_sync();
    init_faq_dropdowns();
    init_links();
    document.body.style.visibility = 'visible';
}

function init_faq_dropdowns()
{
    document.querySelectorAll('.faq-question').forEach(question => {
        question.addEventListener('click', () => {
            const faq_item = question.parentElement;
            if(!faq_item) return;
            const answer = faq_item.querySelector('.faq-answer');
            if(!(answer instanceof HTMLElement)) return;

            // Close other open items
            document.querySelectorAll('.faq-item').forEach(item => {
                if (item !== faq_item) {
                    item.classList.remove('active');
                    let answer = item.querySelector('.faq-answer');
                    if(answer instanceof HTMLElement)
                    {
                        answer.style.maxHeight = '';
                    }
                }
            });

            // Toggle active state
            if (faq_item.classList.contains('active')) 
            {
                faq_item.classList.remove('active');
                answer.style.maxHeight = '';
            } 
            else 
            {
                faq_item.classList.add('active');
                answer.style.maxHeight = answer.scrollHeight + "px";
            }
        });
    });
}

function init_links()
{
    let links = document.body.querySelectorAll('.faq-answer a')
        .values()
        .map(l => l as HTMLAnchorElement)
        .toArray();
    
    links.forEach(l => {
        l.addEventListener('click', e => {
            e.preventDefault();
            utils.open(l.href);
        });
    });
}