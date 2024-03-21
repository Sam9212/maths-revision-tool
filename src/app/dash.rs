use crate::{
    app::Route,
    components::{
        administration::{AddAccountForm, DeleteAccountForm},
        inputs::Button,
        layout::Column,
        tabs::{Tab, TabController},
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};
use shared::AccessLevel;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::{components::Redirect, hooks::use_navigator};

#[styled_component(Dashboard)]
pub fn dashboard() -> Html {
    let nav = use_navigator().expect("Could not get hook to navigator");
    let theme = use_theme();
    let user = use_user();
    let user = match (*user).clone().inner {
        Some(user) => user,
        None => {
            return html! { <Redirect<Route> to={Route::Login}/> };
        }
    };

    let class = css!(
        r#"
            padding: ${fs};
            background-color: ${bg};
        "#,
        bg = theme.bg_color,
        fs = theme.font_size
    );

    html! {
        <Column wfill={true} hfill={true} justify_content={"center"} align_items={"center"}>
            <Column {class}>
                <h1>{ "Dashboard" }</h1>
                <br />
                <p>{ "Welcome back, " }{ user.username() }{ "." }</p>
                <br />
                <p>{ "If you would like to take a test" }</p>
                <p>{ "you can go to the browser." }</p>
                <Button onclick={move |_| nav.push(&Route::Browser)}>{ "Go To Browser" }</Button>

                if user.access_level() == &AccessLevel::ADMIN {
                    <br />
                    <TabController name={"add-edit-delete"}>
                        <Tab _id={"Add"}>
                            <AddAccountForm />
                        </Tab>

                        <Tab _id={"Delete"}>
                            <DeleteAccountForm />
                        </Tab>
                    </TabController>
                }
            </Column>
        </Column>
    }
}
