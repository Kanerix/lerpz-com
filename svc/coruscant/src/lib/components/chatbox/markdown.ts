import hljs from "highlight.js/lib/common";
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";

const marked = new Marked(
    markedHighlight({
        emptyLangClass: "hljs",
        langPrefix: "hljs language-",
        highlight(code, lang) {
            if (lang && hljs.getLanguage(lang)) {
                return hljs.highlight(code, { language: lang }).value;
            }
            // No language hint (or an unknown one): let highlight.js guess.
            return hljs.highlightAuto(code).value;
        },
    }),
);

/**
 * Remove the leading indentation that is common to every non-blank line.
 *
 * Content coming from LLMs or pasted into the composer is sometimes uniformly
 * indented. Markdown treats lines indented by 4+ spaces as a code block, so an
 * indented message would otherwise be rendered as one giant `<pre>` (swallowing
 * fences, lists and prose, and triggering bogus syntax highlighting). Stripping
 * the shared indent is a no-op for normal content (common indent of 0) and
 * preserves relative nesting.
 */
function stripCommonIndent(text: string): string {
    const lines = text.split("\n");
    let min = Number.POSITIVE_INFINITY;
    for (const line of lines) {
        if (line.trim() === "") continue;
        const indent = line.length - line.trimStart().length;
        if (indent < min) min = indent;
    }
    if (!Number.isFinite(min) || min === 0) return text;
    return lines.map((line) => line.slice(min)).join("\n");
}

/** Parse a markdown string into highlighted (but unsanitized) HTML. */
export function renderMarkdown(content: string): string {
    return marked.parse(stripCommonIndent(content ?? ""), {
        async: false,
        gfm: true,
        breaks: true,
    }) as string;
}
