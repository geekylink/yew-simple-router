use yew::prelude::*;
use yew_router::prelude::*;

use crate::app_router::{AppRoute, Link};

pub struct Index {
}

impl Component for Index {
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
            <h2>{ "Classy Posts:" }</h2>
            <ul>
                <li><Link route=AppRoute::Post(1)>{"Read it"}</Link></li>
                <li><Link route=AppRoute::Post(101)>{"Or this one"}</Link></li>
            </ul>
            <Link route=AppRoute::About>{"About dis"}</Link>
        </>
        }
    }

}

