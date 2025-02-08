use std::io;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use futures::SinkExt;
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::prelude::Text;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Masked, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState};
use crate::my_ips::{getMyIps, IP_Address};
use crate::netstat::{get_all_connections, get_dns, Netstat};

mod my_ips;
mod netstat;
mod connections_table;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let ipAddresses = getMyIps().await.unwrap();
    let mut ipAddressesStr = String::new();
    let mut ipAddrsLines = Vec::<Line>::new();
    for ipAddress in ipAddresses {
        if ipAddress.interface.len() == 0 {
            ipAddrsLines.push(
                Line::from(vec![
                    Span::raw("External - "),
                    Span::styled(ipAddress.ip, Style::new().fg(Color::Black).bg(Color::Yellow)),
                ])
            );
            // ipAddressesStr.push_str(format!("External - {}\n", ipAddress.ip).as_str())
        } else {
            ipAddrsLines.push(
                Line::from(vec![
                    Span::raw(format!("{} - ", ipAddress.interface)),
                    Span::styled(ipAddress.ip, Style::new().fg(Color::Black).bg(Color::Yellow)),
                ])
            );
            // ipAddressesStr.push_str(format!("{} - {}\n", ipAddress.interface, ipAddress.ip).as_str())
        }
    }


    /////
    /////
    /////
    /////
    /////




    // let allConnections = get_all_connections();
    // let mut allConnLines = Vec::<Line>::new();
    // for connection in allConnections {
    //
    // }

    //
    // printMyIps().await;
    //
    // list_connections();


    ////
    ////
    ////
    ////

    let terminal = ratatui::init();
    let result = run(terminal, ipAddrsLines);
    ratatui::restore();
    result

    // get_dns("8.8.8.8".to_string());

    // Ok(())
}

