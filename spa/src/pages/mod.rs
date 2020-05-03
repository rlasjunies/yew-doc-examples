mod app_container;
mod page1;
mod page2;
mod page3;
mod page4;
mod page5;
mod page6;

use yew_router::Switch;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/app/main"]
    Main,
    #[to = "/app/page1"]
    Page1,
    #[to = "/app/page2"]
    Page2,
    #[to = "/app/page3"]
    Page3,
    #[to = "/app/page4"]
    Page4,
    #[to = "/app/page5"]
    Page5,
    #[to = "/app/page6"]
    Page6,

    // #[to = "/app/{pagename}"]
    // Page(String),
}

pub use app_container::AppContainer;
pub use page1::Page1;
pub use page2::Page2;
pub use page3::Page3;
pub use page4::Page4;
pub use page5::Page5;
pub use page6::Page6;
