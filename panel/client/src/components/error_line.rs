use vertigo::{VDomElement, html, css, Css};

fn css_error_line() -> Css {
    css!{"
        display: flex;
        background: #ff5440;
        border: 1px solid rgba(0, 0, 0, 0.2);
        transition: 300ms all ease-in;
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
            background-color: green;
        }
    "}
}

pub fn error_line(message: impl Into<String>, on_close: impl Fn() + 'static) -> VDomElement {
    let message = message.into();

    html! {
        <div css={css_error_line()}>
            <div css={css_error_message()}>
                { message }
            </div>
            <div css={css_error_close()} on_click={on_close}>"x"</div>
        </div>
    }
}