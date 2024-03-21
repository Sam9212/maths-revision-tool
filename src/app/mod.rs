mod browser;
mod creator;
mod dash;
mod login;
mod quiz;
mod register;
mod review;

use super::{
    app::{
        browser::Browser, creator::Creator, dash::Dashboard, login::Login, quiz::Quiz,
        register::Register, review::Review,
    },
    components::{
        theme_ctx::{Theme, ThemeProvider},
        user_ctx::UserContextProvider,
    },
};
use stylist::yew::{styled_component, Global};
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
    #[at("/browser")]
    Browser,
    #[at("/creator")]
    Creator,
    #[at("/quiz/:set_name")]
    Quiz { set_name: AttrValue },
    #[at("/review")]
    Review,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Browser => html! { <Browser /> },
        Route::Creator => html! { <Creator /> },
        Route::Review => html! { <Review /> },
        Route::Quiz { set_name } => html! { <Quiz {set_name} /> },
        Route::NotFound => html! { "Page Not Found" },
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let context = Theme {
        font_size: "16px".to_string(),
        font_family: "Inter".to_string(),
        input_color: "#BBBBBB".to_string(),
        primary_color: "#1F6AFB".to_string(),
        primary_shade: "#3783FF".to_string(),
        fg_color: "#FFFFFF".to_string(),
        bg_color: "#303030".to_string(),
        bg_shade: "#404040".to_string(),
        success_color: "#39C13F".to_string(),
        fail_color: "#C13939".to_string(),
    };

    let css = css!(
        r#"
            font-family: Arial, Helvetica, sans-serif;
            font-size: ${fs}px;

            color: ${fg};

            font-synthesis: none;
            text-rendering: optimizeLegibility;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
            -webkit-text-size-adjust: 100%;

            * {
                margin: 0;
                padding: 0;
            }

            a {
                color: ${fg};
                text-decoration: none;

                background: linear-gradient(${pc}, ${pc}),
                    linear-gradient(
                        to right,
                        ${pc},
                        ${pcs},
                        ${pc}
                    );
                background-size:
                    100% 3px,
                    0 3px;
                background-position:
                    100% 100%,
                    0 100%;
                background-repeat: no-repeat;
                transition: all 0.3s ease-in;
            }

            a:hover {
                background-size:
                    0 3px,
                    100% 3px;
                color: ${pcs};
            }
        "#,
        fs = context.font_size,
        fg = context.fg_color,
        pc = context.primary_color,
        pcs = context.primary_shade,
    );

    html! {
        <>
            <Global {css}/>
            <BrowserRouter>
                <ThemeProvider {context}>
                    <UserContextProvider>
                        <Switch<Route> render={switch} />
                    </UserContextProvider>
                </ThemeProvider>
            </BrowserRouter>
        </>
    }
}
