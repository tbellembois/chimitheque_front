use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

pub fn download_csv(csv: &String, filename: &str) {
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(&csv));

    let options = BlobPropertyBag::new();
    options.set_type("text/csv;charset=utf-8");

    let blob = Blob::new_with_str_sequence_and_options(&parts, &options).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();

    let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();

    a.set_href(&url);
    a.set_download(filename);
    a.click();

    Url::revoke_object_url(&url).unwrap();
}
