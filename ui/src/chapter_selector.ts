import { get_bible_view } from "./bible.js";
import { debug_print } from "./utils/index.js";

function spawn_normal_option(name: string): HTMLElement
{
    let option_div = document.createElement('div');
    option_div.classList.add('dropdown-option');
    option_div.innerHTML = name;
    return option_div;
}

function spawn_expanded_option(name: string, count: number, on_selected: (name: string, number: number) => void): HTMLElement
{
    let option_div = document.createElement('div');
    option_div.classList.add('dropdown-option');
    option_div.classList.add('expanded-option');

    let title_div = document.createElement('div');
    title_div.classList.add('expanded-option-title');
    title_div.innerHTML = name;
    option_div.appendChild(title_div);

    option_div.appendChild(document.createElement('hr'));

    let grid_div = document.createElement('div');
    grid_div.classList.add("grid-container");

    for(let i = 0; i < count; i++)
    {
        let number = i + 1;   
        let grid_item_div = document.createElement('div');
        grid_item_div.classList.add('grid-item');
        grid_item_div.innerHTML = `${number}`;

        grid_item_div.addEventListener('click', e => {
            on_selected(name, number);
        })

        grid_div.appendChild(grid_item_div);
    }

    option_div.appendChild(grid_div);
    return option_div;
}

function spawn_option(name: string, count: number, selected: boolean, on_selected: (name: string, number: number) => void): HTMLElement
{
    if(selected)
    {
        return spawn_expanded_option(name, count, on_selected); 
    }
    else 
    {
        return spawn_normal_option(name);
    }
}

var current_selected_books: any = {};

export async function build_chapter_selection_dropdown(dropdown_content_id: string, on_selected: (name: string, number: number) => void)
{
    let books = await get_bible_view();

    let dropdown = document.getElementById(dropdown_content_id);
    if(dropdown === null) return;
    dropdown.replaceChildren();

    for(let i = 0; i < books.length; i++)
    {
        let name = books[i].name;
        let chapter_count = books[i].chapter_count;

        let option = spawn_option(name, chapter_count, name === current_selected_books[dropdown_content_id], on_selected);
        let selection_element = option as HTMLElement;

        let titles = option.getElementsByClassName('expanded-option-title');
        if(titles.length !== 0)
        {
            selection_element = titles[0] as HTMLElement;
        }
        
        selection_element.addEventListener('click', e => {
            if(name === current_selected_books[dropdown_content_id])
            {
                current_selected_books[dropdown_content_id] = null;
                build_chapter_selection_dropdown(dropdown_content_id, on_selected);
            }
            else 
            {
                current_selected_books[dropdown_content_id] = name;
                build_chapter_selection_dropdown(dropdown_content_id, on_selected);
            }
        })

        dropdown.appendChild(option);
    }
}