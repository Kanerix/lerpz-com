---
name: migration
description: Write a PostgreSQL migration and refresh the sqlx cache. Use for table, column, type, index and trigger changes.
---

# Writing a migration

`AGENTS.md` covers where migrations live and when one may still be edited. This
file covers the SQL style and what to run afterwards. Read one or two existing
migrations before writing a new one.

## Rules

- Create the file with `just migration NAME`. Never hand-name it, the timestamp
  prefix has to match what sqlx expects. `NAME` is snake_case and describes the
  change, such as `user_settings` or `models_insert`.
- Editing a migration is only an option while yours is the only database that
  has run it, because `just reset` deletes the data volumes and the next
  `just infra` plus `just migrate` replays every migration from scratch. Once it
  has gone further, or nobody is sure, correct it with a new migration.
- Keywords are uppercase.

## SQL style

Columns are written in three aligned columns: name, type, then constraints. The
body is indented four spaces. Group related columns under a lowercase label, and
keep the timestamps last under `-- other`.

```sql
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

- A table or type earns a comment when the name does not carry its purpose. Say
  what it backs or why it exists, not what the columns are and not that it is a
  table.

    ```sql
    -- One row per user, keyed by the identity provider's subject. Backs the
    -- account settings surface.
    CREATE TABLE user_settings (
    ```

## After writing the SQL

1. Start the database if it is not running: `just infra`.
2. Apply the migration: `just migrate`.
3. If any Rust query is affected, run `just prepare` and commit the regenerated
   `.sqlx` cache in the same commit.

## Using the new schema from Rust

- Postgres enums map to a Rust type with
  `#[derive(sqlx::Type)] #[sqlx(type_name = "theme_pref", rename_all = "lowercase")]`.
- Selecting an enum column needs a type override in the query, such as
  `SELECT theme AS "theme: ThemePref" FROM user_settings`.
