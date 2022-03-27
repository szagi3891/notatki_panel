use vertigo::{
    VDomElement,
    Css,
};

use vertigo::{html, css};

fn css_item() -> Css {
    css!("
        display: inline-block;
        border: 1px solid #a0a0a0;
        margin: 5px;
        padding: 0 5px;
        border-radius: 3px;
        height: 25px;
        line-height: 23px;
        font-size: 14px;
        overflow: hidden;

        :hover {
            cursor: pointer;
            background-color: #00ff0060;
        }
    ")
}

pub fn button(label: &'static str, on_click: impl Fn() + 'static) -> VDomElement {
    html! {
        <span css={css_item()} on_click={on_click}>{label}</span>
    }
}

