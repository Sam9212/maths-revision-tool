use yew::prelude::*;
use chrono::Utc;
use web_sys::{
    HtmlInputElement,
    wasm_bindgen::JsCast,
};

#[derive(Properties, PartialEq, Clone)]
pub struct LengthValidationInputProps {
    #[prop_or(Callback::from(|_| ()))]
    pub onchange: Callback<Event>,
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    pub children: Children,
    pub id: AttrValue,
    pub required: bool,
    pub min_length: usize,
    pub max_length: usize,
    #[prop_or(Classes::new())]
    pub class: Classes,
    #[prop_or(None)]
    pub valid: Option<UseStateHandle<bool>>,
    #[prop_or(None)]
    pub valid_reason: Option<UseStateHandle<String>>,
}

#[function_component(LengthValidationInput)]
pub fn length_validation_input(props: &LengthValidationInputProps) -> Html {
    let LengthValidationInputProps { min_length, max_length, id, input_type, children, required, onchange, class, valid, valid_reason, } = props.clone();
    let valid = if let Some(valid) = valid { *valid } else { true };
    let valid_reason = if let Some(valid_reason) = valid_reason { (*valid_reason).clone() } else { String::new() };
    let mut label_style = String::new();
    let mut input_style = String::new();
    if !valid {
        label_style.push_str("color: var(--err);");
        input_style.push_str("border: 4px solid var(--err);");
    } else {
        label_style = String::new();
        input_style = String::new();
    }
    /*
        label:has(+ input[valid="false"]:not([type="date"])) {
            color: var(--err);
        }

        input[valid="false"]:not([type="date"]) {
            border: 4px solid var(--err);
        }
    */
    let maxlength = max_length.to_string();
    
    let onfocusout = move |e: FocusEvent| {
        let ele = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(ele) = ele {
            let len = ele.value().len();
            if len < min_length || len > max_length {
                ele.set_attribute("valid", "false").expect("error setting valid status");
            } else {
                ele.set_attribute("valid", "true").expect("error setting valid status");
            }

            if len == 0 {
                ele.set_attribute("empty", "true").expect("error setting valid status");
            } else {
                ele.set_attribute("empty", "false").expect("error setting valid status");
            }
        }
    };

    html!{
        <div {class}>
            <label style={label_style} for={id.clone()}>{ children }</label>
            <input style={input_style} type={input_type} name={id.clone()} {id} {required} {maxlength} {onchange} {onfocusout}/>
            if !valid {
                <p style={"font-size: 0.75rem;"} class={classes!("margin-bottom-1rem")}>{ valid_reason }</p>
            }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct DateInputProps {
    pub class: Classes,
    pub children: Children,
    pub id: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(DateInput)]
pub fn date_input(props: &DateInputProps) -> Html {
    let DateInputProps { class, children, id, onchange } = props.clone();

    let now = Utc::now();
    let now = now.format("%Y-%m-%d");

    html!{
        <div {class}>
            <label for={id.clone()}>{ children }</label>
            <input type={"date"} {onchange} name={id.clone()} {id} min={"1990-01-01"} max={now.to_string()}/>
        </div>
    }
}