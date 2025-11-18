import { AlertPopupOption, spawn_alert_popup } from "../popups/alert_select_popup.js";
import { Marked } from "../vendor/markdown/marked.js";
export * from "./extensions.js";
export * from "./string_utils.js";
export * from "./color_utils.js";
export * from "./encoding.js";
export * from "./toggle.js";
export * from "./node_management.js";
export * as storage from "./storage.js";
export * from "./button.js";
export * from "./slider.js";
export * as scrolling from "./scrolling.js";
export * as ranges from "./ranges.js";
export * as tts from "./tts.js";
export * as events from "./events.js";
export * as images from "./images.js";
export * from "./dropdown.js";
export * as prompt from "./prompt.js";
export * from "./queue.js";

export const invoke: (fn_name: string, args: any) => Promise<any> = (window as any).__TAURI__.core.invoke;

export const emit_event: (event_name: string, data: any) => Promise<void> = (window as any).__TAURI__.event.emit;

export type AppEvent<T> = {
    event: string,
    payload: T,
    id: number,
}

export type UnlistenFn = () => void;
export type EventCallback<T> = (value: AppEvent<T>) => void;
export async function listen_event<T>(event_name: string, handler: EventCallback<T>): Promise<UnlistenFn>
{
    return (window as any).__TAURI__.event.listen(event_name, handler);
}

export type AsyncableFn = (() => void) | (() => Promise<void>);

export const LEFT_MOUSE_BUTTON = 0;
export const MIDDLE_MOUSE_BUTTON = 1;
export const RIGHT_MOUSE_BUTTON = 2;

export function debug_print(msg: string)
{
    invoke('debug_print', {message: msg});
}

export function debug_json(value: any, pretty_print: boolean = false)
{
    if (pretty_print)
    {
        debug_print(format_json(JSON.stringify(value)));
    }
    else 
    {
        debug_print(JSON.stringify(value));
    }
}

export type Interval = ReturnType<typeof setInterval>;

/**
 * Should generate a uuid in the version v4, although is not tested
 * @returns 
 */
export function gen_uuid_v4(): string 
{
    return '10000000-1000-4000-8000-100000000000'.replace(/018/g, c => 
        (+c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> +c / 4).toString(16)
    );
}

