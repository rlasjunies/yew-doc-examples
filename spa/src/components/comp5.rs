use yew::prelude::*;

pub struct EmitEventComponent {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
    show_message: bool,
}

pub enum Msg {
    Click(),
    Click4Event(),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Clark by default".to_string())]
    pub name: String,

    #[prop_or_default]
    pub onmyclickevent:Callback<String>,
}

impl Component for EmitEventComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props.clone(),
            name: props.name.into(),
            show_message: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click() => self.show_message = true,
            Msg::Click4Event() => self.props.onmyclickevent.emit("Hello Loise".into()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
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
                    {"Click on clark to raised an event for the parent container ;-)"}
                    <h1 onclick=self.link.callback( |_| Msg::Click4Event()) >
                    {format!("Hello {}",self.name)}</h1>
                </>
            }
        }
    }
}
