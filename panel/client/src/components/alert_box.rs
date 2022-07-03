use vertigo::{DomElement, dom, DomComment, DomNode};

use vertigo::{
    Css,
    Computed,
};
use vertigo::{css};

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

fn render_progress(progress: Computed<bool>) -> DomComment {
    progress.render_value_option(|progress| {
        if progress {
            Some(dom! {
                <div css={css_progress()}>
                    "Przetwarzanie ..."
                </div>
            })
        } else {
            None
        }
    })
}

pub struct AlertBox {
    message: DomElement,
    progress: Option<Computed<bool>>,
    buttons: Vec<DomElement>,
    content: Option<DomNode>,
}

impl AlertBox {
    pub fn new(message: DomElement) -> AlertBox {
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

    pub fn button(mut self, component: DomElement) -> Self {
        self.buttons.push(component);
        self
    }

    pub fn set_content(mut self, content: impl Into<DomNode>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn render_popup(content: DomElement) -> DomElement {
        dom! {
            <div css={css_bg()}>
                <div css={css_center()}>
                    {content}
                </div>
            </div>
        }
    }

    pub fn render(self) -> DomElement {
        let AlertBox { message, progress, buttons, content } = self;

        let result = dom! {
            <div>
                <div css={css_message()}>
                    { message.clone() }
                </div>
            </div>
        };
    
        if let Some(progress) = progress {
            result.add_child(render_progress(progress));
        }

        if buttons.len() > 0 {
            let buttons_wrapper = dom! { <div css={css_buttons_wrapper()}/> };

            for button in buttons.into_iter() {
                buttons_wrapper.add_child(button);
            }

            result.add_child(buttons_wrapper);
        }

        if let Some(content) = content {
            result.add_child(dom! {
                <div>
                    { content }
                </div>
            });
        }
    
        Self::render_popup(result)
    }
}
