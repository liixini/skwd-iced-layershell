use iced_graphics::compositor::SurfaceError;

use super::{SurfaceRecovery, configurable_surface, surface_recovery};

#[test]
fn lost_recreates() {
    assert_eq!(
        surface_recovery(&SurfaceError::Lost),
        SurfaceRecovery::Recreate
    );
}

#[test]
fn outdated_reconfigures() {
    assert_eq!(
        surface_recovery(&SurfaceError::Outdated),
        SurfaceRecovery::Reconfigure
    );
}

#[test]
fn transient_retries() {
    assert_eq!(
        surface_recovery(&SurfaceError::Timeout),
        SurfaceRecovery::Retry
    );
    assert_eq!(
        surface_recovery(&SurfaceError::Other),
        SurfaceRecovery::Retry
    );
}

#[test]
fn oom_aborts() {
    assert_eq!(
        surface_recovery(&SurfaceError::OutOfMemory),
        SurfaceRecovery::Abort
    );
}

#[test]
fn zero_size_not_configurable() {
    assert!(!configurable_surface(0, 0));
    assert!(!configurable_surface(1, 0));
    assert!(!configurable_surface(0, 1));
    assert!(configurable_surface(1, 1));
}
