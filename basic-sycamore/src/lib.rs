use sycamore::{
    builder::html::*,
    prelude::{component, Signal, View},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Clone)]
struct FieldInfo {
    title: String,
    value: Signal<String>,
    // Vec<(value, title)>
    options: Option<Vec<(String, String)>>,
}

#[component(MyRadiobutton<G>)]
fn render(info: FieldInfo) -> View<G> {
    let radiobutton_value = info.value.clone();

    div()
        .child(h2().text(info.title.clone()).build())
        .child(
            form()
                .child(View::new_fragment(
                    info.options
                        .unwrap()
                        .iter()
                        .map(|(value, title)| {
                            div()
                                .child(
                                    input()
                                        .id(value)
                                        .attr("type", "radio")
                                        .attr("value", value)
                                        .attr("name", info.title.clone())
                                        .build(),
                                )
                                .child(label().attr("for", value).text(title).build())
                                .build()
                        })
                        .collect(),
                ))
                .bind_value(radiobutton_value.clone())
                .on("change", move |_| {
                    log((*radiobutton_value.get()).to_string().as_str())
                })
                .build(),
        )
        .build()
}

#[component(MyDropdown<G>)]
fn render(info: FieldInfo) -> View<G> {
    let dropdown_value = info.value.clone();
    div()
        .child(h2().text(info.title).build())
        .child(
            select()
                .child(View::new_fragment(
                    info.options
                        .unwrap()
                        .iter()
                        .map(|(value, title)| option().attr("value", value).text(title).build())
                        .collect(),
                ))
                .bind_value(info.value)
                .on("change", move |_| {
                    log((*dropdown_value.get()).to_string().as_str())
                })
                .build(),
        )
        .build()
}

#[wasm_bindgen(start)]
pub fn run_app() {
    let info = FieldInfo {
        title: "Some dropdown".to_string(),
        value: Signal::new(String::new()),
        options: Some(vec![
            ("1".to_string(), "apples".to_string()),
            ("2".to_string(), "oranges".to_string()),
            ("3".to_string(), "peaches".to_string()),
            ("4".to_string(), "pinepapples".to_string()),
        ]),
    };
    sycamore::render(|| component::<_, MyDropdown<_>>(info));

    let info = FieldInfo {
        title: "Some Radio buttons".to_string(),
        value: Signal::new(String::new()),
        options: Some(vec![
            ("1".to_string(), "apples".to_string()),
            ("2".to_string(), "oranges".to_string()),
            ("3".to_string(), "peaches".to_string()),
            ("4".to_string(), "pinepapples".to_string()),
        ]),
    };
    sycamore::render(|| component::<_, MyRadiobutton<_>>(info));

    sycamore::render(|| div().attr("class", "class1 class2").build());
}
