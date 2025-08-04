-- Migration 016: Create plugin_hooks table
CREATE TABLE plugin_hooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plugin_slug VARCHAR(100) NOT NULL REFERENCES plugins(slug) ON DELETE CASCADE,
    hook_name VARCHAR(100) NOT NULL, -- e.g., 'post_created', 'user_login', 'page_render'
    callback_function VARCHAR(200) NOT NULL, -- Function to call
    priority INTEGER NOT NULL DEFAULT 10, -- Execution order
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_plugin_hooks_name ON plugin_hooks(hook_name);
CREATE INDEX idx_plugin_hooks_plugin ON plugin_hooks(plugin_slug);
CREATE INDEX idx_plugin_hooks_priority ON plugin_hooks(hook_name, priority); 