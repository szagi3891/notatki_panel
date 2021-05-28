use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html, css};
use crate::state::{StateViewIndex};

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        padding: 0 5px;
        border-bottom: 1px solid black;
    ")
}

fn css_item() -> Css {
    css!("
        display: block;
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

pub fn render_menu(state: &Computed<StateViewIndex>) -> VDomElement {
    let state = state.get_value();

    let on_click = move || {
        state.current_edit();
    };

    html! {
        <div css={css_footer()}>
            <span css={css_item()}>"Utwórz plik"</span>
            <span css={css_item()}>"Utwórz katalog"</span>
            <span css={css_item()}>"Zmień nazwę"</span>
            <span css={css_item()} onClick={on_click}>"Edycja pliku"</span>
        </div>
    }
}
