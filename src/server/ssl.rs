use actix_web::rt::spawn;
use futures_util::StreamExt;
use rustls_acme::caches::DirCache;
use rustls_acme::futures_rustls::rustls::ServerConfig;
use rustls_acme::AcmeConfig;
use std::env;
use std::sync::Arc;

pub async fn ssl() -> ServerConfig {
    let mut state = AcmeConfig::new(["luxalpa.com"])
        .contact(["mailto:acme.lxc@luxalpa.com"])
        .cache(DirCache::new("acme"))
        .directory_lets_encrypt(
            env::var("USE_PROD_TLS")
                .map(|s| s == "1")
                .unwrap_or_default(),
        )
        .state();
    let rustls_config = Arc::into_inner(state.challenge_rustls_config()).unwrap();

    spawn(async move {
        loop {
            match state.next().await.unwrap() {
                Ok(ok) => log::info!("event: {:?}", ok),
                Err(err) => {
                    log::error!("error: {:?}", err);
                }
            }
        }
    });

    rustls_config
}
