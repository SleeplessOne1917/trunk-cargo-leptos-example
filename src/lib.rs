pub mod counters;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::counters::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(Counters);
}
