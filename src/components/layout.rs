#![allow(non_camel_case_types)]

use stylist::yew::styled_component;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[styled_component(Row)]
pub fn row(
    children: &Children,
    #[prop_or(false)] hfill: bool,
    #[prop_or(false)] wfill: bool,
    #[prop_or(AttrValue::from("start"))] justify_content: AttrValue,
    #[prop_or(AttrValue::from("start"))] align_items: AttrValue,
    #[prop_or(Classes::new())] class: Classes,
) -> Html {
    let hfill = if hfill { css!("height: 100%;") } else { css!() };
    let wfill = if wfill { css!("width: 100%;") } else { css!() };

    let class_inner = css!(
        r#"
            display: flex;
            flex-flow: row nowrap;
            justify-content: ${jc};
            align-items: ${ai};
        "#,
        jc = justify_content,
        ai = align_items,
    );

    html! {
        <div class={classes!(class, class_inner, hfill, wfill)}>
            { children }
        </div>
    }
}

#[autoprops]
#[styled_component(Column)]
pub fn column(
    children: &Children,
    #[prop_or(false)] hfill: bool,
    #[prop_or(false)] wfill: bool,
    #[prop_or(AttrValue::from("start"))] justify_content: AttrValue,
    #[prop_or(AttrValue::from("start"))] align_items: AttrValue,
    #[prop_or(Classes::new())] class: Classes,
) -> Html {
    let hfill = if hfill { css!("height: 100%;") } else { css!() };
    let wfill = if wfill { css!("width: 100%;") } else { css!() };

    let class_inner = css!(
        r#"
            display: flex;
            flex-flow: column nowrap;
            justify-content: ${jc};
            align-items: ${ai};
        "#,
        jc = justify_content,
        ai = align_items,
    );

    html! {
        <div class={classes!(class, class_inner, hfill, wfill)}>
            { children }
        </div>
    }
}
