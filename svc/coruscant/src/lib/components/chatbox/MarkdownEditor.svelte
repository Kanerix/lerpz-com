<script lang="ts">
import { cn } from "@lerpz/ui/lib/utils";
import { Editor, Extension } from "@tiptap/core";
import { Placeholder } from "@tiptap/extensions";
import { Markdown } from "@tiptap/markdown";
import StarterKit from "@tiptap/starter-kit";
import { untrack } from "svelte";

let {
    value = "",
    onChange,
    onEnter,
    disabled = false,
    placeholder = "",
    autofocus = false,
    class: className = "",
}: {
    value?: string;
    onChange?: (markdown: string) => void;
    onEnter?: () => void;
    disabled?: boolean;
    placeholder?: string;
    autofocus?: boolean;
    class?: string;
} = $props();

let element = $state<HTMLDivElement | null>(null);
let editor = $state<Editor | null>(null);

// The last markdown string we either emitted upward or pushed into the editor.
// Used to break the value <-> editor feedback loop.
let lastSynced = untrack(() => value);

// Keep callbacks current without re-creating the editor when props change.
const handlers = untrack(() => ({ onChange, onEnter }));
$effect(() => {
    handlers.onChange = onChange;
    handlers.onEnter = onEnter;
});

const SubmitOnEnter = Extension.create({
    name: "submitOnEnter",
    addKeyboardShortcuts() {
        return {
            Enter: () => {
                handlers.onEnter?.();
                return true;
            },
        };
    },
});

$effect(() => {
    const el = element;
    if (!el) return;

    const instance = new Editor({
        element: el,
        editable: !untrack(() => disabled),
        autofocus: untrack(() => autofocus) ? "end" : false,
        content: untrack(() => value),
        contentType: "markdown",
        extensions: [
            StarterKit,
            Markdown.configure({
                markedOptions: { gfm: true },
            }),
            Placeholder.configure({
                placeholder: untrack(() => placeholder),
            }),
            SubmitOnEnter,
        ],
        editorProps: {
            attributes: {
                class: "markdown-editor focus:outline-none",
            },
        },
        onUpdate: ({ editor }) => {
            const markdown = editor.markdown?.serialize(editor.getJSON()) ?? "";
            lastSynced = markdown;
            handlers.onChange?.(markdown);
        },
    });

    editor = instance;
    return () => {
        instance.destroy();
        editor = null;
    };
});

$effect(() => {
    const next = value;
    const instance = editor;
    if (!instance) return;
    if (next === lastSynced) return;
    lastSynced = next;
    instance.commands.setContent(next, { contentType: "markdown" });
});

$effect(() => {
    editor?.setEditable(!disabled);
});
</script>

<div bind:this={element} class={cn("markdown-editor-root", className)}></div>

<style>
.markdown-editor-root :global(.markdown-editor) {
    min-height: 1.5rem;
    max-height: 40vh;
    overflow-y: auto;
    font-size: 0.875rem;
    line-height: 1.6;
    word-break: break-word;
}

/* Placeholder shown on the first empty block. */
.markdown-editor-root :global(.markdown-editor p.is-editor-empty:first-child::before) {
    content: attr(data-placeholder);
    color: var(--muted-foreground);
    float: left;
    height: 0;
    pointer-events: none;
}

.markdown-editor-root :global(.markdown-editor > * + *) {
    margin-top: 0.5rem;
}

.markdown-editor-root :global(.markdown-editor h1),
.markdown-editor-root :global(.markdown-editor h2),
.markdown-editor-root :global(.markdown-editor h3) {
    font-weight: 600;
    line-height: 1.3;
}

.markdown-editor-root :global(.markdown-editor h1) {
    font-size: 1.25rem;
}
.markdown-editor-root :global(.markdown-editor h2) {
    font-size: 1.125rem;
}
.markdown-editor-root :global(.markdown-editor h3) {
    font-size: 1rem;
}

.markdown-editor-root :global(.markdown-editor ul),
.markdown-editor-root :global(.markdown-editor ol) {
    padding-left: 1.25rem;
}
.markdown-editor-root :global(.markdown-editor ul) {
    list-style: disc;
}
.markdown-editor-root :global(.markdown-editor ol) {
    list-style: decimal;
}

.markdown-editor-root :global(.markdown-editor code) {
    background: var(--muted);
    border-radius: 0.25rem;
    padding: 0.1rem 0.3rem;
    font-size: 0.85em;
}

.markdown-editor-root :global(.markdown-editor pre) {
    background: var(--muted);
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
    overflow-x: auto;
}
.markdown-editor-root :global(.markdown-editor pre code) {
    background: transparent;
    padding: 0;
}

.markdown-editor-root :global(.markdown-editor blockquote) {
    border-left: 3px solid var(--border);
    padding-left: 0.75rem;
    color: var(--muted-foreground);
}

.markdown-editor-root :global(.markdown-editor a) {
    color: var(--primary);
    text-decoration: underline;
}
</style>
