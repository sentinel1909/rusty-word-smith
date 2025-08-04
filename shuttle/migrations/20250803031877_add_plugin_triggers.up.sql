-- Migration 022: Add plugin triggers
CREATE TRIGGER update_plugins_updated_at 
    BEFORE UPDATE ON plugins 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_custom_post_types_updated_at 
    BEFORE UPDATE ON custom_post_types 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column(); 