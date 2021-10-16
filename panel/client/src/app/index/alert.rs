use std::rc::Rc;
use vertigo::VDomElement;
use vertigo::{
    Css,
    computed::{
        Computed,
        Value
    },
};
use vertigo_html::{html, css};
use crate::app::AppState;
use crate::components::button;

fn css_bg() -> Css {
    css!("
        position: fixed;
        left: 0;
        right: 0;
        top: 0;
        bottom: 0;
        background-color: #00000080;

        display: flex;
        align-items: center;
        justify-content: center;
    ")
}

fn css_center() -> Css {
    css!("
        background: white;
        width: 400px;

        justify-content: center;
        padding-top: 20px;
        padding-bottom: 20px;
        display: flex;
        flex-direction: column;
    ")
}

fn css_buttons_wrapper() -> Css {
    css!("
        display: flex;
        justify-content: center;
        margin-top: 20px;
    ")
}

fn css_message() -> Css {
    css!("
        display: flex;
        justify-content: center;
    ")
}

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile {
        message: String,
        // progress: Value<bool>,
    },
    //delete dir
}

#[derive(PartialEq)]
pub struct AlertState {
    app_state: Rc<AppState>,
    progress: Value<bool>,
    progress_computed: Computed<bool>,
    view: Value<AlertView>,
    // None,
    // DeleteFile {
    //     message: String,
    //     app_state: Rc<AppState>,
    //     progress: Value<bool>,
    // },
    //delete dir
}

impl AlertState {
    pub fn new(app_state: Rc<AppState>) -> Computed<AlertState> {
        let view = app_state.root.new_value(AlertView::None);
        let progress = app_state.root.new_value(false);
        let progress_computed = progress.to_computed();

        app_state.root.new_computed_from(AlertState {
            app_state: app_state.clone(),
            progress,
            progress_computed,
            view,
        })
    }

    pub fn delete(&self, message: String) {
        log::info!("delete akcja ...");

        self.view.set_value(AlertView::DeleteFile {
            message,
        });

        //trzeba przełączyć się na alert który pyta czy skasować
    }

    //delete_yes ... przetwarzanie ...
    fn delete_yes(&self) {
        log::info!("usuwamy ...");
    }

    fn delete_no(&self) {
        log::info!("anulujemy usuwanie ...");
        self.view.set_value(AlertView::None);
    }
}

fn render_progress(progress_computed: &Computed<bool>) -> VDomElement {
    let progress = progress_computed.get_value();

    if *progress {
        return html! {
            <div>
                "Przetwarzanie ..."
            </div>
        }
    }

    html! {
        <div/>
    }
}

pub fn render_alert(state: &Computed<AlertState>) -> VDomElement {
    let alert_state = state.get_value();
    let alert = alert_state.view.get_value();

    match alert.as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile {message} => {
            let message = format!("aler delete file ... {}", message);

            // let on_click = {
            //     let alert_state= alert_state.clone();
            //     move || {
            //         alert_state.delete_message();
            //     }
            // };

            let on_click_yes = {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_yes();
                }
            };

            let on_click_no = {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_no();
                }
            };

            let button_no = button("Nie", on_click_no);
            let button_yes = button("Tak", on_click_yes);
    
            html! {
                <div css={css_bg()}>
                    <div css={css_center()}>
                        <div css={css_message()}>
                            { message }
                        </div>

                        <component {render_progress} data={alert_state.progress_computed} />

                        <div css={css_buttons_wrapper()}>
                            { button_no }
                            { button_yes }
                        </div>
                    </div>
                </div>
            }
        }
    }
}