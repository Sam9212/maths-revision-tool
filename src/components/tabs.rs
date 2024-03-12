#![allow(non_camel_case_types)]

use super::theme_ctx::use_theme;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[styled_component(TabController)]
pub fn tab_controller(name: AttrValue, children: &ChildrenWithProps<Tab>) -> Html {
    // use_state generates a handle so that pieces of code only run once and then cache
    // their values after the #[function_component] macro converts functions into structs.

    // map() is an iterator transformation function. I'm using it to turn the collection
    // of children into a Vec (also known as a list/growable array in other langs) that
    // contains the name of the tab and its contents so that I can search for the names
    // later
    // Cloning is just to satisfy borrow checker - It's a cheap clone though, most clones
    // used in Yew code are clones of Reference Counted smart pointers which means they
    // don't have much performance impact - a Rust deep clone is the equivalent of any cloning/copying
    // that would happen your average langugae, but Rust has much faster shallow copies/clones that can
    // be done on certain types, like RC pointers and Copy'able types.
    let tabs = children
        .clone()
        .into_iter()
        .map(|c| (c.props._id.clone(), c))
        .collect::<Vec<_>>();

    // Useful thing about these state handles is that you can change the value inside later
    // to dynamically change the UI. We use that feature here with the selected variable to
    // assign each tab to a button so that we can move between them easily.
    let selected = use_state_eq(|| (*tabs)[0].clone().0);

    // Each re-render that happens will then call this code again, retrieving the tab that has
    // the same name as the selected variable. I am using the filter iterator transform to
    // fetch the tab because they are so handy to use in a non-mutable context like this.
    let selected_tab = tabs.iter().filter(|p| p.0 == *selected).collect::<Vec<_>>()[0]
        .clone()
        .1;

    // This is the construction of the radio buttons that let me navigate between tabs. It's in
    // a state block because I don't want to run this long code more than once - it doesn't need
    // to be triggered every re-render.
    let radios = use_state(|| {
        // This is to get a list of the keys without the actual contents so that I can put the name
        // of each tab onto the buttons, alongside the key name of the correct tab into the onclick
        // event that i construct for each one.
        let keys = tabs.iter().map(|p| p.0.clone());
        keys.map(|id| {
            let onclick = {
                // We need a new clone of these values inside of this inner context so that we can
                // add an onclick callback that sets the selected tab.
                let selected = selected.clone();
                let id = id.clone();
                move |_| {
                    selected.set(id.clone());
                }
            };

            html!{
                <div>
                    <input type={"radio"} key={&*id.clone()} id={id.clone()} name={name.clone()} {onclick}/>
                    <label for={id.clone()}>{ id }</label>
                </div>
            }
        }).collect::<Html>()
        // Collecting as an Html object is a very interesting feature of the Yew library.
        // This allows me to iterate over an object and turn it into a list of HTML elements,
        // Which results in this nice and clean pattern as seen above, for the button construction.
    });

    // I can grab the distributed theme struct using the custom 'use_theme()' hook that I wrote to
    // get the color scheme and put it onto each of the elements. This is also used for something
    // much more vital, that being the layout of the elements.
    let theme = use_theme();

    let class = css!(
        r#"
            background-color: ${bg};

            .panel input {
                display: none;
            }

            .panel label {
                display: block;
                padding: 4px;
                background-color: ${p}
            }

            .panel input:checked + label {
                background-color: ${ps}
            }

            .panel {
                display: flex;
                flex-direction: row;
                width: 100%;
            }

            .panel > * {
                text-align: center;
                font-size: calc(1.5 * ${fs});
                flex-grow: 1;
            }
        "#,
        bg = theme.bg_color,
        p = theme.primary_color,
        ps = theme.primary_shade,
        fs = theme.font_size,
    );

    // The HTML for the tab controller is actually very simple, with the selected tab being
    // displayed above, and the row of buttons for selection being underneath.
    html! {
        <div {class}>
            <div>
            { selected_tab }
            </div>
            <div class={classes!("panel")}>
                { (*radios).clone() }
            </div>
        </div>
    }
}

#[autoprops]
#[function_component(Tab)]
pub fn tab(_id: AttrValue, children: &Children) -> Html {
    html! { children.clone() }
}
