// shuttle/src/main.rs

// module dependencies
mod shuttle_pavex;
mod startup;

// dependencies
use pavex::server::Server;
use shuttle_runtime::{SecretStore, Secrets};
use shuttle_shared_db::Postgres;
use sqlx::PgPool;
use startup::{
    build_application_state, determine_profile, load_application_config,
    run_migrations, setup_components, setup_telemetry,
};

// main function
#[shuttle_runtime::main]
async fn shuttle_pavex(
    #[Postgres] pool: PgPool,
    #[Secrets] secrets: SecretStore,
) -> shuttle_pavex::ShuttlePavex {
    setup_telemetry()?;
    run_migrations(&pool).await?;
    let app_profile = determine_profile(&secrets)?;
    let app_config = load_application_config(app_profile)?;
    let (template_engine, static_server) = setup_components(&app_config)?;
    let app_state =
        build_application_state(app_config, template_engine, static_server, pool).await?;

    let app_server = Server::new();
    let shuttle_px = shuttle_pavex::PavexService(app_server, app_state);

    Ok(shuttle_px)
}