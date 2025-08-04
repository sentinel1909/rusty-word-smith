-- Migration 018: Create plugin_pages table (for admin pages)
CREATE TABLE plugin_pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plugin_slug VARCHAR(100) NOT NULL REFERENCES plugins(slug) ON DELETE CASCADE,
    page_title VARCHAR(100) NOT NULL,
    menu_title VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    parent_slug VARCHAR(100), -- For submenu pages
    capability VARCHAR(50) NOT NULL DEFAULT 'manage_options', -- Required permission
    icon VARCHAR(100),
    position INTEGER,
    template_path VARCHAR(255), -- Path to template file
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(plugin_slug, slug)
);

CREATE INDEX idx_plugin_pages_plugin ON plugin_pages(plugin_slug);
CREATE INDEX idx_plugin_pages_parent ON plugin_pages(parent_slug); 