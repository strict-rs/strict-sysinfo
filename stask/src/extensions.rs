//! Consumer-owned registry for `just x <name>` commands.

/// Build this repository's intentionally empty extension registry.
///
/// # Errors
///
/// Returns a typed registration error if the controlled `x` router metadata
/// is invalid.
pub fn commands() -> template_stask::RegistryResult<()> {
    template_stask::empty_registry("sysinfo extensions")
}
