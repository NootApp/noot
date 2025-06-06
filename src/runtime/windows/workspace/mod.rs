use std::path::PathBuf;
use std::time::{Duration, Instant};
use rust_i18n::t;
use iced::{window, Theme, Task as IcedTask, Size, Length, Background, color, Border, Padding};
use iced::widget::{center, column, container, horizontal_space, mouse_area, row, scrollable, text, text_input, vertical_space};
use iced::widget::container::Style;
use iced::window::{icon, Id, Position, Settings};
use material_icons::Icon;
use stringcase::kebab_case;
use regex::Regex;
use crate::consts::{APP_ICON, APP_NAME, FONT_BOLD, TEXT_INPUT_INVALID};
use crate::runtime::messaging::{Message, WindowMessage, WindowMessageKind};
use crate::runtime::{Element, Task, GLOBAL_STATE};
use crate::runtime::windows::DesktopWindow;
use crate::runtime::windows::editor::messaging::EditorMessage;
use crate::runtime::windows::workspace::WorkspaceWindowMessageKind::{CreateWorkspace, LoadWorkspaceFromClick, PhaseChange, WorkspaceHovered};
use crate::storage::workspace::{render_directory, WorkspaceError, WorkspaceManager};
use crate::utils::components::buttons::{button_with_icon, RichButton, NEON_BUTTON_CONTAINER};

#[derive(Debug)]
pub struct WorkspaceWindow {
    pub id: Id,
    phase: WorkspacePhase,
    hovered_workspace: String,
    workspace_dir: PathBuf,
    animation_start: Instant
}

#[derive(Debug, Clone)]
pub enum WorkspacePhase {
    Menu,
    New(NewWorkspaceData),
    Local(String),
}


#[derive(Debug, Clone)]
pub struct NewWorkspaceData {
    pub name: String,
    pub path: String,
    pub name_valid: (bool, String),
    pub path_valid: (bool, String),
}

impl NewWorkspaceData {
    pub fn new() -> NewWorkspaceData {
        NewWorkspaceData {
            name: "My Cool Workspace".to_string(),
            path: render_directory(
                "$PROJECT_DIR/my-cool-workspace".to_string(),
                GLOBAL_STATE.lock().unwrap().config.workspace_directory.clone()
            ).to_str().unwrap().to_string(),
            name_valid: (true, "".to_string()),
            path_valid: (true, "".to_string()),
        }
    }

    pub fn update_name(mut self, new: String) -> Self {
        let matcher = Regex::new(r"[^a-zA-Z0-9_\-\s]").unwrap();

        self.name = new;

        if matcher.is_match(&self.name) {
            info!("Name is invalid: '{}'", self.name.as_str());
            self.name_valid = (false, t!("windows.workspace-manager.new.fields.name.validation.invalid-characters").to_string());
            return self;
        } else if self.name.len() < 2 {
            self.name_valid = (false, t!("windows.workspace-manager.new.fields.name.validation.too-short").to_string());
            return self;
        } else if self.name.len() > 30 {
            self.name_valid = (false, t!("windows.workspace-manager.new.fields.name.validation.too-long").to_string());
            return self;
        } else {
            info!("Name is valid: '{}'", self.name.as_str());
            self.name_valid = (true, "".to_string());
        }


        let mut temp_path = PathBuf::from(&self.path);
        // Delete folder name
        temp_path.pop();

        temp_path.push(kebab_case(&self.name));

        self.update_path(temp_path.to_str().unwrap().to_string())
    }

    pub fn update_path(mut self, new: String) -> Self {
        let matcher = Regex::new(r"[^a-zA-Z0-9_\-\\/:\s]").unwrap();

        self.path = new;

        if matcher.is_match(&self.path) {
            self.path_valid = (false, t!("windows.workspace-manager.new.fields.path.validation.invalid-characters").to_string());
        } else {
            self.path_valid = (true, "".to_string());
        }

        self
    }
}

