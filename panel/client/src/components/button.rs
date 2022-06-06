use std::rc::Rc;

use vertigo::{
    VDomElement,
    Css, Computed, VDomComponent,
};

use vertigo::{html, css};

enum ButtonType {
    Disabled,
    Active,
    Process,
}

fn css_item(button_type: ButtonType) -> Css {
    let style = css!("
        display: inline-block;
        border: 1px solid #a0a0a0;
        margin: 5px;
        padding: 0 5px;
        border-radius: 3px;
        height: 25px;
        line-height: 23px;
        font-size: 14px;
        overflow: hidden;

        :hover {            
            background-color: #00ff0060;
        }
    ");

    match button_type {
        ButtonType::Disabled => style.extend(css!("
            opacity: 0.3;
        ")),
        ButtonType::Active => style.extend(css!("
            cursor: pointer;
        ")),
        ButtonType::Process => style.extend(css!("
            opacity: 0.3;
            color: yellow;
        ")),
    }
}


pub fn button(label: &'static str, on_click: impl Fn() + 'static) -> VDomElement {
    html! {
        <span css={css_item(ButtonType::Active)} on_click={on_click}>{label}</span>
    }
}

#[derive(Clone)]
pub enum ButtonState {
    #[allow(dead_code)]
    None,
    #[allow(dead_code)]
    Disabled {
        label: String,
    },
    #[allow(dead_code)]
    Active {
        label: String,
        action: Rc<dyn Fn()>,
    },
    #[allow(dead_code)]
    Process {
        label: String,
    }
}

impl ButtonState {
    pub fn active(label: impl Into<String>, action: impl Fn() + 'static) -> ButtonState {
        ButtonState::Active {
            label: label.into(),
            action: Rc::new(action)
        }
    }

    pub fn disabled(label: impl Into<String>) -> ButtonState {
        ButtonState::Disabled {
            label: label.into(),
        }
    }

    fn render(self: &ButtonState) -> VDomElement {
        match self {
            Self::None => html!{ <span></span> },
            Self::Disabled { label } => html! {
                <span css={css_item(ButtonType::Disabled)}>{label}</span>
            },
            Self::Active { label, action } => {
                let action = action.clone();

                let on_click = move || {
                    action();
                };

                html!{
                    <span css={css_item(ButtonType::Active)} on_click={on_click}>{label}</span>
                }
            },
            Self::Process { label } => html!{
                <span css={css_item(ButtonType::Process)}>{label}</span>
            }
        }
    }
}

pub struct ButtonComponent {
    value: Computed<ButtonState>,
}

impl ButtonComponent {
    pub fn new(fun: impl Fn() -> ButtonState + 'static) -> VDomComponent {
        let state = ButtonComponent {
            value: Computed::from(fun)
        };

        state.component()
    }

    fn component(self) -> VDomComponent {
        VDomComponent::from(self, |state| {
            let state = state.value.get();
            state.render()
        })
    }
}


