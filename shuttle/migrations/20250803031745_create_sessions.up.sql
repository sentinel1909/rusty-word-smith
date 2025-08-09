-- Create the sessions table if it doesn’t exist
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    deadline TIMESTAMPTZ NOT NULL,
    state JSONB NOT NULL
);

-- Create the index on the deadline column if it doesn’t exist
CREATE INDEX IF NOT EXISTS idx_sessions_deadline ON sessions(deadline);
