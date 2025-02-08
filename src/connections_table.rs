//! # [Ratatui] Table example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::error::Error;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::KeyModifiers;
use itertools::Itertools;
use ratatui::{
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint, Layout, Margin, Rect},
  style::{self, Color, Modifier, Style, Stylize},
  text::Text,
  widgets::{
    Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Table, TableState,
  },
  DefaultTerminal, Frame,
};
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;
use crate::netstat::{get_all_connections, Netstat};

// const PALETTES: [tailwind::Palette; 4] = [
//   tailwind::BLUE,
//   tailwind::EMERALD,
//   tailwind::INDIGO,
//   tailwind::RED,
// ];
// const INFO_TEXT: [&str; 2] = [
//   "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
//   "(Shift + →) next color | (Shift + ←) previous color",
// ];

const ITEM_HEIGHT: usize = 1;

// fn main() -> Result<(), Box<dyn Error>> {
//   let terminal = ratatui::init();
//   let app_result = ConnectionsTable::new().run(terminal);
//   ratatui::restore();
//   app_result
// }
struct TableColors {
  buffer_bg: Color,
  header_bg: Color,
  header_fg: Color,
  row_fg: Color,
  selected_row_style_fg: Color,
  selected_column_style_fg: Color,
  selected_cell_style_fg: Color,
  normal_row_color: Color,
  alt_row_color: Color,
  footer_border_color: Color,
}

impl TableColors {
  const fn new(color: &tailwind::Palette) -> Self {
    Self {
      buffer_bg: Color::White, // color.c400, // tailwind::SLATE.c950,
      header_bg: Color::LightBlue,
      header_fg: Color::Black, // tailwind::SLATE.c200,
      row_fg: Color::White, //tailwind::SLATE.c200,
      selected_row_style_fg: Color::White, // color.c400,
      selected_column_style_fg: Color::White, // color.c400,
      selected_cell_style_fg: Color::White, //color.c600,
      normal_row_color: Color::White, // tailwind::SLATE.c950,
      alt_row_color: Color::White, //tailwind::SLATE.c900,
      footer_border_color: color.c400,
    }
  }
}

// struct Data {
//   name: String,
//   address: String,
//   email: String,
// }
//
// impl Data {
//   const fn ref_array(&self) -> [&String; 3] {
//     [&self.name, &self.address, &self.email]
//   }
//
//   fn name(&self) -> &str {
//     &self.name
//   }
//
//   fn address(&self) -> &str {
//     &self.address
//   }
//
//   fn email(&self) -> &str {
//     &self.email
//   }
// }

pub struct ConnectionsTable {
  state: TableState,
  items: Vec<Netstat>,
  // longest_item_lens: (u16, u16, u16, u16, u16, u16, u16), // order is (name, address, email)
  scroll_state: ScrollbarState,
  // colors: TableColors,
  // color_index: usize,
}

impl ConnectionsTable {
  pub fn new() -> Self {
    let data_vec = get_all_connections(); // generate_fake_names();
    Self {
      state: TableState::default().with_selected(0),
      // longest_item_lens: (10, 20, 10, 20, 10, 10, 10) , //constraint_len_calculator(&data_vec),
      scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
      // colors: TableColors::new(&PALETTES[0]),
      // color_index: 0,
      items: data_vec,
    }
  }
  pub fn next_row(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
  }

  pub fn previous_row(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
  }

  // pub fn next_column(&mut self) {
  //   self.state.select_next_column();
  // }
  //
  // pub fn previous_column(&mut self) {
  //   self.state.select_previous_column();
  // }
  //
  // pub fn next_color(&mut self) {
  //   self.color_index = (self.color_index + 1) % PALETTES.len();
  // }
  //
  // pub fn previous_color(&mut self) {
  //   let count = PALETTES.len();
  //   self.color_index = (self.color_index + count - 1) % count;
  // }
  //
  // pub fn set_colors(&mut self) {
  //   self.colors = TableColors::new(&PALETTES[self.color_index]);
  // }

  pub fn run(mut self, frame: &mut Frame, rect: Rect) {
    // loop {
      // terminal.draw(|frame| self.draw(frame))?;
      self.draw(frame, rect);



      // if let Event::Key(key) = event::read().unwrap() {
      //   if key.kind == KeyEventKind::Press {
      //     let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
      //     match key.code {
      //       // KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
      //       KeyCode::Char('t') | KeyCode::Down => self.next_row(),
      //       KeyCode::Char('y') | KeyCode::Up => self.previous_row(),
      //       // KeyCode::Char('l') | KeyCode::Right if shift_pressed => self.next_color(),
      //       // KeyCode::Char('h') | KeyCode::Left if shift_pressed => {
      //       //   self.previous_color();
      //       // }
      //       // KeyCode::Char('l') | KeyCode::Right => self.next_column(),
      //       // KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
      //       _ => {}
      //     }
      //   }
      // }
    // }
  }

  fn draw(&mut self, frame: &mut Frame, rect: Rect) {
    // let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
    // let rects = vertical.split(frame.area());

    // self.set_colors();

    self.render_table(frame, rect);
    // self.render_scrollbar(frame, rect);
    // self.render_footer(frame, rects[1]);
  }

