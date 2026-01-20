use std::net::SocketAddr;
use std::path::PathBuf;

use axum::Router;
use dav_server::{DavHandler, fakels::FakeLs, localfs::LocalFs};
use tower_http::cors::CorsLayer;

use crate::core::Result;

pub fn serve(port: u16, dir: &str) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async { serve_async(port, dir).await })
}

async fn serve_async(port: u16, dir: &str) -> Result<()> {
    let dir = PathBuf::from(dir).canonicalize()?;

    let dav = DavHandler::builder()
        .filesystem(LocalFs::new(dir.clone(), false, false, false))
        .locksystem(FakeLs::new())
        .build_handler();

    let app = Router::new()
        .fallback(move |req| {
            let dav = dav.clone();
            async move { dav.handle(req).await }
        })
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("WebDAV server listening on http://{addr}");
    println!("Serving: {}", dir.display());
    println!();
    println!("Mount with: mount -t davfs http://localhost:{port} /mnt/chatfiles");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
