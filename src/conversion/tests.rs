use super::mouse_interaction;
use iced_core::mouse::Interaction;

#[test]
fn idle_uses_the_default_cursor() {
    assert_eq!(mouse_interaction(Interaction::Idle), "default");
    assert_eq!(mouse_interaction(Interaction::None), "default");
}

#[test]
fn busy_and_clickable_controls_keep_their_cursors() {
    assert_eq!(mouse_interaction(Interaction::Wait), "wait");
    assert_eq!(mouse_interaction(Interaction::Pointer), "pointer");
    assert_eq!(mouse_interaction(Interaction::Text), "text");
}
