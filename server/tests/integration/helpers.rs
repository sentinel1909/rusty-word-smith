// server/tests/integration/helpers.rs

// dependencies
use app::configuration::{DatabaseConfig, StaticServer, TemplateEngine};
use pavex::{config::ConfigLoader, server::Server};
use server::configuration::Profile;
use server_sdk::{ApplicationConfig, ApplicationState, run};
use std::sync::Once;
use tracing::subscriber::set_global_default;
use tracing_subscriber::EnvFilter;

pub struct TestApi {
    pub api_address: String,
    pub api_client: reqwest::Client,
}

impl TestApi {
    pub async fn spawn() -> Self {
        Self::init_telemetry();
        
        let config = Self::get_config_with_absolute_paths();
        
        Self::spawn_with_config(config).await
    }

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
        let db_pool = DatabaseConfig::get_database_pool(&config.databaseconfig).await;

        let application_state = ApplicationState::new(config, template_engine, static_server, db_pool)
            .await
            .expect("Failed to build the application state");

        tokio::spawn(async move { run(server_builder, application_state).await });

                TestApi {
            api_address,
            api_client: reqwest::Client::new(),
        }
    }



    /// Load the test configuration with absolute paths for cross-platform compatibility.
    fn get_config_with_absolute_paths() -> ApplicationConfig {
        let workspace_root = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("..")
            .canonicalize()
            .expect("Failed to canonicalize workspace root path");

        let mut config: ApplicationConfig = ConfigLoader::new()
            .profile(Profile::Test)
            .load()
            .expect("Failed to load test configuration");

        // Override the paths with absolute paths for testing
        let template_dir = workspace_root.join("test_data/templates").to_string_lossy().to_string();
        let static_dir = workspace_root.join("test_data/static");

        config.templateconfig.dir = template_dir.into();
        config.staticserverconfig.root_dir = static_dir;

        // Debug: Print the template directory path and current directory
        println!("Current directory: {:?}", std::env::current_dir().unwrap());
        println!("Template directory: {}", config.templateconfig.dir);
        println!("Static directory: {:?}", config.staticserverconfig.root_dir);
        
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




}
