<script lang="ts">
import { cn } from "@lerpz/ui/lib/utils";
import DOMPurify from "dompurify";
import { browser } from "$app/environment";
import { renderMarkdown } from "./markdown.js";

let {
    content = "",
    class: className = "",
}: { content?: string; class?: string } = $props();

let html = $derived.by(() => {
    if (!browser) return null;
    return DOMPurify.sanitize(renderMarkdown(content));
});
</script>

{#if html !== null}
  <div class={cn("markdown-body", className)}>{@html html}</div>
{:else}
  <div class={cn("markdown-body whitespace-pre-wrap", className)}>{content}</div>
{/if}

<style>
.markdown-body :global(> * + *) {
    margin-top: 0.5rem;
}

.markdown-body :global(h1),
.markdown-body :global(h2),
.markdown-body :global(h3) {
    font-weight: 600;
    line-height: 1.3;
}
.markdown-body :global(h1) {
    font-size: 1.25rem;
}
.markdown-body :global(h2) {
    font-size: 1.125rem;
}
.markdown-body :global(h3) {
    font-size: 1rem;
}

.markdown-body :global(ul),
.markdown-body :global(ol) {
    padding-left: 1.25rem;
}
.markdown-body :global(ul) {
    list-style: disc;
}
.markdown-body :global(ol) {
    list-style: decimal;
}

.markdown-body :global(code) {
    background: var(--muted);
    border-radius: 0.25rem;
    padding: 0.1rem 0.3rem;
    font-size: 0.85em;
}

.markdown-body :global(pre) {
    background: var(--muted);
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
    overflow-x: auto;
    /* Match the ScrollArea component's thumb (Firefox). */
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
}
/* Match the ScrollArea component's thumb (WebKit/Chromium). */
.markdown-body :global(pre::-webkit-scrollbar) {
    height: 0.625rem;
    width: 0.625rem;
}
.markdown-body :global(pre::-webkit-scrollbar-track) {
    background: transparent;
}
.markdown-body :global(pre::-webkit-scrollbar-thumb) {
    background: var(--border);
    border-radius: 9999px;
    border: 2px solid transparent;
    background-clip: padding-box;
}
.markdown-body :global(pre code) {
    background: transparent;
    padding: 0;
}

.markdown-body :global(blockquote) {
    border-left: 3px solid var(--border);
    padding-left: 0.75rem;
    color: var(--muted-foreground);
}

.markdown-body :global(a) {
    color: var(--primary);
    text-decoration: underline;
}

.markdown-body :global(table) {
    border-collapse: collapse;
    width: 100%;
}
.markdown-body :global(th),
.markdown-body :global(td) {
    border: 1px solid var(--border);
    padding: 0.35rem 0.6rem;
    text-align: left;
}

.markdown-body :global(hr) {
    border: none;
    border-top: 1px solid var(--border);
}

/*
 * Variant for messages rendered on the solid primary bubble: the default
 * prose colors (muted code background, primary links) are invisible there, so
 * derive everything from the bubble's own foreground color instead.
 */
.markdown-on-primary :global(code) {
    background: color-mix(in oklch, var(--primary-foreground) 20%, transparent);
}
.markdown-on-primary :global(pre) {
    background: color-mix(in oklch, var(--primary-foreground) 15%, transparent);
    scrollbar-color: color-mix(in oklch, var(--primary-foreground) 40%, transparent) transparent;
}
.markdown-on-primary :global(pre::-webkit-scrollbar-thumb) {
    background: color-mix(in oklch, var(--primary-foreground) 40%, transparent);
}
.markdown-on-primary :global(a) {
    color: inherit;
    text-decoration: underline;
}
.markdown-on-primary :global(blockquote) {
    border-left-color: color-mix(in oklch, var(--primary-foreground) 40%, transparent);
    color: inherit;
}
.markdown-on-primary :global(th),
.markdown-on-primary :global(td) {
    border-color: color-mix(in oklch, var(--primary-foreground) 30%, transparent);
}
.markdown-on-primary :global(hr) {
    border-top-color: color-mix(in oklch, var(--primary-foreground) 30%, transparent);
}

/* highlight.js token theme (GitHub-style), matched to class-based dark mode. */
.markdown-body :global(.hljs) {
    color: var(--foreground);
    background: transparent;
}
.markdown-body :global(.hljs-comment),
.markdown-body :global(.hljs-quote) {
    color: #6a737d;
    font-style: italic;
}
.markdown-body :global(.hljs-keyword),
.markdown-body :global(.hljs-selector-tag),
.markdown-body :global(.hljs-deletion) {
    color: #d73a49;
}
.markdown-body :global(.hljs-string),
.markdown-body :global(.hljs-meta .hljs-string),
.markdown-body :global(.hljs-regexp),
.markdown-body :global(.hljs-addition) {
    color: #032f62;
}
.markdown-body :global(.hljs-number),
.markdown-body :global(.hljs-literal),
.markdown-body :global(.hljs-variable),
.markdown-body :global(.hljs-template-variable),
.markdown-body :global(.hljs-attr),
.markdown-body :global(.hljs-attribute) {
    color: #005cc5;
}
.markdown-body :global(.hljs-title),
.markdown-body :global(.hljs-title.function_),
.markdown-body :global(.hljs-section) {
    color: #6f42c1;
}
.markdown-body :global(.hljs-built_in),
.markdown-body :global(.hljs-type),
.markdown-body :global(.hljs-title.class_),
.markdown-body :global(.hljs-meta) {
    color: #e36209;
}
.markdown-body :global(.hljs-name),
.markdown-body :global(.hljs-symbol),
.markdown-body :global(.hljs-bullet) {
    color: #22863a;
}
.markdown-body :global(.hljs-emphasis) {
    font-style: italic;
}
.markdown-body :global(.hljs-strong) {
    font-weight: 600;
}

:global(.dark) .markdown-body :global(.hljs-comment),
:global(.dark) .markdown-body :global(.hljs-quote) {
    color: #8b949e;
}
:global(.dark) .markdown-body :global(.hljs-keyword),
:global(.dark) .markdown-body :global(.hljs-selector-tag),
:global(.dark) .markdown-body :global(.hljs-deletion) {
    color: #ff7b72;
}
:global(.dark) .markdown-body :global(.hljs-string),
:global(.dark) .markdown-body :global(.hljs-meta .hljs-string),
:global(.dark) .markdown-body :global(.hljs-regexp),
:global(.dark) .markdown-body :global(.hljs-addition) {
    color: #a5d6ff;
}
:global(.dark) .markdown-body :global(.hljs-number),
:global(.dark) .markdown-body :global(.hljs-literal),
:global(.dark) .markdown-body :global(.hljs-variable),
:global(.dark) .markdown-body :global(.hljs-template-variable),
:global(.dark) .markdown-body :global(.hljs-attr),
:global(.dark) .markdown-body :global(.hljs-attribute) {
    color: #79c0ff;
}
:global(.dark) .markdown-body :global(.hljs-title),
:global(.dark) .markdown-body :global(.hljs-title.function_),
:global(.dark) .markdown-body :global(.hljs-section) {
    color: #d2a8ff;
}
:global(.dark) .markdown-body :global(.hljs-built_in),
:global(.dark) .markdown-body :global(.hljs-type),
:global(.dark) .markdown-body :global(.hljs-title.class_),
:global(.dark) .markdown-body :global(.hljs-meta) {
    color: #ffa657;
}
:global(.dark) .markdown-body :global(.hljs-name),
:global(.dark) .markdown-body :global(.hljs-symbol),
:global(.dark) .markdown-body :global(.hljs-bullet) {
    color: #7ee787;
}
</style>
