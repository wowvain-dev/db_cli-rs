mod types;
mod functions;

use functions::{
    create_database, run_app
};

use tui::{
    backend::CrosstermBackend,
    Terminal
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use types::App;

use std::
{
    io,
    fs,
    path::Path, io::{Read},
};

fn main() -> Result<(), std::io::Error>{
    // INITIALIZE THE DATABASE
    let mut file = match Path::new("./baza_de_date.io").exists() {
        false =>  fs::File::create("./baza_de_date.io").unwrap(),
        true => fs::OpenOptions::new()
            .append(true)
            .read(true)
            .open("./baza_de_date.io").unwrap()
    };
    
    let mut file_contents:String = String::new();
    file.read_to_string(&mut file_contents).expect("Couldn't read file.");

    let mut db = create_database(file_contents);
    
    // TUI INITIALIZATION
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app: App = App::default();
    app.data_base = db;
    
    run_app(&mut terminal, app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}