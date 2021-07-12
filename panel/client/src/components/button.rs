use vertigo::{
    VDomElement,
    Css,
};

use vertigo_html::{html, css};

//display: block;
fn css_item() -> Css {
    css!("
        display: inline-block;
        border: 1px solid #a0a0a0;
        margin: 5px 10px 5px 0;
        padding: 0 5px;
        border-radius: 3px;
        height: 25px;
        line-height: 25px;
        font-size: 14px;

        :hover {
            cursor: pointer;
            background-color: #00ff0060;
        }
    ")
}

pub fn button<F: Fn() + 'static>(label: &'static str, on_click: F) -> VDomElement {
    html! {
        <span css={css_item()} on_click={on_click}>{label}</span>
    }
}
