use yew::prelude::*;

pub struct TrapEventComponent {
    link: ComponentLink<Self>,

    name: String,
    show_message: bool,
}

pub enum Msg {
    Click(),
}

impl Component for TrapEventComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: "Clark".into(),
            show_message: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click() => self.show_message = true,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if !self.show_message {
            html! {
                <>
                    <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"}</button>
                </>
            }
        } else {
            html! {
                <>
                    <h1>{format!("Hello {}",self.name)}</h1>
                </>
            }
        }
    }
}
