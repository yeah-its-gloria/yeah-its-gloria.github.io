use wasm_bindgen::JsCast;

pub(crate) fn toggle_drop_down_menu() {
    let menu = web_sys::window().unwrap()
                       .document().unwrap()
                       .get_element_by_id("header_small_dropdown_menu").unwrap()
                       .dyn_into::<web_sys::HtmlElement>().unwrap();

    let style = menu.style();
    if style.get_property_value("visibility").unwrap() != "visible" {
        style.set_property("visibility", "visible").unwrap();
        style.set_property("top", "4rem").unwrap(); // Number of buttons
        return;
    }

    style.set_property("visibility", "hidden").unwrap();
    style.set_property("top", "-16rem").unwrap(); // Number of buttons, squared
}


pub(crate) fn switch_page(page_name: &str) {
    let window = web_sys::window().unwrap();
    let element = window.document().unwrap().query_selector("head").unwrap().unwrap();

    if element.get_attribute("data-current-page").unwrap() == page_name {
        toggle_drop_down_menu();
        return;
    }

    let location = window.location();

    match page_name {
        "main"     => location.set_href("/Main.html").unwrap(),
        "projects" => location.set_href("/Projects.html").unwrap(),
        "cell"     => location.set_href("/Cell.html").unwrap(),
        "random"   => location.set_href("/RandomStuff.html").unwrap(),

        &_         => panic!("Invalid location given")
    }
}
