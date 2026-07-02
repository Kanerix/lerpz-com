import hljs from "highlight.js/lib/common";

/** Serialize a value to pretty-printed JSON, falling back gracefully. */
export function stringifyJson(value: unknown): string {
    let json: string;
    try {
        json = JSON.stringify(value, null, 2);
    } catch {
        json = String(value);
    }
    return json ?? "undefined";
}

/**
 * Serialize a value to pretty-printed JSON and return highlight.js HTML.
 *
 * highlight.js escapes the source before wrapping tokens in `<span>`s, so the
 * returned markup is safe to inject with `{@html}` without extra sanitization.
 * Token colors are supplied by the `.hljs-*` styles in `ErrorDialog.svelte`.
 */
export function highlightJson(value: unknown): string {
    return hljs.highlight(stringifyJson(value), { language: "json" }).value;
}
