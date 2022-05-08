use std::{io};
use crossterm::event::{self, Event, KeyCode};
use tui::{
    Terminal,
    backend::{Backend}, 
    Frame, 
    layout::{Layout, Direction, Constraint, Margin, Alignment, Rect}, 
    style::{Style, Modifier, Color},
    text::{Span, Text, Spans}, widgets::{Paragraph, Block, Borders, BorderType, Tabs, Table, Row}
};

use crate::types::{BazaDate, Client, App, InputMode, MenuItem};

/// Clear the terminal screen and moves the cursor at the top.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Creating the database using the `String` provided.
pub fn create_database(file_contents: String) -> BazaDate {



    let mut bd: BazaDate = BazaDate::new();
    let mut count: i32 = 0;

    let mut temp: Client = Client::new();

    let mut buffer = String::new();
    for (_, val) in file_contents.as_str().chars().enumerate() {
        if val == '\n' {
            match count {
                0 => {
                    temp.nume = buffer.clone();
                    temp.nume = temp.nume.replace("\r", "");
                    buffer.clear();
                    count += 1;
                }, 
                1 => {
                    temp.prenume = buffer.clone();
                    temp.prenume = temp.prenume.replace("\r", "");
                    buffer.clear();
                    count += 1;
                },
                2 => {
                    temp.nr_telefon = buffer.clone();
                    temp.nr_telefon = temp.nr_telefon.replace("\r", "");
                    buffer.clear();
                    count += 1;
                },
                3 => {
                    temp.adresa = buffer.clone();
                    temp.adresa = temp.adresa.replace("\r", "");
                    temp.nr_ordine = bd.top + 1;
                    bd.clienti.push_back(temp);
                    bd.top += 1;
                    buffer.clear();
                    temp = Client::new();
                    count = 0;
                },
                _ => {} 
            };
        } else {
            buffer.push(val);
        }
    }
    temp.adresa = buffer.clone();
    temp.nr_ordine = bd.top + 1;
    bd.clienti.push_back(temp);
    buffer.clear();
    temp = Client::new();

    bd
}

/// App execution loop.
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut active_menu_item = MenuItem::Home;
    loop {
        
        terminal.draw(|f| {
            app.window_size.height = f.size().height;
            app.window_size.width = f.size().width;
            ui(f, &app, &active_menu_item);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    },
                    KeyCode::Char('q') => {
                        return Ok(());
                    },
                    KeyCode::Char('h') => {
                        active_menu_item = MenuItem::Home;
                    },
                    KeyCode::Char('m') => {
                        active_menu_item = MenuItem::MenuItem;
                    },
                    KeyCode::Char('c') => {
                        active_menu_item = MenuItem::Clients;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    },
                    KeyCode::Backspace =>{
                        app.input.pop();
                    },
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    },
                    _ => {}
                }
            }
        }
    }
}

/// App user interface
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, active_menu_item: &MenuItem) {
    let mut rows:Vec<Row> = Vec::new();
    
    let table_cell_constraints = &[
                    Constraint::Length(3), 
                    Constraint::Length(20), 
                    Constraint::Length(20), 
                    Constraint::Length(15), 
                    Constraint::Length(app.size().width-58)];

    for (index, value) in app.data_base.clienti.iter().enumerate() {
        rows.push(Row::new(vec![value.nr_ordine.to_string(), 
                                      value.prenume.clone(), 
                                      value.nume.clone(), 
                                      value.nr_telefon.clone(), 
                                      value.adresa.clone()]));
    }

    //println!("{:?}", rows);

    let table = Table::new(rows)
        .header(Row::new(vec!["ID", "First Name", "Last Name", "Phone Number", "Address"])
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD|Modifier::UNDERLINED|Modifier::ITALIC)))
        .widths(table_cell_constraints)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .column_spacing(10);

    let menu_titles = vec!["Home", "Main Menu", "Clients"];
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3)
            ].as_ref()
        ).split(f.size());

    let secondary_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(65)
            ].as_ref()
        ).split(chunks[3]);

    let main_content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60)
            ].as_ref()
        ).split(chunks[3]);

    let menu: Vec<Spans>  = menu_titles 
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first, 
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                    ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        }).collect();

    let tabs = Tabs::new(menu)
        .select((*active_menu_item).into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider(Span::raw("|"));

    f.render_widget(tabs, chunks[2]);

    let copyright = Paragraph::new("DB-CLI 2022 - all rights reserved ©wowvain-dev")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(
                    Span::styled("Copyright", Style::default().add_modifier(Modifier::ITALIC))
                ).title_alignment(Alignment::Center)
                .border_type(BorderType::Plain),
        );
    

    let mut title = Paragraph::new("Database Management Shell")
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center);


    let help = Paragraph::new("Q - quit app")
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL).title("Shortcuts"));
    

    // Rendering the previously created widgets
    f.render_widget(title, chunks[0]);
    f.render_widget(help, chunks[1]);
    f.render_widget(Block::default()
        .borders(Borders::LEFT|Borders::TOP|Borders::RIGHT)
        .border_style(Style::default().fg(Color::White)).border_type(BorderType::Rounded), chunks[2]);
    
    match *active_menu_item {
        MenuItem::Home => {
            f.render_widget(Block::default().borders(Borders::LEFT|Borders::RIGHT).border_type(BorderType::Rounded), main_content_chunks[0]);
            f.render_widget(render_home(app), main_content_chunks[1]);
        }, 
        MenuItem::Clients => {
            f.render_widget(table.block(Block::default()
                .borders(Borders::LEFT|Borders::BOTTOM|Borders::RIGHT)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)), chunks[3])
        },
        _ => f.render_widget(Block::default()
                .borders(Borders::LEFT|Borders::BOTTOM|Borders::RIGHT)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded), chunks[3])
    }

    f.render_widget(copyright, chunks[4]);
}

/// Render the contents of the home page tab. 
pub fn render_home(app: &App) -> Paragraph {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "DB-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::LEFT|Borders::BOTTOM|Borders::RIGHT)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
    );
    home
}

/// Render the contents of the main menu tab.
pub fn render_menu<B: Backend>(f:&mut Frame<B>, app: &App) {
}

/// Render the contents of the clients tab.
pub fn render_clients<B: Backend>(f:&mut Frame<B>, app:&App, rendering_zone: Rect) {
    
}