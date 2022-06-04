use vertigo::VDomComponent;

use vertigo::{
    Css,
    Computed,
};
use vertigo::{html, css};

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

fn render_progress(progress: Computed<bool>) -> VDomComponent {
    VDomComponent::from(progress, |progress| {
        let progress = progress.get();

        if progress {
            return html! {
                <div css={css_progress()}>
                    "Przetwarzanie ..."
                </div>
            }
        }

        html! {
            <div/>
        }
    })
}

pub struct AlertBox {
    message: VDomComponent,
    progress: Computed<bool>,
    buttons: Vec<VDomComponent>,
    content: Option<VDomComponent>,
}

impl AlertBox {
    pub fn new(message: VDomComponent, progress: Computed<bool>) -> AlertBox {
        AlertBox {
            message,
            progress,
            buttons: Vec::new(),
            content: None,
        }
    }

    pub fn button(mut self, component: VDomComponent) -> Self {
        self.buttons.push(component);
        self
    }

    pub fn set_content(mut self, content: VDomComponent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn render_popup(content: VDomComponent) -> VDomComponent {
        VDomComponent::from_html(
            html! {
                <div css={css_bg()}>
                    <div css={css_center()}>
                        {content}
                    </div>
                </div>
            }
        )
    }

    pub fn render(self) -> VDomComponent {
        let AlertBox { message, progress, buttons, content } = self;
        let progress = render_progress(progress);

        let content = match content {
            Some(content) => {
                html! {
                    <div>
                        { content }
                    </div>
                }
            },
            None => html! {
                <div />
            }
        };

        let content = VDomComponent::from_html(html! {
            <div>
                <div css={css_message()}>
                    { message }
                </div>

                { progress }

                <div css={css_buttons_wrapper()}>
                    { ..buttons }
                </div>

                { content }
            </div>
        });

        Self::render_popup(content)
    }
}
