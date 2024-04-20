use std::panic;
use std::{
    collections::HashMap,
    io::{self, Stdout},
};

use anyhow::Result;
use crossterm::event::KeyModifiers;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::models::app_model::AppMode;
use crate::models::app_model::AppModelActions;
use crate::models::app_state::AppStateActions;
use crate::{
    event::{Event, EventHandler},
    models::app_state::AppState,
};

use super::{
    command_bar::view::CommandBar,
    views::{view::View, welcome_view::WelcomeVIew},
};

#[derive(Default)]
pub struct UiManager {}

// TOOD
// rewrite router to handle routes elm style
// ADD a power / status bar thingy
// add main input handler that handles proper store and passes stuff lower

impl UiManager {
    pub fn new() -> Self {
        UiManager {}
    }

    pub fn run(&mut self) -> Result<()> {
        install_panic_hook();
        let mut app_state = AppState::new();
        let mut routes_map = HashMap::<String, Box<dyn View>>::new();

        let welcome_view = WelcomeVIew::new();
        routes_map.insert("/".into(), Box::new(welcome_view));

        let mut command_bar = CommandBar::new();

        app_state
            .router_store
            .register_routes(Vec::from_iter(routes_map.keys().cloned()));

        let mut terminal = setup_terminal()?;
        let events = EventHandler::new(16);

        while !app_state.app_state_store.get_should_quit() {
            match events.next()? {
                Event::Tick => {
                    let current_route = app_state.router_store.get_current_route();
                    let view = routes_map.get(&current_route).unwrap().to_owned();
                    terminal.draw(|frame| self.render_ui(frame, view, &command_bar, &app_state))?;
                }
                Event::Key(key_event) => match key_event.code {
                    input_keycode => {
                        let is_ctrl_pressed = key_event.modifiers == KeyModifiers::CONTROL;
                        let is_shift_pressed = key_event.modifiers == KeyModifiers::SHIFT;
                        let app_mode = app_state.app_state_store.get_app_mode();

                        // if we get a signal : and we're in normal, we should change into command mode
                        match (input_keycode, app_mode) {
                            (Char(':'), AppMode::Normal) => {
                                app_state.update(Some(AppStateActions::AppModelActions(
                                    AppModelActions::ChangeMode(AppMode::Command),
                                )))
                            }
                            _ => {}
                        }

                        // otherwise, we pipe every input into the proper view
                        match app_mode {
                            AppMode::Command => {
                                let event = command_bar.handle_event(
                                    &key_event,
                                    is_ctrl_pressed,
                                    is_shift_pressed,
                                    &app_state,
                                );
                                app_state.update(event);
                            }
                            _ => {
                                let current_route = app_state.router_store.get_current_route();
                                let event =
                                    routes_map.get_mut(&current_route).unwrap().handle_event(
                                        &key_event,
                                        is_ctrl_pressed,
                                        is_shift_pressed,
                                        &app_state,
                                    );
                                app_state.update(event);
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

    fn render_ui(
        &self,
        frame: &mut Frame,
        view: &Box<dyn View>,
        command_bar: &CommandBar,
        app_state: &AppState,
    ) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(100), Constraint::Min(1)])
            .split(frame.size());

        view.render(frame, main_layout[0], app_state);
        command_bar.render(frame, main_layout[1], app_state);
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
