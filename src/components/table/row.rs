use crate::app::GlobalEvent;
use iced::widget::{horizontal_space, row, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub struct TableRow {
    fields: Vec<String>,
    width: Option<Length>,
    height: Option<Length>,
}


impl TableRow {
    pub fn new<S: ToString>(fields: Vec<S>) -> TableRow {
        TableRow { fields: fields.iter().map(|v| v.to_string()).collect(), width: None, height: None }
    }

    pub fn width<L: Into<Length>>(&mut self, width: L) -> &mut TableRow {
        self.width = Some(width.into());
        self
    }

    pub fn height<L: Into<Length>>(&mut self, height: L) -> &mut TableRow {
        self.height = Some(height.into());
        self
    }
    pub fn view(&self) -> Element<GlobalEvent> {
        let cells: Vec<Element<GlobalEvent>> = self.fields.iter().map(|field| {row([text(field.clone()).into(), horizontal_space().into()]).into()}).collect();

        let mut row = row(cells);

        if let Some(width) = self.width {
            row = row.width(width);
        }

        if let Some(height) = self.height {
            row = row.height(height);
        }


        row.into()
    }
}


