-- Migration 021: Extend posts table for custom post types
ALTER TABLE posts ADD COLUMN post_type VARCHAR(50) NOT NULL DEFAULT 'post';
ALTER TABLE posts ADD COLUMN custom_fields JSONB;

CREATE INDEX idx_posts_type ON posts(post_type); 