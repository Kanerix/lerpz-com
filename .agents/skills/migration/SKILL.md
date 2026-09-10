---
name: migration
description: Write a PostgreSQL migration for this repository, following the house SQL style and the sqlx offline cache workflow. Use when adding or changing a table, column, enum type, index or trigger, or when a SQL change means the .sqlx cache has to be regenerated.
---

# Writing a migration

Migrations live in the top-level `migrations/` directory and are applied by sqlx.
Read one or two existing files before writing a new one, for example
`migrations/20260310101737_chats.sql` and `migrations/20260720102822_user_settings.sql`.

## Rules

- Create the file with `just migration NAME`. Never hand-name it, the timestamp
  prefix has to match what sqlx expects. `NAME` is snake_case and describes the
  change, such as `user_settings` or `models_insert`.
- A merged migration is append-only. Fix a mistake with a new migration, never
  by editing one that has already been applied.
- Keywords are uppercase.

## SQL style

Columns are written in three aligned columns: name, type, then constraints. The
body is indented four spaces. Group related columns under a lowercase comment
label, and keep the timestamps last under `-- other`.

```sql
-- Conversations table
CREATE TABLE conversations (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- ownership
    user_id             VARCHAR(255)    NOT NULL,
    -- content
    title               VARCHAR(500),
    -- state
    archived            BOOLEAN         NOT NULL DEFAULT FALSE,
    -- other
    created_at          TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP
);
```

- Surrogate keys are `UUID PRIMARY KEY DEFAULT uuidv7()`. A table keyed by the
  identity provider's subject uses `user_id VARCHAR(255) PRIMARY KEY` instead.
- A user reference is always `VARCHAR(255)`, not a foreign key.
- Owned rows use `REFERENCES parent(id) ON DELETE CASCADE`.
- Index names are `idx_<table>_<column>`.
- Enums are Postgres types with lowercase values, declared above the table that
  uses them:

    ```sql
    -- Theme preference
    CREATE TYPE theme_pref AS ENUM ('light', 'dark', 'system');
    ```

- Any table with an `updated_at` column gets the trigger. The
  `update_timestamp()` function is already defined in the init migration.

    ```sql
    CREATE TRIGGER update_timestamp
        BEFORE UPDATE ON user_settings
        FOR EACH ROW
        EXECUTE FUNCTION update_timestamp();
    ```

- Put a comment above a table or type when its purpose is not obvious from the
  name. Explain what it backs or why it exists, not what the columns are.

## After writing the SQL

1. Start the database if it is not running: `just infra`.
2. Apply the migration: `just migrate`.
3. If any Rust query is affected, run `just prepare` and commit the regenerated
   `.sqlx` cache in the same commit. The workspace compiles offline, so a
   missing cache entry breaks the build for everyone else.

## Using the new schema from Rust

- Postgres enums map to a Rust type with
  `#[derive(sqlx::Type)] #[sqlx(type_name = "theme_pref", rename_all = "lowercase")]`.
- Selecting an enum column needs a type override in the query, such as
  `SELECT theme AS "theme: ThemePref" FROM user_settings`.
- Queries use the compile-time `query!`, `query_as!` and `query_scalar!` macros
  and are written where they are used.
