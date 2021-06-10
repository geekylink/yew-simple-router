use yew::prelude::*;
use yew_router::prelude::*;

use crate::app_router::{AppRoute, Link};

pub struct About {
}

impl Component for About {
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
        html! {
        <>
            <div>{ "About Yo!" }</div>
            <div>
                { "Powered by " }
                <a href="https://en.wikipedia.org/wiki/Beer">{ "Beer" }</a>
            </div>
            <Link route=AppRoute::Index>{"Go home"}</Link>
        </>
        }
    }

}