#[derive(Debug, Clone)]
pub enum WorkspaceWindowMessageKind {
    Tick,
    PhaseChange(WorkspacePhase),
    CreateWorkspace(String, String),
    LoadWorkspaceFromClick(String),
    WorkspaceHovered(String)
}

#[derive(Debug, Clone)]
pub struct WorkspaceWindowMessage {
    kind: WorkspaceWindowMessageKind,
    source_id: Option<Id>,
}

impl WorkspaceWindowMessage {
    pub fn new(kind: WorkspaceWindowMessageKind, source_id: Option<Id>) -> Self {
        Self {
            kind,
            source_id,
        }
    }

    pub fn phase_change(phase: WorkspacePhase, source_id: Id) -> Self {
        Self::new(PhaseChange(phase), Some(source_id))
    }

    pub fn create_workspace(metadata: NewWorkspaceData, source_id: Id) -> Self {
        Self::new(CreateWorkspace(metadata.name, metadata.path), Some(source_id))
    }

    pub fn load_from_click<S: Into<String>>(workspace: S, source_id: Id) -> Self {
        Self::new(LoadWorkspaceFromClick(workspace.into()), Some(source_id))
    }

    pub fn hovered<S: Into<String>>(workspace: S, source_id: Id) -> Self {
        Self::new(WorkspaceHovered(workspace.into()), Some(source_id))
    }
}


impl Into<Message> for WorkspaceWindowMessage {
    fn into(self) -> Message {
        WindowMessage{
            kind: WindowMessageKind::Workspace(self.clone()),
            source_id: self.source_id
        }.into()
    }
}

impl Into<WorkspaceWindowMessage> for WindowMessageKind {
    fn into(self) -> WorkspaceWindowMessage {
        let WindowMessageKind::Workspace(message) = self else { panic!("Somehow got invalid workspace event") };
        message
    }
}

impl Into<EditorMessage> for WindowMessageKind {
    fn into(self) -> EditorMessage {
        let WindowMessageKind::Editor(message) = self else { panic!("Somehow got invalid workspace event") };
        message
    }
}

impl WorkspaceWindow {
    pub(crate) fn new() -> (WorkspaceWindow, IcedTask<Id>) {

        let (id, task) = window::open(Self::settings());
        info!("Spawning window with ID: {}", id);
        (
            Self {
                id,
                phase: WorkspacePhase::Menu,
                hovered_workspace: "".to_string(),
                workspace_dir: GLOBAL_STATE.lock().unwrap().config.workspace_directory.clone(),
                // Start the animation 200 ms ago (so that it is already finished)
                animation_start: Instant::now().checked_sub(Duration::from_millis(200)).unwrap()
            },
            task
        )
    }

