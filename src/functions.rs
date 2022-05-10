use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, TableState, Tabs},
    Frame, Terminal,
};

use crate::types::{App, BazaDate, Client, InputMode, MenuItem, QueryMode, SortMode, SortOrd, RecordOption};

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
                }
                1 => {
                    temp.prenume = buffer.clone();
                    temp.prenume = temp.prenume.replace("\r", "");
                    buffer.clear();
                    count += 1;
                }
                2 => {
                    temp.nr_telefon = buffer.clone();
                    temp.nr_telefon = temp.nr_telefon.replace("\r", "");
                    buffer.clear();
                    count += 1;
                }
                3 => {
                    temp.adresa = buffer.clone();
                    temp.adresa = temp.adresa.replace("\r", "");
                    temp.nr_ordine = bd.top + 1;
                    bd.clienti.push(temp);
                    bd.top += 1;
                    buffer.clear();
                    temp = Client::new();
                    count = 0;
                }
                _ => {}
            };
        } else {
            buffer.push(val);
        }
    }

    bd
}

/// App execution loop.
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: &mut App) -> io::Result<()> {
    let mut active_menu_item = MenuItem::Home;
    let mut active_record_option = RecordOption::None;
    let mut table_state = TableState::default();
    table_state.select(Some(0));
    loop {
        terminal.draw(|f| {
            app.window_size.height = f.size().height;
            app.window_size.width = f.size().width;
            ui(f, &app, &active_menu_item, &mut table_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => match active_menu_item {
                        MenuItem::Clients => app.input_mode = InputMode::Editing,
                        _ => {}
                    },
                    KeyCode::Char('d') | KeyCode::Backspace => match active_menu_item {
                        MenuItem::Clients => {
                            if let Some(selected) = table_state.selected() {
                                app.data_base.delete_record(selected);
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('h') => {
                        active_menu_item = MenuItem::Home;
                    }
                    KeyCode::Char('m') => {
                        active_menu_item = MenuItem::Menu;
                    }
                    KeyCode::Char('c') => {
                        active_menu_item = MenuItem::Clients;
                    }
                    KeyCode::Down => match active_menu_item {
                        MenuItem::Clients => {
                            if let Some(selected) = table_state.selected() {
                                if selected >= app.data_base.top as usize {
                                    table_state.select(Some(0));
                                } else {
                                    table_state.select(Some(selected + 1));
                                }
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Up => match active_menu_item {
                        MenuItem::Clients => {
                            if let Some(selected) = table_state.selected() {
                                if selected == 0 {
                                    table_state.select(Some(app.data_base.top as usize));
                                } else {
                                    table_state.select(Some(selected - 1));
                                }
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Char('+') => match active_menu_item {
                        MenuItem::Clients => app.sort_order = SortOrd::Incr,
                        _ => {}
                    },
                    KeyCode::Char('-') => match active_menu_item {
                        MenuItem::Clients => app.sort_order = SortOrd::Decr,
                        _ => {}
                    },
                    KeyCode::Tab => match active_menu_item {
                        MenuItem::Clients => {
                            app.sort_mode = match app.sort_mode {
                                SortMode::Id => SortMode::FirstName,
                                SortMode::FirstName => SortMode::LastName,
                                SortMode::LastName => SortMode::PhoneNumber,
                                SortMode::PhoneNumber => SortMode::Address,
                                SortMode::Address => SortMode::Id,
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char(c) => {
                        if active_menu_item == MenuItem::Clients {
                            app.query.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        app.query.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Tab => match app.query_mode {
                        QueryMode::FirstName => app.query_mode = QueryMode::LastName,
                        QueryMode::LastName => app.query_mode = QueryMode::PhoneNumber,
                        QueryMode::PhoneNumber => app.query_mode = QueryMode::Address,
                        QueryMode::Address => app.query_mode = QueryMode::FirstName,
                    },
                    _ => {}
                },
            }
        }
    }
}

/// App user interface
pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    active_menu_item: &MenuItem,
    table_state: &mut TableState,
) {
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
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let main_content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[3]);

    let menu: Vec<Spans> = menu_titles
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
        })
        .collect();
    let secondary_menus: Vec<Spans> = vec!["None", "Add", "Edit", "Delete"]
        .iter()
        .map(|t| {
            Spans::from(
                vec![
                    Span::styled(t.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC))
                ]
            )
        }).collect();

    let tabs = Tabs::new(menu)
        .select((*active_menu_item).into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider(Span::raw("|"));

    let secondary_tabs = Tabs::new(secondary_menus)

        .block(Block::default().title("Record Actions").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider(Span::raw("|"));

    let tabs_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(30),
                Constraint::Length(30)
            ].as_ref()
        )
        .split(chunks[2]);

    f.render_widget(tabs, tabs_layout[0]);
    f.render_widget(secondary_tabs, tabs_layout[1]);

    let copyright = Paragraph::new("DB-CLI 2022 - all rights reserved ©wowvain-dev")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(Span::styled(
                    "Copyright",
                    Style::default().add_modifier(Modifier::ITALIC),
                ))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Plain),
        );

    let title = Paragraph::new("Database Management Shell")
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center);

    let mut shortcuts: Vec<Span> = Vec::new();

    // Rendering the previously created widgets
    f.render_widget(title, chunks[0]);
    f.render_widget(
        Block::default()
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded),
        chunks[2],
    );

    match *active_menu_item {
        MenuItem::Home => {
            shortcuts = vec![Span::styled(
                "Q - Exit App",
                Style::default().fg(Color::Yellow),
            )];

            f.render_widget(
                Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .border_type(BorderType::Rounded),
                main_content_chunks[0],
            );
            f.render_widget(render_home(), main_content_chunks[1]);
        }
        MenuItem::Clients => {
            shortcuts = if let InputMode::Normal = app.input_mode {
                vec![
                    Span::styled("Q - Exit App | ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        "E - Enter Typing Mode | ",
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        "\u{2193}/\u{2191} - Travel Through Records | ",
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        "Tab - Change Sorting Mode | ",
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        "-/+ - Change Sorting Order",
                        Style::default().fg(Color::Yellow),
                    ),
                ]
            } else {
                vec![
                    Span::styled(
                        "Esc - Exit Typing Mode | ",
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        "Tab - Change search query",
                        Style::default().fg(Color::Yellow),
                    ),
                ]
            };
            render_clients(f, app, chunks[3], table_state);
        }
        MenuItem::Menu => {
            shortcuts = vec![Span::styled(
                "Q - Exit App",
                Style::default().fg(Color::Yellow),
            )];
            render_add(f, app, chunks[3]);
        }
    }

    let help = Paragraph::new(vec![Spans::from(shortcuts)])
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Shortcuts"));
    f.render_widget(help, chunks[1]);
    f.render_widget(copyright, chunks[4]);
}

/// Render the contents of the home page tab.
pub fn render_home<'a>() -> Paragraph<'a> {
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
            .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded),
    );
    home
}

/// Render the Add Record option zone.
pub fn render_add<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    rendering_zone: Rect,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(rendering_zone);
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout[0]);
    let upper_layout = Layout::default()
        .margin(3)
        .horizontal_margin(4)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)].as_ref())
        .split(main_layout[0]);
    let lower_layout = Layout::default()
        .margin(3)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_layout[1]);

    let a = Paragraph::new("Test1")
        .block(Block::default().borders(
            Borders::ALL
        ).border_type(BorderType::Rounded));
    let b = Paragraph::new("Test2")
        .block(Block::default().borders(
            Borders::ALL
        ).border_type(BorderType::Rounded));
    let c = Paragraph::new("Test3")
        .block(Block::default().borders(
            Borders::ALL
        ).border_type(BorderType::Rounded));
    let d = Paragraph::new("Test4")
        .block(Block::default().borders(
            Borders::ALL
        ).border_type(BorderType::Rounded));

    let border = Block::default()
        .borders(Borders::LEFT|Borders::RIGHT|Borders::BOTTOM)
        .border_type(BorderType::Rounded);

    f.render_widget(border, layout[0]);
    f.render_widget(a, upper_layout[0]);
    f.render_widget(b, upper_layout[2]);
    f.render_widget(c, lower_layout[0]);
    f.render_widget(d, lower_layout[1]);
}

