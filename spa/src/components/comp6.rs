use yew::prelude::*;

pub struct ClassPropertyComponent {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or_default]
    pub class:String,
}

impl Component for ClassPropertyComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => (),
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
        html! {
            <div class={format!("{}",self.props.class)}>
                <h1>{"I am super Class"}</h1>
            </div>
        }
    }
}