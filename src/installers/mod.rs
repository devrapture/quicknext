pub mod tailwind;
pub mod installer;
pub mod app_router;
pub mod dependency_versions;

pub use tailwind::TailwindConfig;
pub use app_router::AppRouterConfig;
pub use dependency_versions::get_dependency_version_map;
