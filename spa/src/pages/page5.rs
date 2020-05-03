use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::*;
pub struct Page5 {
    link: ComponentLink<Self>,
    event_received:bool,
    answer:String,
}

pub enum Msg{
    EventReceived(String),
}

impl Component for Page5 {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            event_received:false,
            answer:"".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EventReceived(answer) => {
                self.event_received = true;
                self.answer = answer;
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        // true // This will always re-render when new props are provided.
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="full-height">
                <EmitEventComponent 
                    name="Clark"
                    onmyclickevent = self.link.callback (|answer| Msg::EventReceived(answer)) 
                    />
                { if self.event_received {
                    html!{
                        <>
                            {format!("Answer received by event from the component:[{}]", self.answer)}
                        </>
                    }
                    } else {
                        html!{}
                    }
                }
            </div>
        }
    }
}
