import * as utils from "../utils/index.js";

import {EditorState} from "../vendor/prosemirror/prosemirror-state/index.js"
import {EditorView} from "../vendor/prosemirror/prosemirror-view/index.js"
import {DOMParser, Node, DOMSerializer} from "../vendor/prosemirror/prosemirror-model/index.js"
import * as setup from "./setup.js";
import { SCHEMA } from "./schema.js";
import { EventHandler } from "../utils/events.js";

export type TextEditorArgs = {
    id: string,
    parent: HTMLElement | null,
    on_ref_clicked: (text: string) => void,
    save?: TextEditorSave,
    has_misc_options: boolean, // close, delete, etc
}

export type TextEditorSaveType = 'html' | 'markdown' | 'json';
export type TextEditorSave = {
    data_type: TextEditorSaveType,
    source: string,
}

export function spawn_editor(args: TextEditorArgs): TextEditor
{
    return new TextEditor(args);
}

export class TextEditor
{
    private readonly root: HTMLElement;
    private readonly editor: HTMLElement;
    private readonly content: HTMLElement;
    private readonly view: EditorView;

    public readonly on_ref_created: EventHandler<HTMLElement> = new EventHandler();
    public readonly on_close: EventHandler<void> = new EventHandler();
    public readonly on_save: EventHandler<EditorState> = new EventHandler();
    public readonly on_delete: EventHandler<void> = new EventHandler();

    public constructor(args: TextEditorArgs)
    {
        this.root = utils.spawn_element('div', ['text-editor'], root => {
            root.id = args.id;
        });

        this.editor = utils.spawn_element('div', ['editor'], editor => {
            this.root.appendChild(editor);
        });

        this.content = utils.spawn_element('div', ['content'], content => {
            this.root.appendChild(content);
            content.style.display = 'none';
        });

        let parent = args.parent ?? document.body;
        parent.appendChild(this.root);
    
        this.view = new EditorView(this.root.querySelector(`#${args.id} > .editor`), {
            state: EditorState.create({
                doc: DOMParser.fromSchema(SCHEMA).parse(this.content),
                plugins: setup.build_plugins({
                    node_created_listeners: [{ name: 'bible_ref', on_event: (node: Node) => {
                        let s = DOMSerializer.fromSchema(SCHEMA);
                        let n = s.serializeNode(node) as HTMLElement;
                        this.on_ref_created.invoke(n);
                    }}],
                    on_close: this.on_close,
                    on_save: this.on_save,
                    on_delete: this.on_delete,
                    has_misc_options: args.has_misc_options,
                })
            })
        });

        let on_ref_clicked = (ev: MouseEvent) => {
            let target = ev.target as HTMLElement;
            let full_text = target.innerHTML;
            let trimmed = full_text.substring(1, full_text.length - 1); // [Gen 1:1] => Gen 1:1
            args.on_ref_clicked(trimmed);
        }; 


        let update_ref_click_listeners = () => 
        {
            this.view.dom.querySelectorAll('.bible-ref').values().filter(e => e instanceof HTMLElement).forEach(r => {
                r.removeEventListener('click', on_ref_clicked);
                r.addEventListener('click', on_ref_clicked);
            });
        }
        
        this.on_save.add_listener(_ => update_ref_click_listeners());

        // idk why we need to do this
        this.editor.querySelectorAll('.ProseMirror-menuseparator').forEach(s => {
            if(s instanceof HTMLElement)
            {
                s.style.display = 'block';
            }
        })
    }
    
    public get_save_json(): string 
    {
        return JSON.stringify(this.view.state.doc.toJSON());
    }

    public get_save(): TextEditorSave
    {
        return {
            data_type: 'json',
            source: this.get_save_json(),
        }
    }

    public load_save(save: TextEditorSave): boolean
    {
        switch(save.data_type)
        {
            case "html":
                this.content.innerHTML = save.source;
                this.view.state.doc = DOMParser.fromSchema(SCHEMA).parse(this.content);
                this.view.update({
                    state: this.view.state,
                })
                break;
            case "markdown":
                let html = utils.render_markdown(save.source);
                this.content.innerHTML = html;
                this.view.state.doc = DOMParser.fromSchema(SCHEMA).parse(this.content);
                this.view.update({
                    state: this.view.state,
                })
                break;
            case "json":
                try 
                {
                    if (!utils.is_empty_str(save.source))
                    {
                        let data = JSON.parse(save.source);
                        this.view.state.doc = Node.fromJSON(SCHEMA, data);
                        this.view.update({ state: this.view.state });
                    }
                }
                catch 
                {
                    utils.debug_print(`Error formatting json content: ${save.source}`);
                    return false;
                }
                break;
        }

        return true;
    }
}