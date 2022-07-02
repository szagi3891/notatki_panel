use vertigo::{Css, VDomComponent, css, html, bind};

use super::AppNewdir;
use crate::components::button;

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

pub fn app_newdir_render(state: AppNewdir) -> VDomComponent {

    let view_new_name = VDomComponent::dom(state.new_name.render(true));

    VDomComponent::from_ref(&state, move |context, state| {
        let parent_path = state.parent.as_slice().join("/");

        let mut buttons = vec!(VDomComponent::dom(button("Wróć", bind(&state.app).call(|_, app| {
            app.redirect_to_index();
        }))));
    
        let save_enable = state.save_enable.get(context);

        if save_enable {
            buttons.push(VDomComponent::dom(button("Zapisz", state.bind_on_save(&state.app))));
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
