#![allow(non_camel_case_types)]

use super::theme_ctx::use_theme;
use chrono::Utc;
use shared::{requests::UserReqError, AccessLevel, User};
use stylist::yew::styled_component;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlSelectElement};
use yew::{
    prelude::*,
    suspense::{use_future_with, Suspension, UseFutureHandle},
};
use yew_autoprops::autoprops;

#[autoprops]
#[styled_component(ValidatedInput)]
pub fn validated_input(
    children: &Children,
    #[prop_or(false)] shaded: bool,
    #[prop_or(3)] minl: usize,
    #[prop_or(20)] maxl: usize,
    id: AttrValue,
    #[prop_or(false)] hidden: bool,
    text_handle: &UseStateHandle<AttrValue>,
    #[prop_or(None)] secondary_handle: &Option<UseStateHandle<AttrValue>>,
    #[prop_or(None)] verif_handle: &Option<UseStateHandle<bool>>,
    validity_handle: &UseStateHandle<bool>,
) -> Html {
    let text_handle = text_handle.clone();
    let secondary_handle = secondary_handle.clone();
    let itype = if hidden { "password" } else { "text" };

    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(&id) {
                if let Ok(element) = element.dyn_into::<HtmlInputElement>() {
                    element.set_value(&*text_handle);
                }
            }
        }
    }

    // whenever the user is typing into the field
    // this runs so that we can store any relevant changes
    // made to the input into state vars
    let onkeyup = {
        // Cloning is necessary, but it's shallow & cheap,
        // so it's ok!
        let text_handle = text_handle.clone();
        let validity_handle = validity_handle.clone();

        // Actual callback function
        move |e: KeyboardEvent| {
            let value: AttrValue = e
                .target_dyn_into::<HtmlInputElement>() // Cast event target into specific type
                .unwrap() // Crash on error
                .value() // Get text from inside
                .into(); // Convert into AttrValue

            text_handle.set(value.clone());

            if value.len() < minl {
                // Case where there is too little text
                validity_handle.set(false);
            } else {
                // Case where the input is valid and verified
                validity_handle.set(true);
            }
        }
    };

    if let Some(secondary_handle) = secondary_handle.clone() {
        if let Some(verif_handle) = verif_handle.clone() {
            if *secondary_handle != *text_handle {
                // Case where double entry verification
                // is enabled and was text was not equal
                // to second input
                verif_handle.set(false);
            } else {
                verif_handle.set(true);
            }
        }
    }

    // Get theme from the context manager to place into
    // css classes for input
    let theme = use_theme();
    // if there is text we need to keep the label up
    // all the time instead of just while its focused
    // so the text is actually readable
    let class_non_empty = if (*text_handle).len() > 0 {
        css!("transform: translateY(-18px);")
    } else {
        css!()
    };

    let verif_bool = if let Some(verif_handle) = verif_handle.clone() {
        *verif_handle
    } else {
        true
    };

    let class = css!(
        r#"
            label {
                margin: calc( 0.5 * ${fs} + 2px );
                position: absolute;
                color: ${ic};
                transition: 0.2s transform cubic-bezier(0.075, 0.82, 0.165, 1);
                background-color: ${bg};
                cursor: text;
            }

            label:has(+ input:focus) {
                transform: translateY(-18px);
            }

            input {
                display: block;
                color: ${fg};
                padding: calc( 0.5 * ${fs} );
                background-color: ${bg};
                border: solid 4px ${pc};
            }

            p, br {
                font-size: calc( 0.75 * ${fs} );
            }
        "#,
        fs = theme.font_size,
        ic = theme.input_color,
        fg = theme.fg_color,
        bg = if !shaded {
            &theme.bg_color
        } else {
            &theme.bg_shade
        },
        pc = if verif_bool && **validity_handle {
            theme.primary_color
        } else {
            theme.fail_color
        },
    );

    html! {
        <div {class}>
            <label class={class_non_empty} for={&id}>{ children }</label>
            <input type={itype} {onkeyup} name={&id} {id} maxlength={maxl.to_string()} />
            if verif_bool && **validity_handle {
                // This only renders when input is valid
                <p><br /></p>
            } else {
                // This only renders when input is invalid
                <p>{ "Input is not valid!" }</p>
            }
        </div>
    }
}

#[autoprops]
#[styled_component(Button)]
pub fn button(
    children: &Children,
    #[prop_or(true)] clickable: bool,
    onclick: Callback<MouseEvent>,
) -> Html {
    let theme = use_theme();
    let class = css!(
        r#"
            color: ${fg};
            background-color: ${pc};
            border: 4px solid ${pc};
            font-size: calc( 1.5 * ${fs} );
            padding: calc( 0.125 * ${fs} ) calc( 1.5 * ${fs} ) calc( 0.125 * ${fs} ) calc( 1.5 * ${fs} );
            cursor: pointer;
            transition: 0.2s all cubic-bezier(0.075, 0.82, 0.165, 1);

            :hover {
                background-color: ${bg};
            }

            :disabled {
                background-color: ${fc};
                border-color: ${fc};
                cursor: not-allowed;
            }
        "#,
        fs = theme.font_size,
        fg = theme.fg_color,
        fc = theme.fail_color,
        bg = theme.bg_color,
        pc = theme.primary_color,
    );

    html! {
        <button disabled={!clickable} {class} onclick={if clickable {onclick} else {Callback::from(|_| {})}}><b>{ children }</b></button>
    }
}

