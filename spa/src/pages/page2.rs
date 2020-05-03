use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::InternalStateComponent;
pub struct Page2 {}

impl Component for Page2 {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
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
                <InternalStateComponent/>
            </div>
        }
    }
}
