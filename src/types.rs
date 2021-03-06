use std::collections::LinkedList;

use tui::widgets::Row;

#[derive(Copy, Clone, PartialEq)]
pub enum QueryMode {
    FirstName,
    LastName,
    PhoneNumber,
    Address,
}

impl From<QueryMode> for usize {
    fn from(input: QueryMode) -> usize {
        match input {
            QueryMode::FirstName => 0,
            QueryMode::LastName => 1,
            QueryMode::PhoneNumber => 2,
            QueryMode::Address => 3,
        }
    }
}

/// The possible menu items.
#[derive(Copy, Clone, PartialEq)]
pub enum MenuItem {
    Home,
    Menu,
    Clients,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Menu => 1,
            MenuItem::Clients => 2,
        }
    }
}

/// The possible record options.
#[derive(Copy, Clone, PartialEq)]
pub enum RecordOption {
    None,
    Add,
    Delete,
    Edit
}

impl From<RecordOption> for usize {
    fn from(input: RecordOption) -> usize {
        match input {
            RecordOption::None => 0,
            RecordOption::Add => 1,
            RecordOption::Delete => 2,
            RecordOption::Edit => 3,
        }
    }
}

/// The input mode of the user.
pub enum InputMode {
    Normal,
    Editing,
}

/// The field by which we sort the records.
#[derive(PartialEq)]
pub enum SortMode {
    Id,
    FirstName,
    LastName,
    PhoneNumber,
    Address,
}

/// The order in which the sorting is done.
#[derive(PartialEq)]
pub enum SortOrd {
    Decr,
    Incr,
}

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

/// Holds the state of the application
pub struct App {
    pub query: String,
    pub input_mode: InputMode,
    pub query_mode: QueryMode,
    pub sort_mode: SortMode,
    pub sort_order: SortOrd,
    pub data_base: BazaDate,
    pub window_size: Size,
}

impl App {
    pub fn size(&self) -> &Size {
        &self.window_size
    }
}

impl Default for App {
    fn default() -> App {
        App {
            query: String::new(),
            input_mode: InputMode::Normal,
            query_mode: QueryMode::FirstName,
            sort_mode: SortMode::Id,
            sort_order: SortOrd::Incr,
            data_base: BazaDate::default(),
            window_size: Size {
                width: 0,
                height: 0,
            },
        }
    }
}

/// Input event similar to the `Crossterm` implementation
pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, Clone)]
/// The Client type struct.
pub struct Client {
    pub nr_ordine: i32,
    pub nume: String,
    pub prenume: String,
    pub nr_telefon: String,
    pub adresa: String,
}

#[derive(Debug, Clone, Default)]
/// The main database type struct.
pub struct BazaDate {
    pub clienti: Vec<Client>,
    pub top: i32,
}

impl Client {
    pub fn new() -> Self {
        Client {
            nr_ordine: 0,
            nume: String::new(),
            prenume: String::new(),
            nr_telefon: String::new(),
            adresa: String::new(),
        }
    }
    pub fn get_row(self) -> Vec<String> {
        vec![
            self.nr_ordine.to_string(),
            self.prenume.clone(),
            self.nume.clone(),
            self.nr_telefon.clone(),
            self.adresa.clone(),
        ]
    }
}

impl BazaDate {
    pub fn new() -> Self {
        BazaDate {
            clienti: Vec::new(),
            top: -1,
        }
    }

    /// Deletes record from database.
    pub fn delete_record(&mut self, index: usize) {
        self.clienti.remove(index);
    }
}