/// Render the contents of the clients tab.
pub fn render_clients<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    rendering_zone: Rect,
    table_state: &mut TableState,
) {
    let table_cell_constraints = &[
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Min(10),
        Constraint::Min(15),
        Constraint::Min(30),
    ];

    let table_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(rendering_zone);

    let search_layout = Layout::default()
        .horizontal_margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(table_layout[0]);

    let search_title = Spans::from(vec![
        Span::raw("Search by "),
        match app.query_mode {
            QueryMode::FirstName => {
                Span::styled("FIRST NAME", Style::default().add_modifier(Modifier::BOLD))
            }
            QueryMode::LastName => {
                Span::styled("LAST NAME", Style::default().add_modifier(Modifier::BOLD))
            }
            QueryMode::PhoneNumber => Span::styled(
                "PHONE NUMBER",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            QueryMode::Address => {
                Span::styled("ADDRESS", Style::default().add_modifier(Modifier::BOLD))
            }
        },
    ]);

    let search_bar = Paragraph::new(app.query.as_str())
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(match app.input_mode {
                            InputMode::Editing => Modifier::BOLD,
                            InputMode::Normal => Modifier::empty(),
                        }),
                )
                .title(search_title)
                .title_alignment(Alignment::Left),
        );

    let custom_sort_text = (
        match app.sort_order {
            SortOrd::Incr => String::from("increasingly "),
            SortOrd::Decr => String::from("decreasingly "),
        },
        match app.sort_mode {
            SortMode::Id => String::from("ID"),
            SortMode::FirstName => String::from("first name"),
            SortMode::LastName => String::from("last name"),
            SortMode::PhoneNumber => String::from("phone number"),
            SortMode::Address => String::from("address"),
        },
    );
    let sort_bar_text = Spans::from(vec![
        Span::from("Sorted "),
        Span::styled(
            custom_sort_text.0,
            Style::default().add_modifier(Modifier::ITALIC),
        ),
        Span::raw("by "),
        Span::styled(
            custom_sort_text.1,
            Style::default().add_modifier(Modifier::ITALIC),
        ),
    ]);

    let sort_bar = Paragraph::new(sort_bar_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    let rows = query_db(&app.query, app.query_mode, app);

    let mut table = Table::new(rows)
        .header(
            Row::new(vec![
                "ID",
                "First Name",
                "Last Name",
                "Phone Number",
                "Address",
            ])
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED | Modifier::ITALIC),
            ),
        )
        .widths(table_cell_constraints)
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(147, 112, 219))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
        .column_spacing(10);
    table = table.block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(
        Block::default().borders(Borders::LEFT | Borders::RIGHT),
        table_layout[0],
    );
    f.render_widget(search_bar, search_layout[0]);
    f.render_widget(sort_bar, search_layout[1]);
    f.render_stateful_widget(table, table_layout[1], table_state);
}

