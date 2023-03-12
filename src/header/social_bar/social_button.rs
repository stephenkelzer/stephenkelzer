use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub icon_class: String,
    pub title: String,
}

#[function_component(SocialButton)]
pub fn social_button(props: &Props) -> Html {
    let a_class = use_style!(
        r#"
        color: #e0e0e0;display: inline-block;
        width: 75px;
        height: 75px;
        line-height: 75px;
        vertical-align: middle;
        border-right: 1px solid #696969;

        &:first-of-type {
            border-left: 1px solid #696969;
        }

        &:hover {
            background-color: #696969;
        }
        "#
    );
    html! {
        <a class={a_class} href={props.url.clone()} title={props.title.clone()} target="_blank" rel="noreferrer">
            <i class={format!("bi bi-{}", props.icon_class.clone())}></i>
        </a>
    }
}
