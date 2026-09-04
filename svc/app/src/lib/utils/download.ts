/**
 * Trigger a browser download of an image to the current user's computer.
 *
 * The image is fetched into a {@link Blob} rather than relying on a plain
 * `<a href={url} download>` because the `download` attribute is ignored for
 * cross-origin URLs — the browser would navigate to / open the image instead
 * of saving it. Fetching to a blob forces an actual download and lets us name
 * the file. Note this requires the image host to permit cross-origin fetches
 * (CORS); otherwise the request will reject.
 *
 * Must run in the browser (it touches `document`), so call it from client-side
 * event handlers.
 *
 * @param url The image URL to download.
 * @param filename Optional file name. When omitted, it is derived from the URL
 *   and given an extension inferred from the response's MIME type.
 * @param options Optional settings, e.g. an `AbortSignal` to cancel the fetch.
 * @throws If the fetch fails or the response is not OK.
 */
export async function downloadImage(
    url: string,
    filename?: string,
    options?: { signal?: AbortSignal },
): Promise<void> {
    const response = await fetch(url, { signal: options?.signal });
    if (!response.ok) {
        throw new Error(
            `Failed to download image: ${response.status} ${response.statusText}`,
        );
    }

    const blob = await response.blob();
    const name = resolveFilename(url, filename, blob.type);

    saveBlob(blob, name);
}

/**
 * Trigger a browser download of an arbitrary file (e.g. a video) to the current
 * user's computer.
 *
 * Like {@link downloadImage}, the file is fetched into a {@link Blob} so the
 * `download` attribute is honoured even for cross-origin URLs (which requires
 * the host to permit CORS). Unlike `downloadImage`, the filename is required
 * since there's no MIME-based fallback for non-image types.
 *
 * Must run in the browser (it touches `document`).
 *
 * @param url The file URL to download.
 * @param filename The name to save the file as.
 * @param options Optional settings, e.g. an `AbortSignal` to cancel the fetch.
 * @throws If the fetch fails or the response is not OK.
 */
export async function downloadFile(
    url: string,
    filename: string,
    options?: { signal?: AbortSignal },
): Promise<void> {
    const response = await fetch(url, { signal: options?.signal });
    if (!response.ok) {
        throw new Error(
            `Failed to download file: ${response.status} ${response.statusText}`,
        );
    }

    const blob = await response.blob();
    saveBlob(blob, filename);
}

/** Save a blob to disk by clicking a temporary object-URL anchor. */
function saveBlob(blob: Blob, name: string) {
    const objectUrl = URL.createObjectURL(blob);
    try {
        const anchor = document.createElement("a");
        anchor.href = objectUrl;
        anchor.download = name;
        anchor.style.display = "none";
        document.body.appendChild(anchor);
        anchor.click();
        anchor.remove();
    } finally {
        URL.revokeObjectURL(objectUrl);
    }
}

/** Derive a download filename from the URL, falling back to a MIME-based extension. */
function resolveFilename(
    url: string,
    filename: string | undefined,
    mimeType: string,
): string {
    if (filename) return filename;

    let base = "image";
    try {
        const pathname = new URL(url, globalThis.location?.href).pathname;
        const last = pathname.split("/").pop();
        if (last) base = decodeURIComponent(last);
    } catch {
        // Malformed URL — keep the default base name.
    }

    if (base.includes(".")) return base;

    const extension = extensionForMimeType(mimeType);
    return extension ? `${base}.${extension}` : base;
}

/** Map a common image MIME type to a file extension. */
function extensionForMimeType(mimeType: string): string | undefined {
    const type = (mimeType.split(";")[0] ?? mimeType).trim().toLowerCase();
    switch (type) {
        case "image/jpeg":
            return "jpg";
        case "image/png":
            return "png";
        case "image/gif":
            return "gif";
        case "image/webp":
            return "webp";
        case "image/avif":
            return "avif";
        case "image/svg+xml":
            return "svg";
        case "image/bmp":
            return "bmp";
        case "image/tiff":
            return "tiff";
        default:
            return undefined;
    }
}
