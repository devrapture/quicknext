pub mod app_router;
pub mod dependency_versions;
pub mod installer;
pub mod tailwind;

pub use app_router::AppRouterConfig;
pub use dependency_versions::get_dependency_version_map;
pub use tailwind::TailwindConfig;
