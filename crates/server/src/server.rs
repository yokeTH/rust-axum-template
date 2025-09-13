use crate::config::Config;
use axum::Router;
use log::info;
use std::net::SocketAddr;
use tokio::signal;

pub struct Server {
    pub config: Config,
    pub router: Router,
}

impl Server {
    pub async fn run(self) -> anyhow::Result<()> {
        let addr: SocketAddr = self.config.get_server_url().parse()?;
        let listener = tokio::net::TcpListener::bind(addr).await?;

        info!("Server running on {}", addr);

        axum::serve(listener, self.router)
            .with_graceful_shutdown(async {
                signal::ctrl_c()
                    .await
                    .expect("failed to listen for shutdown signal");
            })
            .await?;

        Ok(())
    }
}
