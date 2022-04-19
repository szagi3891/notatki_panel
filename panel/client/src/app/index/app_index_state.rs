use std::rc::Rc;
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
    pub app: App,
    pub alert: AppIndexAlert,
}

impl AppIndex {
    pub fn component(app: &App) -> VDomComponent {
        let state_data = app.data.clone();

        let (alert, alert_view) = AppIndexAlert::new(app.data.clone());

        let state = AppIndex {
            data: state_data,
            app: app.clone(),
            alert,
        };

        app_index_render(alert_view, state)
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

    pub fn current_edit(&self) {
        let full_path = self.data.tab.full_path.get_value();
        self.app.redirect_to_content(&full_path);
    }

    pub fn create_file(&self) {
        self.app.redirect_to_new_content();
    }

    pub fn redirect_to_mkdir(&self) {
        self.app.redirect_to_mkdir();
    }

    pub fn current_rename(&self) {
        let path = self.data.tab.dir_select.get_value();
        let select_item = self.data.tab.current_item.get_value();

        if let Some(select_item) = select_item.as_ref() {
            self.app.redirect_to_rename_item(&path, select_item);
        } else {
            log::error!("current_rename fail");
        }
    }

    pub fn current_path_dir(&self) -> Rc<Vec<String>> {
        self.data.tab.dir_select.get_value()
    }
}
