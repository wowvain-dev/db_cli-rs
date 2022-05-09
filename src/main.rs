mod functions;
mod types;

use functions::{run_app, create_database};

use tui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use types::App;

use std::{
    fs::{self, File, OpenOptions},
    io,
    io::{Read, Write},
    path::Path,
};

fn main() -> Result<(), std::io::Error> {

    // INITIALIZE THE DATABASE
    if !Path::new("./baza_de_date.io").exists() {
        let file = File::create("./baza_de_date.io").expect("Can't Create File");
    }

    let mut file = File::open("./baza_de_date.io").expect("Can't Open File");
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Couldn't read file.");

    let mut db = create_database(file_contents);

    // TUI INITIALIZATION
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app: App = App::default();
    app.data_base = db;

    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    let mut file = File::create("./baza_de_date.io").expect("unable to open file");

    for (_, client) in app.data_base.clienti.iter().enumerate() {
        let mut data = String::new();
        data.push_str(client.prenume.as_str());
        data.push('\n');
        data.push_str(client.nume.as_str());
        data.push('\n');
        data.push_str(client.nr_telefon.as_str());
        data.push('\n');
        data.push_str(client.adresa.as_str());
        data.push('\n');
        file.write_all(data.as_bytes()).expect("Unable to write");
    }

    Ok(())
}
