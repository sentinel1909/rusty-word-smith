-- Migration 017: Create custom_post_types table
CREATE TABLE custom_post_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL, -- e.g., 'product', 'event', 'portfolio'
    label VARCHAR(100) NOT NULL, -- Display name
    description TEXT,
    plugin_slug VARCHAR(100) REFERENCES plugins(slug) ON DELETE CASCADE,
    is_public BOOLEAN NOT NULL DEFAULT true,
    supports_comments BOOLEAN NOT NULL DEFAULT false,
    supports_media BOOLEAN NOT NULL DEFAULT true,
    menu_icon VARCHAR(100), -- Icon for admin menu
    menu_position INTEGER,
    settings JSONB, -- Custom post type configuration
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_custom_post_types_name ON custom_post_types(name);
CREATE INDEX idx_custom_post_types_plugin ON custom_post_types(plugin_slug); 