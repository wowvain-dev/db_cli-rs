use std::{collections::LinkedList, default};


#[derive(Copy, Clone)]
pub enum MenuItem {
    Home,
    MenuItem,
    Clients,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::MenuItem => 1,
            MenuItem::Clients => 2,
        }
    }
}

pub enum InputMode {
    Normal, Editing
}

#[derive(Clone, Copy)]
pub struct size {
    pub height: u16,
    pub width: u16,
}

/// Holds the state of the application
pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub data_base: BazaDate,
    pub window_size: size,
}   

impl App {
    pub fn size(&self) -> &size {
        &self.window_size
    }
}

impl Default for App {
    fn default() -> App {
        App { 
            input: String::new(), 
            input_mode: InputMode::Normal, 
            data_base: BazaDate::default(),
            window_size: size {width: 0, height: 0}
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
    pub adresa: String
}

#[derive(Debug, Clone, Default)]
/// The main database type struct.
pub struct BazaDate {
    pub clienti: LinkedList<Client>,
    pub top: i32
}

impl Client {
    pub fn new() -> Self {
        Client { 
            nr_ordine: 0, 
            nume: String::new(), 
            prenume: String::new(), 
            nr_telefon: String::new(), 
            adresa: String::new() }
    }
}

impl BazaDate {
    pub fn new() -> Self {
        BazaDate { 
            clienti: LinkedList::new(), 
            top: -1 }
    }
}