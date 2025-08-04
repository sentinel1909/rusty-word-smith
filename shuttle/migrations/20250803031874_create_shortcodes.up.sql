-- Migration 019: Create shortcodes table
CREATE TABLE shortcodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plugin_slug VARCHAR(100) REFERENCES plugins(slug) ON DELETE CASCADE,
    tag VARCHAR(50) UNIQUE NOT NULL, -- e.g., 'gallery', 'contact_form'
    handler_function VARCHAR(200) NOT NULL,
    description TEXT,
    example_usage TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_shortcodes_tag ON shortcodes(tag);
CREATE INDEX idx_shortcodes_plugin ON shortcodes(plugin_slug); 