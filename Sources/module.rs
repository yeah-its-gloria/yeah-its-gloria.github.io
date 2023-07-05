use wasm_bindgen::prelude::wasm_bindgen;

mod github;
mod page_control;

#[wasm_bindgen]
pub fn header_button_main() {
    page_control::switch_page("main");
}

#[wasm_bindgen]
pub fn header_button_projects() {
    page_control::switch_page("projects");
}

#[wasm_bindgen]
pub fn header_button_cell() {
    page_control::switch_page("cell");
}

#[wasm_bindgen]
pub fn header_button_random() {
    page_control::switch_page("random");
}

#[wasm_bindgen]
pub fn header_button_toggle_drop_down_menu() {
    page_control::toggle_drop_down_menu();
}

#[wasm_bindgen]
pub async fn projects_load() {
    github::load().await;
}
