use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response};

pub(crate) async fn load() {
    let window   = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let list     = document.get_element_by_id("project_list").unwrap().dyn_into::<web_sys::HtmlTableElement>().unwrap();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://api.github.com/users/yeah-its-gloria/repos", &opts).unwrap();
    request.headers().set("Accept", "application/vnd.github.v3+json").unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    assert!(resp_value.is_instance_of::<Response>());

    let resp = resp_value.dyn_into::<Response>().unwrap();
    if resp.status() != 200 {
        return;
    }

    let text = JsFuture::from(resp.text().unwrap()).await.unwrap().as_string().unwrap();
    let projects: Value = serde_json::from_str(text.as_str()).unwrap();

    for project in projects.as_array().unwrap() {
        let entry                            = document.create_element("tr").unwrap().dyn_into::<web_sys::HtmlTableRowElement>().unwrap();
        let project_element                  = document.create_element("td").unwrap().dyn_into::<web_sys::HtmlTableCellElement>().unwrap();
        let project_element_link             = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        let project_element_link_image       = document.create_element("img").unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();

        let mut name = " ".to_owned() + project["name"].as_str().unwrap();
        if project["fork"].as_bool().unwrap() {
            name += " - fork";
        }

        project_element_link.set_inner_text(&name);
        project_element_link.set_href(project["html_url"].as_str().unwrap());
        project_element_link.set_target("_blank");

        project_element_link_image.set_src("/Content/GitHub.png");
        project_element_link_image.set_alt("GitHub");
        project_element_link_image.set_class_name("utilities_icon");

        project_element.append_child(&project_element_link_image).unwrap();
        project_element.append_child(&project_element_link).unwrap();

        entry.append_child(&project_element).unwrap();

        let description = document.create_element("td").unwrap().dyn_into::<web_sys::HtmlTableCellElement>().unwrap();
        description.set_inner_text(project["description"].as_str().unwrap_or("-"));
        entry.append_child(&description).unwrap();

        let language = document.create_element("td").unwrap().dyn_into::<web_sys::HtmlTableCellElement>().unwrap();
        language.set_inner_text(project["language"].as_str().unwrap_or("-"));
        entry.append_child(&language).unwrap();

        list.append_child(&entry).unwrap();
    }
}
