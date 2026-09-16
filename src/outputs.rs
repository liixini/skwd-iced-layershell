use iced_core::window::Id;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static OUTPUTS: LazyLock<Mutex<HashMap<Id, String>>> = LazyLock::new(Mutex::default);

pub fn output_name(id: Id) -> Option<String> {
    OUTPUTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&id)
        .cloned()
}

pub(crate) fn update(id: Id, name: Option<&str>) {
    let mut outputs = OUTPUTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(name) = name {
        if outputs.get(&id).is_none_or(|current| current != name) {
            outputs.insert(id, name.to_owned());
        }
    } else {
        outputs.remove(&id);
    }
}

#[cfg(test)]
mod tests;
