use std::io;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Masked, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use crate::my_ips::{getMyIps, IP_Address};
use crate::netstat::list_connections;

mod my_ips;
mod netstat;

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
}

fn run(mut terminal: DefaultTerminal, ipAddresses: Vec<Line>) -> io::Result<()> {

    let mut vertical_scroll = 0;
    let mut vertical_scroll_state: ScrollbarState = Default::default();

    loop {
        terminal.draw(|frame| {
            // frame.render_widget("hello world", frame.area());

            // create a layout that splits the screen into 2 equal columns and the right column
            // into 2 equal rows
            let [left, right] = Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(64)]).areas(frame.area());
            // let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(3); 4]).areas(right);

            let p = Paragraph::new(ipAddresses.clone())
              .scroll((vertical_scroll as u16, 0))
              .style(Style::default().fg(Color::Black))
              .block(
                  Block::default()
                    .borders(Borders::ALL)
                    .title(" IP Addresses ")
                    .border_type(BorderType::Rounded)
              );

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
              .begin_symbol(Some("↑"))
              .end_symbol(Some("↓"));

            let mut scrollbar_state = ScrollbarState::new(3).position(vertical_scroll);

            frame.render_widget(p, left);



            let mut vertical_scroll_state = vertical_scroll_state.content_length(ipAddresses.len());

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
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                  .begin_symbol(Some("↑"))
                  .end_symbol(Some("↓")),
                left,
                &mut vertical_scroll_state,
            );

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
        })?;
        // if matches!(event::read()?, Event::Key(_)) {
        //     break Ok(());
        // }

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => {
                        vertical_scroll = vertical_scroll.saturating_add(1);
                        vertical_scroll_state =
                          vertical_scroll_state.position(vertical_scroll);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        vertical_scroll = vertical_scroll.saturating_sub(1);
                        vertical_scroll_state =
                          vertical_scroll_state.position(vertical_scroll);
                    }
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
