-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP CONSTRAINT email_unique;