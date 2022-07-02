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
        width: 600px;

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

fn render_progress(progress: Option<Computed<bool>>) -> VDomComponent {
    VDomComponent::from(progress, |context, progress| {
        if let Some(progress) = progress {
            let progress = progress.get(context);

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
        } else {
            html! {
                <div/>
            }
        }
    })
}

pub struct AlertBox {
    message: VDomComponent,
    progress: Option<Computed<bool>>,
    buttons: Vec<VDomComponent>,
    content: Option<VDomComponent>,
}

impl AlertBox {
    pub fn new(message: VDomComponent) -> AlertBox {
        AlertBox {
            message,
            progress: None,
            buttons: Vec::new(),
            content: None,
        }
    }

    pub fn progress(mut self, progress: Computed<bool>) -> Self {
        self.progress = Some(progress);
        self
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
        VDomComponent::from_fn(move |_| {
            html! {
                <div css={css_bg()}>
                    <div css={css_center()}>
                        {content.clone()}
                    </div>
                </div>
            }
        })
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

        let content = VDomComponent::from_fn(move |_| {
            html! {
                <div>
                    <div css={css_message()}>
                        { message.clone() }
                    </div>

                    { progress.clone() }

                    <div css={css_buttons_wrapper()}>
                        { ..buttons.clone() }
                    </div>

                    { content.clone() }
                </div>
            }
        });

        Self::render_popup(content)
    }
}