/// Function for returning only the rows with clients that
/// match the query while also sorting them according to the sort order and mode.
fn query_db(query: &String, query_mode: QueryMode, app: &App) -> Vec<Row<'static>> {
    let mut sorted_clients: Vec<Client> = app.data_base.clienti.clone();

    sorted_clients.sort_by(|a, b| match app.sort_mode {
        SortMode::Id => match app.sort_order {
            SortOrd::Incr => a.nr_ordine.partial_cmp(&b.nr_ordine).unwrap(),
            SortOrd::Decr => b.nr_ordine.partial_cmp(&a.nr_ordine).unwrap(),
        },
        SortMode::FirstName => match app.sort_order {
            SortOrd::Incr => a.prenume.partial_cmp(&b.prenume).unwrap(),
            SortOrd::Decr => b.prenume.partial_cmp(&a.prenume).unwrap(),
        },
        SortMode::LastName => match app.sort_order {
            SortOrd::Incr => a.nume.partial_cmp(&b.nume).unwrap(),
            SortOrd::Decr => b.nume.partial_cmp(&a.nume).unwrap(),
        },
        SortMode::PhoneNumber => match app.sort_order {
            SortOrd::Incr => a.nr_telefon.partial_cmp(&b.nr_telefon).unwrap(),
            SortOrd::Decr => b.nr_telefon.partial_cmp(&a.nr_telefon).unwrap(),
        },
        SortMode::Address => match app.sort_order {
            SortOrd::Incr => a.adresa.partial_cmp(&b.adresa).unwrap(),
            SortOrd::Decr => b.adresa.partial_cmp(&a.adresa).unwrap(),
        },
    });

    if query.is_empty() {
        sorted_clients
            .iter()
            .map(|client| {
                Row::new(vec![
                    client.nr_ordine.to_string(),
                    client.prenume.clone(),
                    client.nume.clone(),
                    client.nr_telefon.clone(),
                    client.adresa.clone(),
                ])
            })
            .collect()
    } else {
        match query_mode {
            QueryMode::FirstName => sorted_clients
                .iter()
                .filter(|q| (**q).prenume.contains(query))
                .map(|client| {
                    Row::new(vec![
                        client.nr_ordine.to_string(),
                        client.prenume.clone(),
                        client.nume.clone(),
                        client.nr_telefon.clone(),
                        client.adresa.clone(),
                    ])
                })
                .collect(),
            QueryMode::LastName => sorted_clients
                .iter()
                .filter(|q| (**q).nume.contains(query))
                .map(|client| {
                    Row::new(vec![
                        client.nr_ordine.to_string(),
                        client.prenume.clone(),
                        client.nume.clone(),
                        client.nr_telefon.clone(),
                        client.adresa.clone(),
                    ])
                })
                .collect(),
            QueryMode::PhoneNumber => sorted_clients
                .iter()
                .filter(|q| (**q).nr_telefon.contains(query))
                .map(|client| {
                    Row::new(vec![
                        client.nr_ordine.to_string(),
                        client.prenume.clone(),
                        client.nume.clone(),
                        client.nr_telefon.clone(),
                        client.adresa.clone(),
                    ])
                })
                .collect(),
            QueryMode::Address => sorted_clients
                .iter()
                .filter(|q| (**q).adresa.contains(query))
                .map(|client| {
                    Row::new(vec![
                        client.nr_ordine.to_string(),
                        client.prenume.clone(),
                        client.nume.clone(),
                        client.nr_telefon.clone(),
                        client.adresa.clone(),
                    ])
                })
                .collect()
        }
    }
}
