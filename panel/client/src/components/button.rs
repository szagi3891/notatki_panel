use std::rc::Rc;

use vertigo::{
    Css, Computed, dom, DomNode,
};

use vertigo::{css};

enum ButtonType {
    Disabled,
    Active,
    Process,
}

#[derive(Clone)]
pub struct CallbackRc {
    id: u64,
    callback: Rc<dyn Fn()>,
}

impl CallbackRc {
    pub fn new(callback: impl Fn() + 'static) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            id,
            callback: Rc::new(callback),
        }
    }

    pub fn run(&self) {
        (self.callback)();
    }
}

impl PartialEq for CallbackRc {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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


pub fn button(label: &'static str, on_click: impl Fn() + 'static) -> DomNode {
    dom! {
        <span css={css_item(ButtonType::Active)} on_click={on_click}>{label}</span>
    }
}

#[derive(Clone, PartialEq)]
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
        action: CallbackRc,
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
            action: CallbackRc::new(action)
        }
    }

    pub fn disabled(label: impl Into<String>) -> ButtonState {
        ButtonState::Disabled {
            label: label.into(),
        }
    }

    pub fn process(label: impl Into<String>) -> ButtonState {
        ButtonState::Process {
            label: label.into(),
        }
    }

    pub fn render(value: Computed<ButtonState>) -> DomNode {
        value.render_value_option(|value| {
            match value {
                ButtonState::None => {
                    None
                },
                ButtonState::Disabled { label } => {
                    Some(dom! {
                        <span css={css_item(ButtonType::Disabled)}>{label}</span>
                    })
                },
                ButtonState::Active { label, action } => {
                    let on_click = move || {
                        action.run();
                    };

                    Some(dom!{
                        <span css={css_item(ButtonType::Active)} on_click={on_click}>{label}</span>
                    })
                },
                ButtonState::Process { label } => {
                    Some(dom!{
                        <span css={css_item(ButtonType::Process)}>{label}</span>
                    })
                }
            }
        })
    }
}
