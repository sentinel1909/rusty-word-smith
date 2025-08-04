-- Migration 021: Revert posts table changes
ALTER TABLE posts DROP COLUMN IF EXISTS post_type;
ALTER TABLE posts DROP COLUMN IF EXISTS custom_fields; 