  fn render_table(&mut self, frame: &mut Frame, area: Rect) {
    let header_style = Style::default()
      .bold()
      .fg(Color::Black)
      .bg(Color::Blue);
    let selected_row_style = Style::default();
      // .add_modifier(Modifier::REVERSED)
      // .fg(self.colors.selected_row_style_fg);
    let selected_col_style = Style::default().fg(Color::Black);
    // let selected_cell_style = Style::default()
    //   .add_modifier(Modifier::REVERSED)
    //   .fg(self.colors.selected_cell_style_fg);

    let header = ["Protocol", "Local Ip", "Local Port", "Remote Ip", "Remote Port", "State", "PIDs"]
      .into_iter()
      .map(Cell::from)
      .collect::<Row>()
      .style(header_style)
      .height(1);
    // let rows = self.items.iter().enumerate().map(|(i, data)| {
    //   let color = match i % 2 {
    //     0 => self.colors.normal_row_color,
    //     _ => self.colors.alt_row_color,
    //   };
    //   let item = data;//.ref_array();
    //   item.into_iter()
    //     .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
    //     .collect::<Row>()
    //     .style(Style::new().fg(self.colors.row_fg).bg(color))
    //     .height(4)
    // });

    let mut rows  = Vec::new();
    for conn in self.items.iter().clone() {
      let mut cols = Vec::new();
      cols.push(Cell::from(conn.protocol.to_string()));
      cols.push(Cell::from(conn.local_addr.to_string()));
      cols.push(Cell::from(conn.local_port.to_string()));
      cols.push(Cell::from(conn.remote_addr.to_string()));
      cols.push(Cell::from(conn.remote_port.to_string()));
      cols.push(Cell::from(conn.state.to_string()).style(self.get_font_style_by_state(conn.state.to_string())));
      cols.push(Cell::from(conn.pids.get(0).unwrap().to_string()));

      rows.push(Row::new(cols).style(self.get_style_by_state(conn.state.to_string())));
    }

    let bar = " █ ";
    let t = Table::new(
      rows,
      [
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(12),
        Constraint::Length(15),
        Constraint::Length(12),
        Constraint::Length(15),
        Constraint::Length(10),
      ],
    )
      .header(header)
      .row_highlight_style(selected_row_style)
      .column_highlight_style(selected_col_style)
      // .cell_highlight_style(selected_cell_style)
      .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
      ]))
      .bg(Color::White)
      .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(t, area, &mut self.state);

    frame.render_stateful_widget(
      Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None),
      area.inner(Margin {
        vertical: 1,
        horizontal: 1,
      }),
      &mut self.scroll_state,
    );
  }

  // fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
  //   frame.render_stateful_widget(
  //     Scrollbar::default()
  //       .orientation(ScrollbarOrientation::VerticalRight)
  //       .begin_symbol(None)
  //       .end_symbol(None),
  //     area.inner(Margin {
  //       vertical: 1,
  //       horizontal: 1,
  //     }),
  //     &mut self.scroll_state,
  //   );
  // }

  // fn render_footer(&self, frame: &mut Frame, area: Rect) {
  //   let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
  //     .style(
  //       Style::new()
  //         .fg(self.colors.row_fg)
  //         .bg(self.colors.buffer_bg),
  //     )
  //     .centered()
  //     .block(
  //       Block::bordered()
  //         .border_type(BorderType::Double)
  //         .border_style(Style::new().fg(self.colors.footer_border_color)),
  //     );
  //   frame.render_widget(info_footer, area);
  // }

  fn get_style_by_state(&self, state: String) -> Style {
    if (state == "ESTABLISHED") {
      Style::new().black().on_green().bold()
    } else if (state == "LISTEN") {
      Style::new().black().on_yellow()
    } else {
      Style::new().black().on_white()
    }
  }

  fn get_font_style_by_state(&self, state: String) -> Style {
    if (state == "ESTABLISHED") {
      return Style::new().bold()
    }
    Style::new().not_bold()
  }
}

// fn generate_fake_names() -> Vec<Data> {
//   use fakeit::{address, contact, name};
//
//   (0..20)
//     .map(|_| {
//       let name = name::full();
//       let address = format!(
//         "{}\n{}, {} {}",
//         address::street(),
//         address::city(),
//         address::state(),
//         address::zip()
//       );
//       let email = contact::email();
//
//       Data {
//         name,
//         address,
//         email,
//       }
//     })
//     .sorted_by(|a, b| a.name.cmp(&b.name))
//     .collect()
// }

// fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16) {
//   let name_len = items
//     .iter()
//     .map(Data::name)
//     .map(UnicodeWidthStr::width)
//     .max()
//     .unwrap_or(0);
//   let address_len = items
//     .iter()
//     .map(Data::address)
//     .flat_map(str::lines)
//     .map(UnicodeWidthStr::width)
//     .max()
//     .unwrap_or(0);
//   let email_len = items
//     .iter()
//     .map(Data::email)
//     .map(UnicodeWidthStr::width)
//     .max()
//     .unwrap_or(0);
//
//   #[allow(clippy::cast_possible_truncation)]
//   (name_len as u16, address_len as u16, email_len as u16)
// }
