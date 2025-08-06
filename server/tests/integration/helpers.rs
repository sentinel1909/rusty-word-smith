// server/tests/integration/helpers.rs

// dependencies
use app::configuration::{StaticServer, TemplateEngine};
use pavex::{config::ConfigLoader, server::Server};
use server::configuration::Profile;
use server_sdk::{ApplicationConfig, ApplicationState, run};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use tracing::subscriber::set_global_default;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

/// A test user struct for integration tests
pub struct TestUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
}

impl TestUser {
    /// Create a new test user with valid default values
    pub fn new() -> Self {
        Self {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        }
    }

    /// Create a test user with a unique username and email (useful for parallel tests)
    pub fn unique() -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            username: format!("user_{}", id).replace('-', "_"),
            email: format!("user_{}@example.com", id),
            password: "password123".to_string(),
            display_name: format!("User {}", id),
        }
    }

    /// Convert to JSON body for API requests
    pub fn to_json_body(&self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "username": self.username,
            "email": self.email,
            "password": self.password,
        });

        if !self.display_name.is_empty() {
            json["display_name"] = serde_json::Value::String(self.display_name.clone());
        }

        json
    }
}

impl Default for TestUser {
    fn default() -> Self {
        Self::new()
    }
}

/// helper function to configure an independent database for each test, so that teste are isolated
async fn configure_database(config: &ApplicationConfig) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.databaseconfig.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                config.databaseconfig.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.databaseconfig.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("../shuttle/migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

/// A convenient struct for calling the API under test.
pub struct TestApi {
    pub api_address: String,
    pub api_client: reqwest::Client,
    pub _api_db_pool: PgPool,
}

/// Convenient methods for calling the API under test.
impl TestApi {
    pub async fn spawn() -> Self {
        Self::init_telemetry();

        let mut config = Self::get_config_with_absolute_paths();
        config.databaseconfig.database_name = Uuid::new_v4().to_string();
        configure_database(&config).await;
        Self::spawn_with_config(config).await
    }

    /// Spawn the server with the given configuration.
    async fn spawn_with_config(config: ApplicationConfig) -> Self {
        let tcp_listener = config
            .server
            .listener()
            .await
            .expect("Failed to bind the server TCP listener");
        let address = tcp_listener
            .local_addr()
            .expect("The server TCP listener doesn't have a local socket address");
        let server_builder = Server::new().listen(tcp_listener);
        let api_address = format!("http://{}:{}", config.server.ip, address.port());

        // build the template engine and static server with proper error handling
        let template_engine = TemplateEngine::from_config(&config.templateconfig)
            .expect("Failed to build template engine");
        let static_server = StaticServer::from_config(config.staticserverconfig.clone());
        let db_pool = config.databaseconfig.get_database_pool().await;

        let application_state =
            ApplicationState::new(config, db_pool.clone(), template_engine, static_server)
                .await
                .expect("Failed to build the application state");

        tokio::spawn(async move { run(server_builder, application_state).await });

        TestApi {
            api_address,
            api_client: reqwest::Client::new(),
            _api_db_pool: db_pool,
        }
    }

    /// Load the test configuration with absolute paths for cross-platform compatibility.
    fn get_config_with_absolute_paths() -> ApplicationConfig {
        let config: ApplicationConfig = ConfigLoader::new()
            .profile(Profile::Test)
            .load()
            .expect("Failed to load test configuration");

        // Validate that the test directories exist
        let template_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("test_dir/templates");
        let static_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("test_dir/static");

        if !template_dir.exists() {
            panic!(
                "Test templates directory does not exist: {}",
                template_dir.display()
            );
        }
        if !static_dir.exists() {
            panic!(
                "Test static directory does not exist: {}",
                static_dir.display()
            );
        }

        config
    }

    fn init_telemetry() {
        // Initialize the telemetry setup at most once.
        static INIT_TELEMETRY: Once = Once::new();
        INIT_TELEMETRY.call_once(|| {
            // Only enable the telemetry if the `TEST_LOG` environment variable is set.
            if std::env::var("TEST_LOG").is_ok() {
                let subscriber = tracing_subscriber::fmt::Subscriber::builder()
                    .with_env_filter(
                        EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")),
                    )
                    .finish();
                // We don't redirect panic messages to the `tracing` subsystem because
                // we want to see them in the test output.
                set_global_default(subscriber).expect("Failed to set a `tracing` global subscriber")
            }
        });
    }
}

/// Convenient methods for calling the API under test.
impl TestApi {
    pub async fn get_ping(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/ping", &self.api_address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_index(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/", &self.api_address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_static_file(&self, filename: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/static/{}", &self.api_address, filename))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_register(&self, user: &crate::helpers::TestUser) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/auth/register", &self.api_address))
            .json(&user.to_json_body())
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
