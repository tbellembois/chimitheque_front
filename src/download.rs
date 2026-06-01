use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

pub fn download_csv(csv: &str, filename: &str) {
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(csv));

    let options = BlobPropertyBag::new();
    options.set_type("text/csv;charset=utf-8");

    if let Ok(blob) = Blob::new_with_str_sequence_and_options(&parts, &options)
        && let Ok(url) = Url::create_object_url_with_blob(&blob)
        && let Some(document) = web_sys::window().and_then(|w| w.document())
        && let Ok(a) = document.create_element("a")
    {
        // Ignoring result.
        let _ = a.dyn_into().map(|a: HtmlAnchorElement| {
            a.set_href(&url);
            a.set_download(filename);
            a.click();

            Url::revoke_object_url(&url).ok();
        });
    }
}
