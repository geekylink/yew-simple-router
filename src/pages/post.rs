use yew::prelude::*;
use yew_router::prelude::*;

use crate::app_router::{AppRoute, Link};

pub struct Post {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub id: i64,
}

impl Component for Post {
    type Message = (); 
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
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
            <div>{ "You are on post: "} { self.props.id }</div>
            <Link route=AppRoute::Index>{"Go home"}</Link>
        </>
        }
    }

}

