use super::{output_name, update};
use iced_core::window::Id;

#[test]
fn output_names_follow_surface_lifetime() {
    let first = Id::unique();
    let second = Id::unique();
    update(first, Some("DP-1"));
    update(second, Some("DP-2"));
    assert_eq!(output_name(first).as_deref(), Some("DP-1"));
    update(first, Some("DP-3"));
    assert_eq!(output_name(first).as_deref(), Some("DP-3"));
    assert_eq!(output_name(second).as_deref(), Some("DP-2"));
    update(first, None);
    update(second, None);
    assert_eq!(output_name(first), None);
}
