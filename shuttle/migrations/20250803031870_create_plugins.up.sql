-- Migration 015: Create plugins table
CREATE TABLE plugins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    version VARCHAR(20) NOT NULL,
    description TEXT,
    author VARCHAR(100),
    author_url VARCHAR(500),
    plugin_url VARCHAR(500),
    is_active BOOLEAN NOT NULL DEFAULT false,
    auto_update BOOLEAN NOT NULL DEFAULT false,
    settings JSONB, -- Plugin-specific configuration
    installed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_plugins_slug ON plugins(slug);
CREATE INDEX idx_plugins_active ON plugins(is_active) WHERE is_active = true; 