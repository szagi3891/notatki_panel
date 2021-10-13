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
        height: 300px;
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

    pub fn delete_message(&self) {
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

            let on_click = {
                let alert_state= alert_state.clone();
                move || {
                    alert_state.delete_message();
                }
            };

            html! {
                <div css={css_bg()}>
                    <div css={css_center()}>
                        <div on_click={on_click}>
                            { message }
                        </div>
                        <component {render_progress} data={alert_state.progress_computed} />
                    </div>
                </div>
            }
        }
    }
}