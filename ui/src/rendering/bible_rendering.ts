import { Color, Word } from "../bindings.js";
import * as utils from "../utils.js";

export const HIGHLIGHT_SELECTED_WORD_COLOR = 'blueviolet';

export function render_word(word: Word, searched: string[] | null, c: Color | null)
{
    let word_node: HTMLElement = create_bible_word(word.text);
    if (word.italicized)
    {
        word_node = italicize(word_node);
    }
    
    if (searched !== null && searched.includes(utils.trim_string(word.text).toLocaleLowerCase()))
    {
        word_node = bold(word_node);
    }

    if(c !== null)
    {
        word_node = color(word_node, c);
    }

    return word_node
}

export function get_highest_priority_highlight(word_highlights: string[], catagories: any): string
{
    let max_highlight = word_highlights[0];
    for(let i = 1; i < word_highlights.length; i++)
    {
        let priority = catagories[word_highlights[i]].priority;
        let max_priority = catagories[max_highlight].priority

        if(priority > max_priority)
        {
            max_highlight = word_highlights[i];
        } 
    }

    return max_highlight;
}

export function create_bible_space(): HTMLElement
{
    let space = document.createElement('div');
    space.innerHTML = "&nbsp;"
    space.classList.add('bible-space');
    return space;
}

export function create_bible_word(t: string): HTMLElement
{
    let word = document.createElement('div');
    word.innerHTML = t;
    word.classList.add('bible-word');
    return word;
}

export function italicize(t: HTMLElement): HTMLElement
{
    let i = document.createElement('i');
    i.appendChild(t);
    return i;
}

export function bold(t: HTMLElement): HTMLElement
{
    let b = document.createElement('strong');
    b.appendChild(t);
    return b;
}

export function color(t: HTMLElement, c: Color): HTMLElement
{
    let span = document.createElement('span');
    span.appendChild(t);
    span.style.backgroundColor = utils.color_to_hex(c);
    return span;
}