use wasm_bindgen::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};

use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let filereader = FileReader::new().unwrap().dyn_into::<FileReader>()?;

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let data = element.result().unwrap();
        let js_data = js_sys::Uint8Array::from(data);
        let rust_str: String = js_data.to_string().into();
        log(rust_str.as_str());
    }) as Box<dyn FnMut(_)>);

    filereader.set_onloadend(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    let fileinput: HtmlInputElement = document
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;
    fileinput.set_type("file");

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let filelist = element.files().unwrap();

        let file = filelist.get(0).unwrap();

        filereader.read_as_text(&file).unwrap();
        //log(filelist.length().to_string().as_str());
    }) as Box<dyn FnMut(_)>);
    fileinput.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    body.append_child(&fileinput)?;

    Ok(())
}
