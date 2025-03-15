use iced::{window, Element, Size, Task};
use iced::widget::{row, Container, text, column};
use iced::window::{icon, Id, Settings};
use crate::app::GlobalEvent;
use crate::components::tree::TreeWidget;
use crate::consts::APP_ICON;
use crate::filesystem::workspace::state::WorkspaceState;

#[derive(Debug)]
pub struct EditorWindow {
    pub id: Id,
    workspace: WorkspaceState,
    file_list: TreeWidget
}


#[derive(Debug, Clone)]
pub enum EditorEvent {

}


impl EditorWindow {
    pub fn new(state: WorkspaceState) -> (Self, Id, Task<Id>) {
        let (id, task) = window::open(Self::config());
        
        let state = Self {
            id,
            workspace: state.clone(),
            file_list: TreeWidget::new(state.manifest.parse_local_path().unwrap()),
        };

        (state, id, task)
    }

    pub fn config() -> Settings {
        Settings {
            size: Size { width: 1920., height: 1000. },
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(icon::from_file_data(APP_ICON, None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        }
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        Container::new(
            row!(
                self.file_list.view(),
                column!(
                    text("Editor Section")
                ),
                column!(
                    text("Right Utility Bar")
                )
            )
        ).into()
    }


}