fn run(mut terminal: DefaultTerminal, ipAddresses: Vec<Line>) -> io::Result<()> {


    let mut vertical_scroll = 0;
    let mut vertical_scroll_state: ScrollbarState = Default::default();

    let mut connTableState: TableState = TableState::default().with_selected(0);
    let mut connTableScrollbarState: ScrollbarState = Default::default();
    let allConnections = get_all_connections();

    // let mut vertical_scroll2: usize = 0;
    // let mut vertical_scroll_state2: ScrollbarState = Default::default();

    loop {
        terminal.draw(|frame| {
            // frame.render_widget("hello world", frame.area());

            // create a layout that splits the screen into 2 equal columns and the right column
            // into 2 equal rows
            let [left, right] = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).areas(frame.area());
            // let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(3); 4]).areas(right);

            // let connTable = ConnectionsTable::new();
            // connTable.run(frame, right);

            let p = Paragraph::new(ipAddresses.clone())
              // .scroll((vertical_scroll as u16, 0))
              .style(Style::default().fg(Color::Black))
              .block(
                  Block::default()
                    .borders(Borders::ALL)
                    .title(" IP Addresses ")
                    .border_type(BorderType::Rounded)
              );

            // let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            //   .begin_symbol(Some("↑"))
            //   .end_symbol(Some("↓"));

            // let mut scrollbar_state = ScrollbarState::new(3).position(vertical_scroll);

            frame.render_widget(p, left);


            render_table(frame, right, &allConnections, &mut connTableState, &mut connTableScrollbarState);

              frame.render_stateful_widget(
                  Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                frame.area().inner(Margin {
                  vertical: 1,
                  horizontal: 0,
                }),
                // right,
                &mut connTableScrollbarState,
              );




            // let mut vertical_scroll_state = vertical_scroll_state.content_length(ipAddresses.len());
            // let mut vertical_scroll_state2 = vertical_scroll_state2.content_length(allConnections.len());
            //
            // let mut tableState = TableState::default().with_selected(0);

            // let create_block = |title: &'static str| Block::bordered().gray().title(title.bold());
            //
            // let title = Block::new()
            //   .title_alignment(Alignment::Center)
            //   .title("Use h j k l or ◄ ▲ ▼ ► to scroll ".bold());
            // frame.render_widget(title, chunks[0]);

            // let paragraph = Paragraph::new(ipAddresses.clone())
            //   .gray()
            //   .block(create_block("Vertical scrollbar with arrows"))
            //   .scroll((vertical_scroll as u16, 0));
            // frame.render_widget(paragraph, chunks[1]);


            // frame.render_stateful_widget(
            //     scrollbar,
            //     frame.area().inner(Margin {
            //         // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            //         vertical: 1,
            //         horizontal: 0,
            //     }),
            //     &mut scrollbar_state,
            // );


            // frame.render_widget(Block::bordered().title("Left Block"), left);
            // frame.render_widget(Block::bordered().title("Left Block"), right);
            // frame.render_widget(Block::bordered().title("Top Right Block"), top_right);
            // frame.render_widget(Block::bordered().title("Bottom Right Block"), bottom_right);

            /////
            /////
            /////
            /////
            /////
            // let rows =[
            //     Row::new(vec![
            //         Cell::from("Row31 ").black().bold(),
            //         Cell::from(" Row32 ").style(Style::new().yellow()).black().on_light_green(),
            //         Cell::from(Line::from(vec![Span::raw(" Row"), Span::from("33").green()]).black().on_light_red()),
            //     ]).on_dark_gray(),
            //     Row::new(vec![
            //         Cell::from("Row31"),
            //         Cell::from("Row32").style(Style::new().yellow()).black().on_light_green(),
            //         Cell::from(Line::from(vec![Span::raw("Row"), Span::from("33").green()]).centered()),
            //     ]),
            // ];
            //
            // let mut rows  = Vec::new();
            // for conn in allConnections.iter().clone() {
            //     let mut cols = Vec::new();
            //     cols.push(Cell::from(conn.protocol.to_string()));
            //     cols.push(Cell::from(conn.local_addr.to_string()));
            //     cols.push(Cell::from(conn.local_port.to_string()));
            //     cols.push(Cell::from(conn.remote_addr.to_string()));
            //     cols.push(Cell::from(conn.remote_port.to_string()));
            //     cols.push(Cell::from(conn.state.to_string()));
            //     cols.push(Cell::from(conn.pids.get(0).unwrap().to_string()));
            //
            //     rows.push(Row::new(cols));
            // }
            //
            //
            // // Columns widths are constrained in the same way as Layout...
            // let widths = [
            //     Constraint::Length(10),
            //     Constraint::Length(15),
            //     Constraint::Length(12),
            //     Constraint::Length(15),
            //     Constraint::Length(12),
            //     Constraint::Length(20),
            //     Constraint::Length(10),
            // ];
            // let table = Table::new(rows, widths)
            //   // ...and they can be separated by a fixed spacing.
            //   .column_spacing(0)
            //   // You can set the style of the entire Table.
            //   .style(Style::new().white())
            //
            //   // .style(Style::new().blue())
            //   // .style(Style::new().bg(Color::White))
            //   // It has an optional header, which is simply a Row always visible at the top.
            //   .header(
            //       Row::new(vec!["Protocol", "Local Ip", "Local Port", "Remote Ip", "Remote Port", "State", "PIDs"])
            //         .style(Style::new().bold())
            //         // To add space between the header and the rest of the rows, specify the margin
            //         .bottom_margin(1),
            //   )
            //   // It has an optional footer, which is simply a Row always visible at the bottom.
            //   // .footer(Row::new(vec!["Updated on Dec 28"]))
            //   // As any other widget, a Table can be wrapped in a Block.
            //   .block(Block::new().title("Connections"))
            //   // The selected row, column, cell and its content can also be styled.
            //   .row_highlight_style(Style::new().reversed())
            //   .column_highlight_style(Style::new().red())
            //   .cell_highlight_style(Style::new().blue())
            //   .on_white()
            //   // ...and potentially show a symbol in front of the selection.
            //   .highlight_symbol(">>").on_black();


            // frame.render_widget(table, right);


            // frame.render_stateful_widget(
            //     Scrollbar::new(ScrollbarOrientation::VerticalRight)
            //       .begin_symbol(Some("↑"))
            //       .end_symbol(Some("↓")),
            //     left,
            //     &mut vertical_scroll_state,
            // );

            // frame.render_stateful_widget(
            //     Scrollbar::new(ScrollbarOrientation::VerticalRight)
            //       .begin_symbol(Some("↑"))
            //       .end_symbol(Some("↓")),
            //     right,
            //     &mut connTableScrollbarState,
            //     // &mut tableState,
            // );
        })?;
        // if matches!(event::read()?, Event::Key(_)) {
        //     break Ok(());
        // }

        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => {
                        // vertical_scroll = vertical_scroll.saturating_add(1);
                        // vertical_scroll_state =
                        //   vertical_scroll_state.position(vertical_scroll);

                        next_row(&mut connTableState, &allConnections, &mut connTableScrollbarState)
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        // vertical_scroll = vertical_scroll.saturating_sub(1);
                        // vertical_scroll_state =
                        //   vertical_scroll_state.position(vertical_scroll);

                        previous_row(&mut connTableState, &allConnections, &mut connTableScrollbarState)
                    }
                    // KeyCode::Char('t') | KeyCode::Down => {
                    //     vertical_scroll2 = vertical_scroll2.saturating_add(1);
                    //     vertical_scroll_state2 =
                    //       vertical_scroll_state2.position(vertical_scroll2);
                    //     vertical_scroll_state2= vertical_scroll_state2.position(100);
                    //
                    //     TableState::default().with_selected(20);
                    //     TableState::default().select(Some(20));
                    // }
                    // KeyCode::Char('y') | KeyCode::Up => {
                    //     vertical_scroll2 = vertical_scroll2.saturating_sub(1);
                    //     vertical_scroll_state2 =
                    //       vertical_scroll_state2.position(vertical_scroll2);
                    //     vertical_scroll_state2= vertical_scroll_state2.position(0);
                    // }
                    // KeyCode::Char('h') | KeyCode::Left => {
                    //     self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
                    //     self.horizontal_scroll_state = self
                    //       .horizontal_scroll_state
                    //       .position(self.horizontal_scroll);
                    // }
                    // KeyCode::Char('l') | KeyCode::Right => {
                    //     self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
                    //     self.horizontal_scroll_state = self
                    //       .horizontal_scroll_state
                    //       .position(self.horizontal_scroll);
                    // }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }



    }
}

