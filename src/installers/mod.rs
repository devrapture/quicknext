pub mod tailwind;
pub mod installer;
pub mod app_router;
pub mod trpc;
pub mod next_auth;
pub mod dependency_versions;

pub use tailwind::TailwindConfig;
pub use app_router::AppRouterConfig;
pub use trpc::TrpcConfig;
pub use next_auth::NextAuthConfig;
pub use dependency_versions::get_dependency_version_map;