    fn render_landing_page(&self) -> Element {
        let mut container_padding = Padding::new(5.);
        container_padding.left = 10.;
        container_padding.right = 10.;

        let workspace_containers: Vec<Element> = GLOBAL_STATE.lock().unwrap().workspaces.iter().map(|(id, workspace)| {
            let id = id.clone();
            let id2 = id.clone();
            let is_hovered = id2 == self.hovered_workspace.as_str();
            column!(
                        mouse_area(
                            match is_hovered {
                                true => {
                                    container(
                                        container(
                                            column!(
                                                row!(text(workspace.name.clone()), horizontal_space().width(Length::Fill), text(workspace.last_accessed.format("%y-%m-%d %H:%M:%S").to_string())),
                                                row!(text(workspace.disk_path.clone()))
                                            )
                                        )
                                        .style(move |_| {
                                            Style {
                                                text_color: None,
                                                background: Some(Background::Color(color!(0x1a1a1a))),
                                                border: Border::default().rounded(5.).color(color!(0x1a1a1a)),
                                                shadow: Default::default(),
                                            }
                                        })
                                        .padding(container_padding.clone())
                                    )
                                    .style(move |_| {
                                        NEON_BUTTON_CONTAINER.clone()
                                    })
                                    .padding(1)
                                },
                                false => {
                                    container(
                                    container(
                                        column!(
                                            row!(text(workspace.name.clone()), horizontal_space().width(Length::Fill), text(workspace.last_accessed.format("%y-%m-%d %H:%M:%S").to_string())),
                                            row!(text(workspace.disk_path.clone()))
                                        )
                                    )
                                    .style(move |_| {
                                        Style {
                                            text_color: None,
                                            background: None,
                                            border: Border::default().width(1).rounded(5.).color(color!(0x1a1a1a)),
                                            shadow: Default::default(),
                                        }
                                    })
                                    .padding(container_padding.clone())
                                    ).padding(1)
                                }
                            }

                        )
                            .on_press(WorkspaceWindowMessage::load_from_click(&id, self.id).into())
                            .on_enter(WorkspaceWindowMessage::hovered(&id, self.id).into())
                            .on_exit(WorkspaceWindowMessage::hovered("", self.id).into()),
                        vertical_space().height(5.)
                    ).into()
        }).collect();


        container(
            column!(
                        vertical_space().height(Length::FillPortion(1)),
                        center(text(t!("windows.workspace-manager.menu.workspaces")).font(FONT_BOLD).size(32)).height(Length::Shrink),
                        center(
                            row!(
                                container(
                                    RichButton::new_with_icon(
                                        Icon::Add,
                                    text(t!("windows.workspace-manager.menu.buttons.new"))
                                    )
                                    .on_press(WorkspaceWindowMessage::phase_change(WorkspacePhase::New(NewWorkspaceData::new()), self.id).into())
                                ).style(|_| {
                                    NEON_BUTTON_CONTAINER.clone()
                                }).padding(1),
                                horizontal_space().width(5),
                                RichButton::new_with_icon(Icon::FolderOpen, text(t!("windows.workspace-manager.menu.buttons.open-folder"))),
                                horizontal_space().width(5),
                                RichButton::new_with_icon(Icon::CloudDownload, text(t!("windows.workspace-manager.menu.buttons.cloud-download"))),
                            ).width(520.)
                        ),
                        center(
                            if workspace_containers.is_empty() {
                                container(text(t!("windows.workspace-manager.menu.recent.none"))).width(550.).height(Length::Shrink).into()
                            } else {
                                container(
                                    scrollable(
                                        column!(
                                            text(t!("windows.workspace-manager.menu.recent.some")).size(22).font(FONT_BOLD),
                                            column(
                                                workspace_containers
                                            )
                                        )
                                    )
                                )
                                    .height(400.)
                                    .width(550.)
                            }
                        )


                    )
        ).into()
    }