#[autoprops]
#[styled_component(DateInput)]
pub fn date_input(
    #[prop_or(false)] shaded: bool,
    children: &Children,
    id: AttrValue,
    handle: &UseStateHandle<AttrValue>,
) -> Html {
    let theme = use_theme();
    let class = css!(
        r#"
            padding: calc( 0.5 * ${fs} );
            border: 4px solid ${pc};
            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: center;

            label {
                margin-right: calc( 0.25 * ${fs} );
                color: ${ip};
            }

            input {
                background-color: ${bg};
                color: ${fg};
                border: none;
                margin-bottom: 1px;
            }
        "#,
        ip = theme.input_color,
        bg = if !shaded {
            theme.bg_color
        } else {
            theme.bg_shade
        },
        fg = theme.fg_color,
        pc = theme.primary_color,
        fs = theme.font_size,
    );
    let now = Utc::now().format("%Y-%m-%d").to_string();
    let handle = handle.clone();
    let onchange = move |e: Event| {
        handle.set(
            e.target_dyn_into::<HtmlInputElement>()
                .expect("Input Element Failed To Cast")
                .value()
                .into(),
        );
    };

    html! {
        <div {class}>
            <label for={id.clone()}>{ children }</label>
            <input type={"date"} {onchange} name={id.clone()} {id} min={"1990-01-01"} max={now}/>
        </div>
    }
}

#[autoprops]
#[styled_component(AccessPicker)]
pub fn access_picker(
    children: &Children,
    id: AttrValue,
    handle: &UseStateHandle<AccessLevel>,
) -> Html {
    let onchange = {
        let handle = handle.clone();
        move |e: Event| {
            let level_str = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
            handle.set(match &level_str[..] {
                "User" => AccessLevel::USER,
                "Teacher" => AccessLevel::TEACHER,
                "Admin" => AccessLevel::ADMIN,
                _ => unreachable!("string can only be these 3 values"),
            })
        }
    };

    let theme = use_theme();
    let class = css!(
        r#"
            padding: calc( 0.5 * ${fs} );
            border: 4px solid ${pc};

            label {
                color: ${fc};
            }

            select {
                margin-left: calc( 0.25 * ${fs} );
            }
        "#,
        fc = theme.input_color,
        fs = theme.font_size,
        pc = theme.primary_color,
    );

    html! {
        <div {class}>
            <label for={id.clone()}>{ children }</label>
            <select name={id.clone()} {id} {onchange}>
                <option value="User">{ "User" }</option>
                <option value="Teacher">{ "Teacher" }</option>
                <option value="Admin">{ "Administrator" }</option>
            </select>
        </div>
    }
}

#[hook]
pub fn use_users(
    update: UseStateHandle<bool>,
) -> Result<UseFutureHandle<Result<Vec<User>, UserReqError>>, Suspension> {
    use_future_with(update, |_| async {
        crate::commands::invoke_get_usernames().await
    })
}

#[autoprops]
#[styled_component(UserPicker)]
pub fn user_picker(
    children: &Children,
    id: AttrValue,
    user_handle: &UseStateHandle<AttrValue>,
    dependency: &UseStateHandle<bool>,
) -> Html {
    let theme = use_theme();
    let unames = match use_users(dependency.clone()) {
        Ok(unames) => unames.clone().unwrap(),
        Err(e) => return html! { e },
    };

    let options = unames
        .into_iter()
        .map(|v| {
            let un = v.username();
            html! { <option value={un.clone()}>{un}</option>}
        })
        .collect::<Html>();

    let user_handle = user_handle.clone();
    let onchange = move |e: Event| {
        user_handle.set(
            e.target_dyn_into::<HtmlSelectElement>()
                .expect("failed to cast")
                .value()
                .into(),
        );
    };

    let class = css!(
        r#"
            padding: calc( 0.5 * ${fs} );
            border: 4px solid ${pc};

            label {
                color: ${fc};
            }

            select {
                margin-left: calc( 0.25 * ${fs} );
            }
        "#,
        fc = theme.input_color,
        fs = theme.font_size,
        pc = theme.primary_color,
    );

    html! {
        <div {class}>
            <label for={id.clone()}>{ children }</label>
            <select name={id.clone()} {id} {onchange}>{ options }</select>
        </div>
    }
}

#[autoprops]
#[styled_component(RadioToggle)]
pub fn radio_toggle(name: AttrValue, children: &Children, handle: &UseStateHandle<bool>) -> Html {
    let initial = **handle;

    let onclick_t = {
        let handle = handle.clone();
        move |_| {
            handle.set(true);
        }
    };

    let onclick_f = {
        let handle = handle.clone();
        move |_| {
            handle.set(false);
        }
    };

    let theme = use_theme();

    let class = css!(
        r#"
            input {
                display: none;
            }

            label {
                display: inline;
                padding: 4px;
                background-color: ${p};
            }

            input:checked + label {
                background-color: ${ps};
            }

            * {
                text-align: center;
                font-size: calc(1.5 * ${fs});
                flex-grow: 1;
            }
        "#,
        p = theme.primary_color,
        ps = theme.primary_shade,
        fs = theme.font_size,
    );

    html! {
        <div {class}>
            { children }
            <input type={"radio"} name={&name} id={"t"} onclick={onclick_t} checked={initial}/>
            <label for={"t"}>{ "Yes" }</label>
            <input type={"radio"} name={&name} id={"f"} onclick={onclick_f} checked={!initial}/>
            <label for={"f"}>{ "No" }</label>
        </div>
    }
}
