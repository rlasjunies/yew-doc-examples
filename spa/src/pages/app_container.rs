use super::AppRoute;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{service::RouteService, Switch};

use super::*;

pub struct AppContainer {
    route_service: RouteService<()>,
}

impl Component for AppContainer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            route_service: RouteService::new(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
    
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        // true // This will always re-render when new props are provided.
        false
     }

    fn view(&self) -> Html {
        html! {
            <div class="full-height">
                <menu>
                    <a href="/app/main">{ "Home" }</a>
                    <a href="/app/page1">{ "1- Simple Component" }</a>
                    <a href="/app/page2">{ "2- Internal state" }</a>
                    <a href="/app/page3">{ "3- Trap events" }</a>
                    <a href="/app/page4">{ "4- Use properties" }</a>
                    <a href="/app/page5">{ "5- Emit events" }</a>
                    <a href="/app/page6">{ "6- Class component" }</a>
                </menu>
                <div class="main-pane full-height">
                    { self.view_app_content() }
                </div>
            </div>
        }
    }
}

impl AppContainer {
    fn view_app_content(&self) -> Html {
        let route = self.route_service.get_route();
        match AppRoute::switch(route) {
            Some(AppRoute::Main) => html! {
                <>
                    <h1>{"This project is made to test some examples used in the documentation"}</h1>
                </>
            },
            Some(AppRoute::Page1) => html!{
                <>
                    <Page1/>
                </>
            },
            Some(AppRoute::Page2) => html!{
                <>
                    <Page2/>
                </>
            },
            Some(AppRoute::Page3) => html!{
                <>
                    <Page3/>
                </>
            },
            Some(AppRoute::Page4) => html!{
                <>
                    <Page4/>
                </>
            },
            Some(AppRoute::Page5) => html!{
                <>
                    <Page5/>
                </>
            },
            Some(AppRoute::Page6) => html!{
                <>
                    <Page6/>
                </>
            },
            // Some(AppRoute::Page(page)) => html! {
            //     <>
            //     // <Page1 />
            //     </>
            // },
            _ => html! {
                <div>{ "Route not found. You may mistyped the URL. Use the links in the top of the page" }</div>
            },
        }
    }
}
