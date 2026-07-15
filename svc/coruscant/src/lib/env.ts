import * as v from "valibot";
import { env } from "$env/dynamic/public";

/**
 * Schema for the public (client-exposed) runtime environment.
 *
 * These values are read from `$env/dynamic/public` rather than
 * `$env/static/public`, so the image is built once and configured at deploy
 * time. Validating here guarantees every value is present and well-formed
 * before the app touches it, and narrows the types from `string | undefined`
 * to required `string`.
 */
const PublicEnvSchema = v.object({
    PUBLIC_API_URL: v.pipe(v.string(), v.url()),
    PUBLIC_ENTRA_ID_TENANT_ID: v.pipe(v.string(), v.nonEmpty()),
    PUBLIC_ENTRA_ID_CLIENT_ID: v.pipe(v.string(), v.nonEmpty()),
    PUBLIC_ENTRA_ID_SCOPE: v.pipe(v.string(), v.nonEmpty()),
    PUBLIC_ENTRA_ID_REDIRECT_URI: v.pipe(v.string(), v.url()),
    PUBLIC_ENTRA_ID_LOGOUT_URI: v.pipe(v.string(), v.url()),
});

export type PublicEnv = v.InferOutput<typeof PublicEnvSchema>;

function parsePublicEnv(): PublicEnv {
    const result = v.safeParse(PublicEnvSchema, env);

    if (!result.success) {
        const details = result.issues
            .map((issue) => {
                const path =
                    issue.path
                        ?.map((segment) => String(segment.key))
                        .join(".") ?? "(unknown)";
                return `  - ${path}: ${issue.message}`;
            })
            .join("\n");

        throw new Error(
            `Invalid public environment configuration:\n${details}\n\n` +
                "Ensure every PUBLIC_* variable is set in the runtime environment.",
        );
    }

    return result.output;
}

/**
 * Validated, fully-typed public environment. Import this instead of reading
 * from `$env/dynamic/public` directly so consumers get guaranteed `string`
 * values and a single source of truth.
 */
export const publicEnv: PublicEnv = parsePublicEnv();
