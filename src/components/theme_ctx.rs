use yew::prelude::*;
use yew_autoprops::autoprops;

#[derive(PartialEq, Clone)]
pub struct Theme {
    pub font_size: String,
    pub font_family: String,
    pub primary_color: String,
    pub primary_shade: String,
    pub fg_color: String,
    pub bg_color: String,
    pub bg_shade: String,
    pub success_color: String,
    pub fail_color: String,
}

#[autoprops]
#[function_component(ThemeProvider)]
pub fn theme_provider(children: &Children, context: &Theme) -> Html {
    html!{
        <ContextProvider<Theme> context={context.clone()}>
            { children.clone() }
        </ContextProvider<Theme>>
    }
}

#[hook]
pub fn use_theme() -> Theme {
    use_context().unwrap()
}