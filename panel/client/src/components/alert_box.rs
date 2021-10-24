use vertigo::VDomElement;

use vertigo::{
    Css,
    computed::{
        Computed,
    }
};
use vertigo_html::{html, css};


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
        position: relative;
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
        padding: 0 20px;
    ")
}

fn css_progress() -> Css {
    css!("
        display: flex;
        position: absolute;
        left: 0;
        right: 0;
        top: 0;
        bottom: 0;
        background-color: white;
        justify-content: center;
        align-items: center;
    ")
}


fn render_progress(progress_computed: &Computed<bool>) -> VDomElement {
    let progress = progress_computed.get_value();

    if *progress {
        return html! {
            <div css={css_progress()}>
                "Przetwarzanie ..."
            </div>
        }
    }

    html! {
        <div/>
    }
}

pub struct AlertBox {
    message: String,
    progress: Computed<bool>,
    buttons: Vec<VDomElement>,
}

impl AlertBox {
    pub fn new(message: String, progress: Computed<bool>) -> AlertBox {
        AlertBox {
            message,
            progress,
            buttons: Vec::new(),
        }
    }

    pub fn button<F: Fn() + 'static>(&mut self, label: &'static str, on_click: F) {
        self.buttons.push(button(label, on_click))
    }

    pub fn render_popup(content: VDomElement) -> VDomElement {
        html! {
            <div css={css_bg()}>
                <div css={css_center()}>
                    {content}
                </div>
            </div>
        }
    }

    pub fn render(self) -> VDomElement {
        let AlertBox { message, progress, buttons } = self;

        let content = html! {
            <div>
                <div css={css_message()}>
                    { message }
                </div>

                <component {render_progress} data={progress} />

                <div css={css_buttons_wrapper()}>
                    { ..buttons }
                </div>
            </div>
        };

        Self::render_popup(content)
    }
}
