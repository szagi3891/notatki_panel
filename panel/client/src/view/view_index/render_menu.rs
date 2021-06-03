use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html, css};
use crate::state::{StateViewIndex};
use crate::view::components::button;

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        padding: 0 5px;
        border-bottom: 1px solid black;
    ")
}

pub fn render_menu(state: &Computed<StateViewIndex>) -> VDomElement {
    let state = state.get_value();

    let on_click = move || {
        state.current_edit();
    };

    let mut out = Vec::new();

    out.push(button("Utwórz plik", || {}));
    out.push(button("Utwórz katalog", || {}));
    out.push(button("Zmień nazwę", || {}));
    out.push(button("Edycja pliku", on_click));

    // let out = [
    //     button("Utwórz plik", || {}),
    //     button("Utwórz katalog", || {}),
    //     button("Zmień nazwę", || {}),
    //     button("Edycja pliku", on_click)
    // ];

    // <span css={css_item()}>"Utwórz plik"</span>
    // <span css={css_item()}>"Utwórz katalog"</span>
    // <span css={css_item()}>"Zmień nazwę"</span>
    // <span css={css_item()} onClick={on_click}>"Edycja pliku"</span>

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
