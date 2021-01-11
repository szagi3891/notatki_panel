use vertigo::{
    VDomNode,
    node_attr,
    Css,
    computed::{
        Computed,
    }
};

use super::state::State;

const GLOBAL_RESET: &'static str = "html, body {
    margin: 0;
    padding: 0;
    border: 0;
}";

fn css_wrapper() -> Css {
    Css::one("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        padding: 0;
        width: 100vw;
        height: 100vh;
        box-sizing: border-box;
    ")
}

fn css_header() -> Css {
    Css::one("
        height: 25px;
        border-bottom: 1px solid black;
        line-height: 25px;
        padding: 0 5px;
    ")
}

pub fn render_header(state: &Computed<State>) -> VDomNode {
    let state = state.get_value();
    let current_path = state.current_path.get_value();

    let mut path_chunks: Vec<&str> = Vec::new();
    for path_item in current_path.iter() {
        path_chunks.push(path_item);
    }
    let path_for_view = path_chunks.join(" / ");

    use node_attr::{build_node, text, css};
    build_node("div", vec!(
        css(css_header()),
        text(path_for_view)
    ))
}

fn css_content() -> Css {
    Css::one("
        flex-grow: 1;
        display: flex;
        border-bottom: 1px solid black;
    ")
}

fn css_content_list() -> Css {
    Css::one("
        flex-grow: 1;
        border-right: 1px solid black;
        padding: 5px;
    ")
}

fn css_content_content() -> Css {
    Css::one("
        flex-grow: 1;
        padding: 5px;
    ")
}

pub fn render_content(state: &Computed<State>) -> VDomNode {
    use node_attr::{build_node, text, css, node, on_click};

    let content_click = {
        let state = state.get_value();
        move || {
            state.push_path();
        }
    };

    build_node("div", vec!(
        css(css_content()),
        node("div", vec!(
            css(css_content_list()),
            text("lista plikow")
        )),
        node("div", vec!(
            css(css_content_content()),
            on_click(content_click),
            text("content ...")
        )),
    ))
}


fn css_footer() -> Css {
    Css::one("
        line-height: 25px;
        padding: 0 5px;
    ")
}

pub fn render_footer(state: &Computed<State>) -> VDomNode {
    use node_attr::{build_node, text, css, node};
    build_node("div", vec!(
        css(css_footer()),
        text("lista plików które zostały zmodyfikowane ale nie zapisane")
    ))
}

/*
    path    - cały wiersz
    files content   - dwie kolumny
    zmodyfikowane sciezki - stopka, pliki które są zmodyfikowane
*/

pub fn render(state: &Computed<State>) -> VDomNode {
    use node_attr::{build_node, text, css, node, component};
    build_node("div", vec!(
        node("style", vec!(
            text(GLOBAL_RESET)
        )),
        css(css_wrapper()),
        component(state.clone(), render_header),
        component(state.clone(), render_content),
        component(state.clone(), render_footer),
    ))
}

/*

domyślny korzeń od którego się zaczyna wszystko
main ---> 1


baza danych typu klucz wartość (git jako nośnik danych)

collections /
    notatki /
        p001            --> element 1
        d023/p032       --> element 23032

    ta warstwa zapewnia tylko zapis i odczyt tych elementów

    save
        fajnie byłoby mieć jakiś atomowy zapis ...




    element:
        ostatni timestamp zmiany ...
        data: ....      dane binarne

    2018-01-26T18:30:09.453Z

    Z - moze byc markerem oznaczajacym koniec daty.
    ALbo, mozna wybrac inna literke

    
    zapis mozliwy bylby tylko pod warunkiem ze w request podalismy timestamp aktualnej tresci
    nowy timestamp musi byc wiekszy od tego ktory jest obecnie

    



    File {
        id: u32,
        title: String,
        content: String,
    }
    Dir {
        id: u32,
        title: String,
        child: Vec<u32>         --> rozjazdowka na kolejne dzieci z trescia
    }


    GET /get/:id        - pobranie elementu
    POST /save/:id       - zapisanie nowego elementu
        {
            data: dane do zapisu
            timesamp - data ostatniego zapisu, potrzebne w celu potwierdzenia czy ta zmiana nadal jest świeza
        }
*/