pub mod http_route_manifest;
pub mod manifest;
pub mod paths;
pub mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use routes::build_leaderboard_app_router;

use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_mount() -> HttpRouteManifest {
    gateway_route_manifest()
}
