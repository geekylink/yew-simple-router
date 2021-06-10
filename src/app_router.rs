use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

pub struct AppRouter {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/post/{id}"]
    Post(i64),
    #[to = "/about"]
    About,
    #[to = "/"]
    Index,
}

pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
    type Message =(); 
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|route: AppRoute| match route {
            AppRoute::Index => html! { <pages::Index/> },
            AppRoute::About => html! { <pages::About/> },
            AppRoute::Post(id) => html! { <pages::Post id={id}/> },
            _ => html! { "Not found!" },
            // AppRoute::Order(id) => html! { <pages::Order id={id}/> },
        });

        html! {
            <Router<AppRoute, ()> render=render_func/>
        }
    }

}

/*fn switch(routes: &AppRoute) -> Html {
        html! {
            <h1>
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::About => html!{"About"},
                    AppRoute::Index => html!{"Home"},
                }
            })
            />
            </h1>
        }
}*/

fn main() {
    yew::start_app::<AppRouter>();
}