// a la chatgpt
export function format_html(html: string, tab: string = '  '): string
{
    let formatted = '';
    let indentLevel = 0;
    const voidElements = new Set(['area','base','br','col','embed','hr','img','input','link','meta','param','source','track','wbr']);
    
    html.replace(/>(\s*)</g, '><') // Remove unnecessary spaces between tags
        .split(/(?=<)|(?<=>)/g) // Split at tag boundaries
        .forEach((line) => {
            if (line.match(/^<\//)) {
                indentLevel = Math.max(indentLevel - 1, 0);
            }
            formatted += tab.repeat(indentLevel) + line.trim() + '\n';
            if (line.match(/^<([a-zA-Z0-9]+)([^>]*)>$/) && !voidElements.has(RegExp.$1)) {
                indentLevel++;
            }
        });
    
    return formatted.trim();
}

export function format_json(str: string, tab: string = '   '): string
{
    try 
    {
        const parsed = JSON.parse(str);
        return JSON.stringify(parsed, null, tab);
    } 
    catch (error) 
    {
        return 'Invalid JSON string';
    }
}

type Primitive = number | string | boolean | null | undefined;
type Comparable = 
    | Primitive
    | Comparable[]
    | { [key: string]: Comparable }


export function is_equivalent<T extends Comparable>(a: T, b: T): boolean
{
    if (a === b) return true;

    if (Array.isArray(a) && Array.isArray(b))
    {
        if (a.length !== b.length) return false;
        for(let i = 0; i < a.length; i++)
        {
            if (!is_equivalent(a[i], b[i]))
            {
                return false;
            }
        }

        return true;
    }

    if (is_plain_object(a) && is_plain_object(b))
    {
        let a_keys = Object.keys(a);
        let b_keys = Object.keys(b);
        if (a_keys.length !== b_keys.length) return false;

        for(let i = 0; i < a_keys.length; i++)
        {
            let key = a_keys[i];
            if (!is_equivalent(a[key], b[key]))
            {
                return false;
            }
        }

        return true;
    }


    return false;
}

export function is_plain_object(value: any): value is Record<string, unknown> 
{
    if (typeof value !== 'object' || value === null) return false;

    const proto = Object.getPrototypeOf(value);
    return proto === Object.prototype || proto === null;
}

export function overlap<T>(a: T[], b: T[]): T[]
{
    return a.filter(i => b.includes(i))
}

export function profile<R>(name: string, f: () => R): R
{
    let start = new Date().getTime();
    let r = f();
    let end = new Date().getTime();
    let elapsed = end - start;
    debug_print(`Task ${name} took ${elapsed}ms`);
    return r;
}

export async function is_app_initialized(): Promise<boolean>
{
    return await invoke('is_initialized', {});
}

export function reset_scroll()
{
    window.scrollTo(0, 0);
}

export function conserve_scroll(update: AsyncableFn, id?: string)
{
    let scrollable: Element | Window = window;
    let scroll = { x: window.scrollX, y: window.scrollY };

    if(id !== undefined)
    {
        let e = document.getElementById(id);
        if(e !== null)
        {
            scrollable = e;
            scroll = { x: e.scrollLeft, y: e.scrollTop }
        }
    }

    let r = update();
    if(r instanceof Promise)
    {
        r.then(() => {
            scrollable.scrollTo(scroll.x, scroll.y);
        })
    }
    else 
    {
        scrollable.scrollTo(scroll.x, scroll.y);
    }
}

let copy_event_listener: ((e: any) => void) | null = null;
export function init_format_copy_event_listener()
{
    if(copy_event_listener !== null)
    {
        document.removeEventListener('copy', copy_event_listener);
    }

    let listener = (event: any) => {
        let selection = window.getSelection();
        if(selection === null) return;
        let selectedText = selection.toString();
        selectedText = selectedText.replace(/\u00A0/g, ' '); // Replace non-breaking spaces
        event.clipboardData.setData('text/plain', selectedText);
        event.preventDefault(); // Prevent the default copy action
    }

    document.addEventListener('copy', listener);
    copy_event_listener = listener;
}

export function render_markdown(markdown: string): string
{
    let rendered = Marked.parse(markdown);
    return rendered;
}

export async function display_migration_popup()
{
    let should_display = await invoke('should_display_migration', {});

    if (should_display)
    {
        let title = 'Save Migrated'
        let message = 'Your save has been migrated to the latest version.';
        let options: AlertPopupOption[] = [
            {
                text: 'Ok',
                color: 'blue',
                callback: null,
            }
        ]

        spawn_alert_popup(title, message, options);
    }
}

export async function display_no_save_popup()
{
    let should_display = await invoke('should_display_no_save', {});
    if (should_display)
    {
        let title = 'No Save File'
        let message = 'No save found, creating a new save.';
        let options: AlertPopupOption[] = [
            {
                text: 'Ok',
                color: 'blue',
                callback: null,
            }
        ]

        spawn_alert_popup(title, message, options);
    }
}

export enum AudioClip 
{
    Flip = 'flip',
}

export function play_audio(clip: AudioClip)
{
    invoke('play_clip', { clip_name: clip as string });
}

export function map_keys<T, R>(obj: T, f: (k: keyof T, o: T) => R): R[]
{
    let array: R[] = [];
    for(let key in obj)
    {
        array.push(f(key, obj))
    }

    return array;
}

export function open(path: string)
{
    invoke('open', { path: path });
}

export function open_save_in_file_explorer()
{
    invoke('open_save_in_file_explorer', {}).then(e => {
        let error = e as string | null;
        if(error !== null)
        {
            alert('No save file has been created');
        }
    })
}

export function spawn_element<K extends keyof HTMLElementTagNameMap>(key: K, classes: string[], builder: (e: HTMLElementTagNameMap[K]) => void, parent?: HTMLElement): HTMLElementTagNameMap[K]
{
    let element = document.createElement(key);
    if (classes.length > 0)
    {
        element.classList.add(...classes);
    }
    
    builder(element);
    if(parent !== undefined)
    {
        parent.appendChild(element)
    }
    return element;
}


export function get_month_length(month: number, is_leap_year: boolean): number
{
    switch(month)
    {
        case 0: return 31; // Jan
        case 1: return is_leap_year ? 29 : 28; // Feb
        case 2: return 31; // Mar
        case 3: return 30; // Apr
        case 4: return 31; // May
        case 5: return 30; // Jun
        case 6: return 31; // Jul
        case 7: return 31; // August
        case 8: return 30; // Sept
        case 9: return 31; // Oct
        case 10: return 30; // Nov
        case 11: return 31; // Dec
    }

    return 0;
}

export function sleep(ms: number): Promise<void>
{
    return new Promise(resolve => setTimeout(resolve, ms));
}