    fn render_create_workspace_page<'a>(&'a self, metadata: &'a NewWorkspaceData) -> Element<'a> {
        container(
            column!(
                        vertical_space().height(Length::FillPortion(1)),
                        center(text(t!("windows.workspace-manager.new.text.heading")).font(FONT_BOLD).size(32)).height(Length::Shrink),
                        center(
                            container(
                                column!(
                                    container(
                                        column!(
                                            row!(
                                                text(t!("windows.workspace-manager.new.fields.name.text")),
                                                horizontal_space().width(Length::Fill),
                                                text(metadata.name_valid.1.as_str()).color(color!(TEXT_INPUT_INVALID))
                                            ),
                                            text_input("My cool workspace", metadata.name.as_str())
                                                .on_input(|content| WorkspaceWindowMessage::phase_change(WorkspacePhase::New(metadata.clone().update_name(content)), self.id).into())
                                        )
                                    ),
                                    vertical_space().height(10.),
                                    container(
                                        column!(
                                            row!(
                                                text(t!("windows.workspace-manager.new.fields.path.text")),
                                                horizontal_space().width(Length::Fill),
                                                text(metadata.path_valid.1.as_str()).color(color!(TEXT_INPUT_INVALID))
                                            ),
                                            text_input(
                                                render_directory(
                                                    format!("$PROJECT_DIR/{}", kebab_case(metadata.name.as_str())),
                                                    self.workspace_dir.clone()
                                                ).to_str().unwrap(),
                                                metadata.path.as_str()
                                            )
                                                .on_input(move |content| {
                                                    WorkspaceWindowMessage::phase_change(WorkspacePhase::New(metadata.clone().update_path(content)), self.id).into()
                                                })
                                        )
                                    ),
                                    vertical_space().height(10),
                                    row!(
                                        horizontal_space().width(Length::FillPortion(2)),
                                        button_with_icon(Icon::Cancel, t!("windows.workspace-manager.new.buttons.cancel"))
                                         .on_press(WorkspaceWindowMessage::phase_change(WorkspacePhase::Menu, self.id).into()),
                                        horizontal_space().width(5.),
                                        RichButton::new_with_icon(Icon::Add, text(t!("windows.workspace-manager.new.buttons.create")))
                                            // .style(|_, _| {
                                            //     ButtonStyle::new()
                                            //         .with_background_color(BUTTON_CONFIRM_BACKGROUND)
                                            //         .compile()
                                            // })
                                            .on_press(WorkspaceWindowMessage::create_workspace(metadata.clone(), self.id).into())
                                    )
                                )
                            ).width(550.)
                        ),
                        vertical_space().height(Length::FillPortion(1)),
                    )
        ).into()
    }
}

impl DesktopWindow<WorkspaceWindow, WorkspaceWindowMessage, Message> for WorkspaceWindow {
    fn settings() -> Settings {
        Settings {
            size: Size::new(720., 700.),
            position: Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(icon::from_file_data(APP_ICON, None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        }
   }

    fn title(&self) -> String {
        t!("windows.workspace-manager.title", name = APP_NAME).to_string()
    }

    fn theme(&self) -> Theme {
        Theme::SolarizedDark
    }

    fn update(&mut self, message: WorkspaceWindowMessage) -> Task {
        match message.kind {
            PhaseChange(new_phase) => {
                debug!("Phase change: {:?} -> {:?}", self.phase, new_phase);
                self.phase = new_phase;
                self.animation_start = Instant::now();
                Task::none()
            },
            CreateWorkspace(name, path) => {
                let wm = WorkspaceManager::create(name, path);
                if let Ok(wm) = wm {
                    GLOBAL_STATE.lock().unwrap().workspaces.insert(wm.source.id.clone(), wm.source.clone());

                    return Task::done(Message::open_workspace(wm.source.id.clone())).chain(self.close())
                } else {
                    let err = wm.unwrap_err();
                    match err {
                        WorkspaceError::WorkspaceInvalid(_message) => {}
                        WorkspaceError::WorkspaceNotFound(_message) => {}
                        WorkspaceError::RootNotFound(_message) => {}
                        _ => {}
                    }
                }

                Task::none()
            }
            WorkspaceHovered(id) => {
                self.hovered_workspace = id;
                Task::none()
            }
            LoadWorkspaceFromClick(id) => {
                Task::done(Message::open_workspace(id)).chain(self.close())
            }
            _ => Task::none()
        }
    }

    fn view(&self) -> Element {
        if self.animation_start.elapsed().as_millis() < 200 {
            let timecode = self.animation_start.elapsed().as_millis();
            let width = 720.;
            let frame_duration = 16;
            info!("TC: {}, W: {}, FD: {}, FR: {}", timecode, width, frame_duration, timecode/frame_duration);
            // let w1 =
        }
        match &self.phase {
            WorkspacePhase::Menu => self.render_landing_page(),
            WorkspacePhase::New(metadata) => self.render_create_workspace_page(metadata),
            _ => text("Not Implemented").into()
        }
    }

    fn close(&mut self) -> Task {
        window::close(self.id).chain(Message::window_close(self.id).into())
    }
}
