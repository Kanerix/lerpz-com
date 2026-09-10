// Hand-written client for the chat archive/unarchive endpoint.
//
// It lives here rather than in `$lib/api`, which orval regenerates from the
// api OpenAPI spec (`bun run generate:api`) and would overwrite. This endpoint,
// PATCH /api/v1/chats/{id}, isn't in the generated client yet. Once the API is
// regenerated against a build that includes it, prefer the generated
// `updateChat`/`createUpdateChat` and delete this file.

import type { Conversation } from "$lib/api/models/index.js";
import { customFetch } from "./orval-mutator.js";

export type UpdateChatBody = {
    /** Archive (`true`) or restore (`false`) the conversation. */
    archived: boolean;
};

export const getUpdateChatUrl = (id: string) => `/api/v1/chats/${id}`;

/**
 * Updates mutable fields on a conversation (currently just `archived`).
 * Resolves with the updated conversation, throws on a non-2xx response.
 */
export async function updateChat(
    id: string,
    body: UpdateChatBody,
): Promise<Conversation> {
    const res = await customFetch<{ data: Conversation; status: number }>(
        getUpdateChatUrl(id),
        {
            method: "PATCH",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body),
        },
    );
    return res.data;
}
