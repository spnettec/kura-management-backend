-- First-login password rotation: the seeded bootstrap administrator must change
-- the initial password before the console accepts further work from the session.
ALTER TABLE mgmt_users ADD COLUMN must_change_password boolean NOT NULL DEFAULT false;
