mod admin;
mod dash;
mod login;
mod register;

use super::{
    app::{admin::Admin, dash::Dashboard, login::Login, register::Register},
    components::{
        theme_ctx::{Theme, ThemeProvider},
        user_ctx::UserContextProvider,
    },
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dash")]
    Dashboard,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Admin => html! { <Admin /> },
        Route::NotFound => html! { "Page Not Found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let context = Theme {
        font_size: "16px".to_string(),
        font_family: "Inter".to_string(),
        primary_color: "#1F6AFB".to_string(),
        primary_shade: "#3783FF".to_string(),
        fg_color: "#FFFFFF".to_string(),
        bg_color: "#303030".to_string(),
        bg_shade: "#404040".to_string(),
        success_color: "#39C13F".to_string(),
        fail_color: "#C13939".to_string(),
    };

    html! {
        <BrowserRouter>
            <ThemeProvider {context}>
                <UserContextProvider>
                    <Switch<Route> render={switch} />
                </UserContextProvider>
            </ThemeProvider>
        </BrowserRouter>
    }
}
