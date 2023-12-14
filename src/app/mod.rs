mod login;
mod dash;
mod register;

use yew::prelude::*;
use yew_router::prelude::*;
use super::{
    components::user_ctx::UserContextProvider,
    app::{
        dash::Dashboard,
        login::Login,
        register::Register,
    }
};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Login,

    #[at("/register")]
    Register,

    #[at("/dash")]
    Dashboard,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::NotFound => html! { "Page Not Found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Switch<Route> render={switch} />
            </UserContextProvider>
        </BrowserRouter>
    }
}
