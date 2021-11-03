pub mod components;
pub mod pages;
pub mod switch;

use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

use crate::pages::{HomePage, PageNotFound};
use crate::switch::PublicUrlSwitch;
use crate::switch::{AppRoute, AppRouter};

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| AppRoute::PageNotFound(Permissive(Some(route.route))).into_public())
                    />
                </main>
            </>
        }
    }
}

impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Home => html! { <HomePage /> },
            AppRoute::PageNotFound(Permissive(route)) => html! { <PageNotFound route=route /> },
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
