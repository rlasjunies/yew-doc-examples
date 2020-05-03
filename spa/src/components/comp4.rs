use yew::prelude::*;

pub struct UseOfPropertyComponent {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
    show_message: bool,
}

pub enum Msg {
    Click(),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Clark by default".to_string())]
    pub name: String,
}

impl Component for UseOfPropertyComponent {
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
                    <h1>{format!("Hello {}",self.name)}</h1>
                </>
            }
        }
    }
}
