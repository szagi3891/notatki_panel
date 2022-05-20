use vertigo::{VDomElement, html, css, Css};

#[derive(Clone)]
pub enum MessageBoxType {
    Info,
    Error,
}

impl MessageBoxType {
    fn to_string(self) -> &'static str {
        match self {
            Self::Error => "#ff5440",
            Self::Info => "#008000",
        }
    }
}

fn css_error_line(message_type: MessageBoxType) -> Css {
    let message_color = message_type.to_string();

    css!{"
        display: flex;
        background: {message_color};
        text-align: center;
    "}
}

fn css_error_message() -> Css {
    css!{"
        flex-grow: 1;
        padding: 10px 0;
    "}
}

fn css_error_close() -> Css {
    css!{"
        width: 40px;
        padding: 10px 0;
        cursor: pointer;

        :hover {
            background-color: rgba(12, 8, 225, 0.8);
        }
    "}
}

pub fn message_box(message_type: MessageBoxType, message: impl Into<String>, on_close: impl Fn() + 'static) -> VDomElement {
    let message: String = message.into();

    html! {
        <div css={css_error_line(message_type)}>
            <div css={css_error_message()}>
                { message }
            </div>
            <div css={css_error_close()} on_click={on_close}>"x"</div>
        </div>
    }
}
