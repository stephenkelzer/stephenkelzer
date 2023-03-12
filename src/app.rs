use yew::prelude::*;

use crate::body::body::Body;
use crate::footer::footer::Footer;
use crate::header::header::Header;

#[function_component(App)]
pub fn app() -> Html {
    let version = env!("CARGO_PKG_VERSION").to_string();

    html! {
        <div id={"app"} version={ version }>
            <Header />
            <Body />
            <Footer />
        </div>
    }
}
