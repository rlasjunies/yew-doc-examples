use yew::prelude::*;

pub struct InternalStateComponent {
    name:String,
}

impl Component for InternalStateComponent {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            name: "Clark".into(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{format!("Hello {}",self.name)}</h1>
            </>
        }
    }
}
