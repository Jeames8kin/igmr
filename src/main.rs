mod util;
mod json_parser;
mod app;

extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde::{Serialize, Deserialize};

use crate::app::App;

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
  backend::TermionBackend,
  layout::{Constraint, Direction, Layout},
  widgets::{Block, Borders, Row, Table, TableState, Gauge, Paragraph},
  Terminal,
  style::{Color, Modifier, Style},
  text::{Text, Span, Spans}
};

use util::{
  event::{Event, Events},
  StatefulList,
};



fn main() {

  json_parser::main().unwrap();

  start_ui();
    
  let mut app = App::default();

  fn start_ui() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();



    loop {
        
      terminal.draw(|f| {

        let chunks = Layout::default()
          .direction(Direction::Vertical)
          .constraints(
            [
              Constraint::Percentage(10),
              Constraint::Percentage(20),
              Constraint::Percentage(70),
            ]
            .as_ref(),
                  )
          .margin(1)
          .split(f.size());

          let block = Block::default().title("Block 1").borders(Borders::ALL); //Block 1
          let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Gauge1"))
            .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
            .percent(100);
          
            f.render_widget(block, chunks[0]);     
            f.render_widget(gauge, chunks[0]);  

          let block = Block::default().title("Block 2").borders(Borders::ALL); //Block 2
          f.render_widget(block, chunks[1]);

          let block = Block::default().title("Block 3").borders(Borders::ALL); //Block 3
          let text = vec![
            Spans::from("text1"),
            Spans::from("text2"),
            Spans::from("text3"),
          ];
          let paragraph1 = Paragraph::new(text)
            .block(Block::default().title("Paragraph1").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black));
          f.render_widget(paragraph1, chunks[2]);
          f.render_widget(block, chunks[2]);
                


      })?;

      if let Event::Input(input) = events.next()? {
        if let Key::Char('q') = input {
          break;
              
                
          }
        }
      }
    Ok(())
  }
}
