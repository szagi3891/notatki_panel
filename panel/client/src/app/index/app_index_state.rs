use vertigo::{
    VDomComponent,
};
use crate::app::App;
use crate::data::Data;

use super::alert::AppIndexAlert;
use super::app_index_render::app_index_render;

#[derive(Clone)]
pub struct AppIndex {
    pub data: Data,
    pub alert: AppIndexAlert,
}

impl AppIndex {
    pub fn new(data: &Data) -> AppIndex {
        let alert = AppIndexAlert::new(data.clone());

        let state = AppIndex {
            data: data.clone(),
            alert,
        };

        state
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_index_render(self, app)
    }

    pub fn keydown(&self, code: String) -> bool {
        if self.alert.is_visible() {
            if code == "Escape" {
                self.alert.close_modal();
                return true;
            }

            //TODO - dodać wskaźnik i nawigację klawiaturą po elemencie z listy wyników

            return false;
        }

        if code == "ArrowUp" {
            self.data.tab.pointer_up();
            return true;
        } else if code == "ArrowDown" {
            self.data.tab.pointer_down();
            return true;
        } else if code == "Escape" {
            self.data.tab.pointer_escape();
            return true;
        } else if code == "ArrowRight" || code == "Enter" {
            self.data.tab.pointer_enter();
            return true;
        } else if code == "ArrowLeft" || code == "Backspace" || code == "Escape" {
            self.data.tab.backspace();
            return true;
        }

        log::info!("klawisz ... {:?} ", code);
        false
    }

    pub fn current_path_dir(&self) -> Vec<String> {
        self.data.tab.dir_select.get()
    }
}
