use gloo_utils::document;
use js_sys::Reflect;
use log::error;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use yew::{
    function_component, html, use_effect_with_deps, use_state, virtual_dom::AttrValue, Callback,
    Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub site_key: AttrValue,
    pub on_load: Option<Callback<()>>,
}

#[function_component]
pub fn HCaptcha(props: &Props) -> Html {
    let loaded = use_state(|| false);
    use_effect_with_deps(
        move |on_load| {
            if !*loaded {
                if let Err(e) = inject_script(on_load) {
                    error!("{:?}", e);
                }
            }
            loaded.set(true);
            || ()
        },
        props.on_load.clone(),
    );
    html! {
        <>
            <div class="h-captcha" data-sitekey={props.site_key.to_string()} data-theme="dark"></div>
        </>
    }
}

fn inject_script(on_load: &Option<Callback<()>>) -> Result<(), JsValue> {
    let hcaptcha_loaded = Closure::wrap(Box::new({
        let on_load = on_load.clone();
        move || {
            if let Some(on_load) = &on_load {
                on_load.emit(());
            }
        }
    }) as Box<dyn FnMut()>);
    Reflect::set(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from("hCaptchaLoaded"),
        hcaptcha_loaded.as_ref().unchecked_ref(),
    )?;
    hcaptcha_loaded.forget();
    let script = document().create_element("script").unwrap();
    script.set_attribute("async", "true")?;
    script.set_attribute("defer", "true")?;
    script.set_attribute(
        "src",
        "https://js.hcaptcha.com/1/api.js?hl=en&onload=hCaptchaLoaded",
    )?;
    script.set_attribute("type", "text/javascript")?;
    let body = document()
        .body()
        .ok_or(JsValue::from_str("Can't find body"))?;
    body.append_child(&script)?;
    Ok(())
}
