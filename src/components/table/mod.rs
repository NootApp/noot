use crate::app::GlobalEvent;
use crate::components::table::row::TableRow;
use iced::widget::{column, scrollable, Container};
use iced::{Element, Length};

pub mod row;


#[derive(Debug, Clone)]
pub struct Table {
    pub headers: TableRow,
    pub rows: Vec<TableRow>,
    height: Option<Length>,
    width: Option<Length>,
    scrollable: bool,
}

impl Table {
    pub fn new() -> Table {
        Self {
            headers: TableRow::new(vec!["Headers"]),
            rows: vec![],
            height: None,
            width: None,
            scrollable: true,
        }
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    pub fn height<L: Into<Length>>(mut self, height: L) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn width<L: Into<Length>>(mut self, width: L) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn headers(mut self, headers: TableRow) -> Self {
        self.headers = headers;
        self
    }

    pub fn append(mut self, rows: &[TableRow]) -> Self {
        for row in rows {
            self.rows.push(row.clone());
        }
        self
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        Container::new(
            scrollable(
                column([
                    self.headers.view(),
                    column(self.rows.iter().map(|row| row.view())).into(),
                ])
            )
        ).width(400).height(400).into()
    }


}