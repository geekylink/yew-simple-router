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
        });

        html! {
            <Router<AppRoute, ()> render=render_func/>
        }
    }

}

fn main() {
    yew::start_app::<AppRouter>();
}
