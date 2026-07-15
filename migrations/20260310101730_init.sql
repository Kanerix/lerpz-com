CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Application role.
--
-- Backend services connect as this dedicated, non-owner login role rather than
-- the migration/owner role. Because it owns no objects, its privileges can be
-- restricted independently of migrations -- e.g. later migrations can make
-- catalog tables read-only for this role while migrations (run as the owner)
-- remain able to manage them.
--
-- NOTE: the password here is intended for local development only. Manage real
-- credentials via your secret store and rotate them as needed.
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'lerpz_app') THEN
        CREATE ROLE lerpz_app LOGIN PASSWORD 'Password123';
    END IF;
END
$$;

DO $$
BEGIN
    EXECUTE format('GRANT CONNECT ON DATABASE %I TO lerpz_app', current_database());
END
$$;

GRANT USAGE ON SCHEMA public TO lerpz_app;

ALTER DEFAULT PRIVILEGES IN SCHEMA public
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO lerpz_app;
ALTER DEFAULT PRIVILEGES IN SCHEMA public
    GRANT USAGE, SELECT ON SEQUENCES TO lerpz_app;
