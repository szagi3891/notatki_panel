use vertigo::{Css, VDomComponent, css, html, bind};

use super::AppNewdir;
use crate::{components::button, app::App};

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        width: 100vw;
        height: 100vh;
    ")
}

fn css_header() -> Css {
    css!("
        border-bottom: 1px solid black;
        padding: 5px;
    ")
}

pub fn app_newdir_render(state: AppNewdir, app: App) -> VDomComponent {

    let view_new_name = state.new_name.render(true);

    VDomComponent::new(&state, move |state| {
        let parent_path = state.parent.as_slice().join("/");

        let mut buttons = vec!(button("Wróć", bind(&app).call(|app| {
            app.redirect_to_index();
        })));

        let save_enable = state.save_enable.get_value();

        if *save_enable {
            buttons.push(button("Zapisz", state.bind_on_save(&app)));
        }

        html! {
            <div css={css_wrapper()}>
                <div css={css_header()}>
                    "tworzenie katalogu => "
                    {parent_path}
                </div>
                <div css={css_header()}>
                    { ..buttons }
                </div>
                { view_new_name.clone() }

                <div data-run-module="funkcjaJs">
                </div>
            </div>
        }
    })
}