pub fn next_row(state: &mut TableState, items: &Vec<Netstat>, scroll_state: &mut ScrollbarState) {
    let i = match state.selected() {
        Some(i) => {
            if i >= items.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
    scroll_state.position(i * 1);
}

pub fn previous_row(state: &mut TableState, items: &Vec<Netstat>, scroll_state: &mut ScrollbarState) {
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                items.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
    scroll_state.position(i * 1);
}

fn render_table(frame: &mut Frame, area: Rect, allConnections: &Vec<Netstat>, state: &mut TableState, scroll_state: &mut ScrollbarState) {
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

    let header = ["Protocol", "Local Ip", "Local Port", "Remote Ip", "Remote Port", "DNS", "State", "PID", "Proc"]
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
    for conn in allConnections.iter().clone() {
        let mut cols = Vec::new();
        cols.push(Cell::from(conn.protocol.to_string()));
        cols.push(Cell::from(conn.local_addr.to_string()));
        cols.push(Cell::from(conn.local_port.to_string()));
        cols.push(Cell::from(conn.remote_addr.to_string()));
        cols.push(Cell::from(conn.remote_port.to_string()));
        cols.push(Cell::from(conn.dns.to_string()));
        cols.push(Cell::from(conn.state.to_string()).style(get_font_style_by_state(conn.state.to_string())));
        cols.push(Cell::from(conn.pids.get(0).unwrap().to_string()));
        cols.push(Cell::from(conn.proc_name.to_string()));

        rows.push(Row::new(cols).style(get_style_by_state(conn.state.to_string())));
    }

    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(30),
            Constraint::Length(15),
            Constraint::Length(7),
            Constraint::Length(20),
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
    frame.render_stateful_widget(t, area, state);

    // frame.render_stateful_widget(
    //     Scrollbar::default()
    //       .orientation(ScrollbarOrientation::VerticalRight)
    //       .begin_symbol(None)
    //       .end_symbol(None),
    //     area.inner(Margin {
    //         vertical: 0,
    //         horizontal: 1,
    //     }),
    //     scroll_state,
    // );
}

fn get_style_by_state(state: String) -> Style {
    if (state == "ESTABLISHED") {
        Style::new().black().on_green().bold()
    } else if (state == "LISTEN") {
        Style::new().black().on_yellow()
    } else {
        Style::new().black().on_white()
    }
}

fn get_font_style_by_state(state: String) -> Style {
    if (state == "ESTABLISHED") {
        return Style::new().bold()
    }
    Style::new().not_bold()
}


// fn render(frame: &mut Frame) {
//
//
//     // frame.render_widget("hello world", frame.area());
//
//     // create a layout that splits the screen into 2 equal columns and the right column
//     // into 2 equal rows
//     let [left, right] = Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(64)]).areas(frame.area());
//     // let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(3); 4]).areas(right);
//
//     let p = Paragraph::new(ipAddressesStr)
//       .style(Style::default().fg(Color::Black))
//       .block(
//           Block::default()
//             .borders(Borders::ALL)
//             .title(" IP Addresses ")
//             .border_type(BorderType::Rounded)
//       );
//     frame.render_widget(p, left);
//
//
//     // frame.render_widget(Block::bordered().title("Left Block"), left);
//     // frame.render_widget(Block::bordered().title("Left Block"), right);
//     // frame.render_widget(Block::bordered().title("Top Right Block"), top_right);
//     // frame.render_widget(Block::bordered().title("Bottom Right Block"), bottom_right);
// }
