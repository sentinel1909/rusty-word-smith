-- Migration 022: Remove plugin triggers
DROP TRIGGER IF EXISTS update_plugins_updated_at ON plugins;
DROP TRIGGER IF EXISTS update_custom_post_types_updated_at ON custom_post_types; 