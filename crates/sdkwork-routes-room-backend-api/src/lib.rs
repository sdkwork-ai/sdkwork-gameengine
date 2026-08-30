mod http_route_manifest;
mod manifest;
mod paths;
mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use manifest::{
    API_AUTHORITY, OWNER, PACKAGE_NAME, PREFIX, ROUTE_MANIFEST_PATH, SDK_FAMILY, SURFACE,
};
pub use paths::{
    BACKEND_GAMES_ROOMS_LIST_PATH, BACKEND_GAMES_ROOM_DETAIL_PATH,
    BACKEND_GAMES_ROOM_FORCE_CLOSE_PATH, BACKEND_GAMES_ROOM_SEATS_LIST_PATH,
};
pub use routes::build_room_backend_router;

use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    gateway_route_manifest()
}

pub fn gateway_mount() -> HttpRouteManifest {
    gateway_route_manifest()
}
