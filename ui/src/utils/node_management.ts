import { debug_print } from "./index.js";

export function set_display(id: string, display: string)
{
    let element = document.getElementById(id);
    if(element !== null)
    {
        element.style.display = display;
    }
}

export function on_click(id: string, f:(e: Event) => void)
{
    let element = document.getElementById(id);
    if(element === null)
    {
        debug_print(`could not find element ${id}`);
    }
    element?.addEventListener('click', (e) => {
        f(e)
    });
}

export function read_value(id: string): string | undefined
{
    let element = document.getElementById(id);
    if(element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement)
    {
        return element.value;
    }
    else 
    {
        return undefined
    }
}

export function set_value(id: string, value: string)
{
    let element = document.getElementById(id);
    if(element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement)
    {
        element.value = value;
    }
}

export function set_html(id: string, html: string)
{
    let element = document.getElementById(id);
    if (element instanceof Element)
    {
        element.innerHTML = html;
    }
}

export function set_opacity(id: string, opacity: string)
{
    let element = document.getElementById(id);
    if(element instanceof HTMLElement)
    {
        element.style.opacity = opacity;
    }
}