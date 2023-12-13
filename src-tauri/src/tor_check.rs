use arti_client::{config::TorClientConfigBuilder, TorClient};
use arti_hyper::ArtiHttpConnector;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Runtime};
use tls_api::{TlsConnector as TlsConnectorTrait, TlsConnectorBuilder};
use semver::Version;
use std::collections::HashMap;

#[cfg(not(target_vendor = "apple"))]
use tls_api_native_tls::TlsConnector;
#[cfg(target_vendor = "apple")]
use tls_api_openssl::TlsConnector;

#[derive(Debug, thiserror::Error)]
pub(crate) enum CheckError {
    #[error("Could not resolve app data or cache directory")]
    PathResolution,
    #[error(transparent)]
    ConfigBuildError(#[from] arti_client::config::ConfigBuildError),
    #[error(transparent)]
    ArtiError(#[from] arti_client::Error),
    #[error(transparent)]
    TlsError(#[from] anyhow::Error),
    #[error(transparent)]
    HttpError(#[from] hyper::Error),
    #[error(transparent)]
    DeserializationError(#[from] serde_json::Error),
    #[error("No Update Found")]
    NoUpdateFound
}

impl Serialize for CheckError {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

type Result<T> = std::result::Result<T, CheckError>;

#[derive(Serialize, Deserialize)]
pub struct Release {
    pub version: Version,
    pub notes: Option<String>,
    pub pub_date: Option<String>,
    pub platforms: HashMap<String, ReleaseManifest>
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub url: String,
    pub signature: String, 
}

#[tauri::command]
pub(crate) async fn check<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Release> {
    let Some(mut tor_cache_dir) = app.path_resolver().app_cache_dir() else {
        return Err(CheckError::PathResolution);
    };
    tor_cache_dir.push("/tor");
    let Some(mut tor_data_dir) = app.path_resolver().app_data_dir() else {
        return Err(CheckError::PathResolution);
    };
    tor_data_dir.push("/tor");

    let config = TorClientConfigBuilder::from_directories(tor_data_dir, tor_cache_dir).build()?;

    let tor_client = TorClient::create_bootstrapped(config).await?;

    let tls_connector = TlsConnector::builder()?.build()?;

    let tor_connector = ArtiHttpConnector::new(tor_client, tls_connector);

    let http = hyper::Client::builder().build::<_, hyper::Body>(tor_connector);

    for url in vec!["https://e253.github.io/cowsay/latest.json"] {
        let mut response = http.get(url.try_into().unwrap()).await?;
        let body = hyper::body::to_bytes(response.body_mut()).await?;
        println!("{}", String::from_utf8(body.to_vec()).unwrap());
        let update: Release = serde_json::from_str::<Release>(&String::from_utf8(body.to_vec()).unwrap())?;
        if update.version > app.package_info().version {
            return Ok(update);
        }
    }

    Err(CheckError::NoUpdateFound)
}