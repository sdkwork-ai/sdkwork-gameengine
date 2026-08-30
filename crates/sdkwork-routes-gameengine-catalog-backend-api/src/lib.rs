mod http_route_manifest;
mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use routes::build_catalog_backend_router;

use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    gateway_route_manifest()
}

pub fn gateway_mount() -> HttpRouteManifest {
    gateway_route_manifest()
}
