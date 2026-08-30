mod http_route_manifest;
mod paths;
mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use routes::{build_catalog_app_router, respond_list, GamesListQuery};

use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_mount() -> HttpRouteManifest {
    gateway_route_manifest()
}
