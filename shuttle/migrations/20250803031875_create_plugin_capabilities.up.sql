-- Migration 020: Create plugin_capabilities table
CREATE TABLE plugin_capabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plugin_slug VARCHAR(100) NOT NULL REFERENCES plugins(slug) ON DELETE CASCADE,
    capability VARCHAR(100) NOT NULL,
    description TEXT,
    default_roles TEXT[], -- Array of roles that get this capability by default
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(plugin_slug, capability)
);

CREATE INDEX idx_plugin_capabilities_plugin ON plugin_capabilities(plugin_slug);
CREATE INDEX idx_plugin_capabilities_capability ON plugin_capabilities(capability); 