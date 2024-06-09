use std::panic;
use std::{
    collections::HashMap,
    io::{self, Stdout},
};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::models::app_model::AppMode;
use crate::models::app_model::AppModelActions;
use crate::models::app_state::{AppStateActions, BaseActions};
use crate::{
    event::{Event, EventHandler},
    models::app_state::AppState,
};

use super::views::editor_view::EditorView;
use super::views::view::{TabState, ViewContext};
use super::{
    command_bar::view::CommandBar,
    views::{view::View, welcome_view::WelcomeVIew},
};

#[derive(Default)]
pub struct UiManager {}

// TOOD
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

        let editor_view = EditorView::new();
        routes_map.insert("/editor".into(), Box::new(editor_view));

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
                    let view = routes_map.get_mut(&current_route).unwrap();
                    terminal
                        .draw(|frame| self.render_ui(frame, view, &command_bar, &mut app_state))?;
                }
                Event::Key(key_event) => {
                    let input_keycode = key_event.code;

                    let is_ctrl_pressed = key_event.modifiers == KeyModifiers::CONTROL;
                    let is_shift_pressed = key_event.modifiers == KeyModifiers::SHIFT;
                    let tab_state = match key_event.code {
                        KeyCode::Tab => TabState::Tab,
                        KeyCode::BackTab => TabState::BackTab,
                        _ => TabState::None,
                    };

                    let app_mode = app_state.app_state_store.get_app_mode();

                    // if we get a signal : and we're in normal, we should change into command mode
                    if let (Char(':'), AppMode::Normal) = (input_keycode, app_mode) {
                        app_state.update(Some(AppStateActions::AppModelActions(
                            AppModelActions::ChangeMode(AppMode::Command),
                        )))
                    }
                    let context = ViewContext::new(is_ctrl_pressed, is_shift_pressed, tab_state);
                    match app_mode {
                        AppMode::Command => {
                            let event = command_bar.handle_event(&key_event, context, &app_state);
                            app_state.update(event);
                        }
                        // otherwise, we pipe every input into the proper view
                        _ => {
                            let current_route = app_state.router_store.get_current_route();
                            let event = routes_map
                                .get_mut(&current_route)
                                .unwrap()
                                .handle_event(&key_event, context, &app_state);
                            app_state.update(event);
                        }
                    }
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {
                    app_state.update(Some(AppStateActions::BaseAppActions(BaseActions::Resized)))
                }
                Event::Paste(_) => {}
            };
        }

        restore_terminal(&mut terminal)
    }

    fn render_ui(
        &self,
        frame: &mut Frame,
        view: &mut Box<dyn View>,
        command_bar: &CommandBar,
        app_state: &mut AppState,
    ) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(100), Constraint::Min(1)])
            .split(frame.size());

        let rect = main_layout[0];
        // technically, we should not be initializing
        // stuff on the first render, or when it was not initialized yez
        // but some models require frames to work, and here's the only place to get them
        // this also places a problem, what to do when we resize the terminal?
        if !view.get_has_been_initialized(&app_state) {
            app_state.update(view.init(frame, rect, &app_state));
        }

        if view.get_has_been_resized(app_state) {
            app_state.update(view.handle_resize(frame, rect, app_state));
        }

        let view = view;
        view.render(frame, rect, app_state);
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
