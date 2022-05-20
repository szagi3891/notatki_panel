use vertigo::{VDomElement, html, css, Css};

fn css_wrapper() -> Css {
    css!{"
        position: fixed;
        left: 0;
        top: 0;
        right: 0;
    "}
}

pub fn stict_to_top(content: VDomElement) -> VDomElement{

    html! {
        <div css={css_wrapper()}>
            { content }
        </div>
    }
}

