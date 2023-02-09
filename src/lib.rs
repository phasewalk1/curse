mod helpers;
pub mod init_prelude
{
    #[cfg(target_os = "linux")]
    pub use super::helpers::init::linux_install_deps;
    #[cfg(target_os = "macos")]
    pub use super::helpers::init::macos_install_deps;
}
