use crate::update::update;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use std::panic;

use crate::event::{Event, EventHandler};
use crate::model::{Model, ModelActions};

use crate::store::dispatcher::Dispatcher;
use ratatui::prelude::*;

use super::{
    router::Router,
    views::counter_view::{self, CounterView},
};

pub struct UiManager {
    router: Option<Router>,
}

impl UiManager {
    pub fn new() -> Self {
        UiManager { router: None }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = setup_terminal()?;
        install_panic_hook();

        let mut model_dispatcher = Dispatcher::<ModelActions>::new();
        model_dispatcher.register_store(Model::new());

        let counter_view = CounterView::new(&model_dispatcher);
        let starter_view = Box::new(counter_view);

        self.router = Some(Router::new(starter_view));

        let events = EventHandler::new(250);

        while !{
            model_dispatcher
                .get_store::<Model>()
                .unwrap_or(&Model::exit_model())
                .should_quit
        } {
            match events.next()? {
                Event::Tick => {
                    terminal.draw(|frame| self.router.as_ref().unwrap().render_view(frame));
                }
                Event::Key(key_event) => {
                    self.router.as_mut().unwrap().handle_event(&key_event);
                    // update(&mut model_dispatcher, key_event)
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                Event::Paste(_) => {}
            };
        }

        restore_terminal(&mut terminal)
    }
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    reset()?;
    Ok(terminal.show_cursor()?)
}

fn reset() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn install_panic_hook() {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset().unwrap();
        panic_hook(panic);
    }));
}
