use anyhow::Result;
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture,
        KeyCode::{self, Char},
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::panic;
use std::{
    collections::HashMap,
    io::{self, Stdout},
    sync::{Arc, Mutex},
};

use crate::models::counter::{CounterModel, CounterModelActions};
use crate::{
    event::{Event, EventHandler},
    models::app_state::{AppMode, AppState, AppStateActions},
};

use crate::store::dispatcher::Dispatcher;
use ratatui::prelude::*;

use super::{
    command_bar::view::CommandBar,
    router::{Navigate, Router},
    views::{counter_view::CounterView, view::View, welcome_view::WelcomeVIew},
};

#[derive(Default)]
pub struct UiManager {}

// TOOD
// ADD a power / status bar thingy
// add main input handler that handles proper store and passes stuff lower

impl UiManager {
    pub fn new() -> Self {
        UiManager {}
    }

    pub fn run(&mut self) -> Result<()> {
        install_panic_hook();

        let app_state_dispatcher = Arc::new(Mutex::new(Dispatcher::<AppStateActions>::new()));
        app_state_dispatcher
            .lock()
            .unwrap()
            .register_store(AppState::new());

        let counter_model_dispatcher =
            Arc::new(Mutex::new(Dispatcher::<CounterModelActions>::new()));
        counter_model_dispatcher
            .lock()
            .unwrap()
            .register_store(CounterModel::new());

        let mut routes_map = HashMap::<String, Box<dyn View>>::new();
        let mut router_store = Router::new();

        let mut command_bar = Box::new(CommandBar::new(Arc::clone(&app_state_dispatcher)));
        let welcome_view = WelcomeVIew::new(Arc::clone(&app_state_dispatcher));
        let counter_view = CounterView::new(
            Arc::clone(&app_state_dispatcher),
            Arc::clone(&counter_model_dispatcher),
        );

        routes_map.insert("/".into(), Box::new(welcome_view));
        routes_map.insert("/counter".into(), Box::new(counter_view));
        router_store.register_routes(Vec::from_iter(routes_map.keys().cloned()));

        let mut route_dispatcher = Dispatcher::<Navigate>::new();
        route_dispatcher.register_store(router_store);

        let mut terminal = setup_terminal()?;
        let events = EventHandler::new(100);

        while !{
            app_state_dispatcher
                .lock()
                .unwrap()
                .get_store::<AppState>()
                .unwrap_or(&AppState::exit_model())
                .should_quit
        } {
            match events.next()? {
                Event::Tick => {
                    let current_route = {
                        route_dispatcher
                            .get_store::<Router>()
                            .unwrap()
                            .current_route
                            .clone()
                    };
                    let view = routes_map.get(&current_route).unwrap().to_owned();
                    terminal.draw(|frame| self.render_ui(frame, view, &command_bar))?;
                }
                Event::Key(key_event) => match key_event.code {
                    input_keycode => {
                        let app_mode = {
                            app_state_dispatcher
                                .lock()
                                .unwrap()
                                .get_store::<AppState>()
                                .unwrap()
                                .mode
                        };

                        // if we get a signal : and we're in normal, we should change into command mode
                        match (input_keycode, app_mode) {
                            (Char(':'), AppMode::Normal) => {
                                app_state_dispatcher
                                    .lock()
                                    .unwrap()
                                    .dispatch(AppStateActions::ChangeMode(AppMode::Command));
                            }
                            _ => {}
                        }
                        // otherwise, we pipe every input into the proper view
                        match app_mode {
                            AppMode::Command => {
                                command_bar.handle_event(&key_event);
                            }
                            _ => {
                                let current_route = {
                                    route_dispatcher
                                        .get_store::<Router>()
                                        .unwrap()
                                        .current_route
                                        .clone()
                                };
                                routes_map
                                    .get_mut(&current_route)
                                    .unwrap()
                                    .handle_event(&key_event, &mut route_dispatcher);
                            }
                        }
                    }
                },
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                Event::Paste(_) => {}
            };
        }

        restore_terminal(&mut terminal)
    }

    fn render_ui(&self, frame: &mut Frame, view: &Box<dyn View>, command_bar: &Box<CommandBar>) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(100), Constraint::Min(1)])
            .split(frame.size());

        view.render(frame, main_layout[0]);
        command_bar.render(frame, main_layout[1]);
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
