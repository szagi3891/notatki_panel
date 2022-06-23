use std::rc::Rc;

use vertigo::{
    VDomElement,
    Css, Computed, DomElement, create_node,
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

impl PartialEq for ButtonState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::None, Self::None) => true,
            (
                Self::Disabled { label: label1 },
                Self::Disabled { label: label2 }
            ) => {
                label1.eq(label2)
            },
            (
                Self::Active { label: label1, action: action1 },
                Self::Active { label: label2, action: action2 },
            ) => {
                let compare1 = label1.eq(label2);
                let compare2 = Rc::as_ptr(action1) == Rc::as_ptr(action2);

                compare1 && compare2
            },
            (
                Self::Process { label: label1 },
                Self::Process { label: label2 }
            ) => {
                label1.eq(label2)
            },
            (_, _) => false,
        }
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


    pub fn render(value: Computed<ButtonState>) -> DomElement {
        create_node("div")
        .value(value, |value| {
            match value {
                ButtonState::None => {
                    //html!{ <span></span> },
                    create_node("span")
                },
                ButtonState::Disabled { label } => {
                    create_node("span")
                        .css(css_item(ButtonType::Disabled))
                        .text(label)
                //     html! {
                //     <span css={css_item(ButtonType::Disabled)}>{label}</span>
                // },
                },
                ButtonState::Active { label, action } => {
                    let on_click = move || {
                        action();
                    };

                    create_node("span")
                        .css(css_item(ButtonType::Active))
                        .on_click(on_click)
                        .text(label)
                    
                    // let action = action.clone();

                    // let on_click = move || {
                    //     action();
                    // };

                    // html!{
                    //     <span css={css_item(ButtonType::Active)} on_click={on_click}>{label}</span>
                    // }
                },
                ButtonState::Process { label } => {
                    create_node("span")
                        .css(css_item(ButtonType::Process))
                        .text(label)
                //     html!{
                //     <span css={css_item(ButtonType::Process)}>{label}</span>
                // }
                }
            }
        })

    // fn render(self: &ButtonState) -> VDomElement {
    //     match self {
    //         Self::None => html!{ <span></span> },
    //         Self::Disabled { label } => html! {
    //             <span css={css_item(ButtonType::Disabled)}>{label}</span>
    //         },
    //         Self::Active { label, action } => {
    //             let action = action.clone();

    //             let on_click = move || {
    //                 action();
    //             };

    //             html!{
    //                 <span css={css_item(ButtonType::Active)} on_click={on_click}>{label}</span>
    //             }
    //         },
    //         Self::Process { label } => html!{
    //             <span css={css_item(ButtonType::Process)}>{label}</span>
    //         }
    //     }
    }
}

// fn component(self) -> VDomComponent {
//     VDomComponent::from(self, |state| {
//         let state = state.value.get();
//         state.render()
//     })
// }


