# scripts/init_db.ps1

# Enable verbose output and stop on errors
$VerbosePreference = "Continue"
$ErrorActionPreference = "Stop"

# Function to check if a command exists
function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    }
    catch {
        return $false
    }
}

# Check if sqlx is installed
if (-not (Test-Command "sqlx")) {
    Write-Error "Error: sqlx is not installed."
    Write-Error "Use:"
    Write-Error "    cargo install --version='~0.8.4 sqlx-cli --no-default-features --features rustls,postgres"
    Write-Error "to install it."
    exit 1
}

# Check if a custom user has been set, otherwise default to 'postgres'
$DB_USER = if ($env:POSTGRES_USER) { $env:POSTGRES_USER } else { "postgres" }
# Check if a custom password has been set, otherwise default to 'password'
$DB_PASSWORD = if ($env:POSTGRES_PASSWORD) { $env:POSTGRES_PASSWORD } else { "password" }
# Check if a custom database name has been set, otherwise default to 'rusty_word_smith_org'
$DB_NAME = if ($env:POSTGRES_DB) { $env:POSTGRES_DB } else { "rusty_word_smith_org" }
# Check if a custom port has been set, otherwise default to '5432'
$DB_PORT = if ($env:POSTGRES_PORT) { $env:POSTGRES_PORT } else { "5432" }
# Check if a custom host has been set, otherwise default to 'localhost'
$DB_HOST = if ($env:POSTGRES_HOST) { $env:POSTGRES_HOST } else { "localhost" }

# Allow to skip Docker if a dockerized Postgres database is already running
if (-not $env:SKIP_DOCKER) {
    Write-Verbose "Starting PostgreSQL Docker container..."
    docker run -e POSTGRES_USER=$DB_USER -e POSTGRES_PASSWORD=$DB_PASSWORD -e POSTGRES_DB=$DB_NAME -p "${DB_PORT}:5432" -d postgres postgres -N 1000
}

# Keep pinging Postgres until it's ready to accept commands
$env:PGPASSWORD = $DB_PASSWORD

Write-Verbose "Waiting for PostgreSQL to be ready..."
do {
    $postgresReady = $false
    try {
        # Test connection using sqlx database create (which will fail gracefully if DB exists)
        $testUrl = "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/postgres"
        $env:DATABASE_URL = $testUrl
        $result = sqlx database create 2>&1
        if ($LASTEXITCODE -eq 0 -or $result -like "*already exists*") {
            $postgresReady = $true
        }
    }
    catch {
        # Connection failed, continue waiting
    }
    
    if (-not $postgresReady) {
        Write-Verbose "PostgreSQL is still unavailable - sleeping"
        Start-Sleep -Seconds 1
    }
} while (-not $postgresReady)

Write-Verbose "PostgreSQL is up and running on port $DB_PORT - running migrations now!"

$DATABASE_URL = "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
$env:DATABASE_URL = $DATABASE_URL

Write-Verbose "Creating database..."
sqlx database create

Write-Verbose "Running migrations..."
sqlx migrate run --source shuttle/migrations

Write-Verbose "PostgreSQL has been migrated, ready to go!" 