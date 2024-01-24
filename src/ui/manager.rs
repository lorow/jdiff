use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::{self, Stdout}, collections::HashMap, sync::{Mutex, Arc}};
use std::panic;

use crate::event::{Event, EventHandler};
use crate::model::{Model, ModelActions};

use crate::store::dispatcher::Dispatcher;
use ratatui::prelude::*;

use super::{views::{counter_view::CounterView, view::View, welcome_view::{self, WelcomeVIew}}, router::{Navigate, Router}};

#[derive(Default)]
pub struct UiManager {}

// TOOD
// ADD a power / status bar thingy 
// extract app state to a proper store
// add main input handler that handles proper store and passes stuff lower

impl UiManager 
{
    pub fn new() -> Self {
        UiManager {}
    }

    pub fn run(&mut self) -> Result<()> {
        install_panic_hook();

        let model_dispatcher = Arc::new(Mutex::new(Dispatcher::<ModelActions>::new()));
        model_dispatcher.lock().unwrap().register_store(Model::new());

        let mut routes_map = HashMap::<String, Box<dyn View>>::new();
        let mut router_store = Router::new();

        let welcome_view: WelcomeVIew = WelcomeVIew::new();
        let counter_view: CounterView = CounterView::new(Arc::clone(&model_dispatcher)); 
        routes_map.insert("/".into(), Box::new(welcome_view));
        routes_map.insert("/counter".into(), Box::new(counter_view));
        router_store.register_routes(Vec::from_iter(routes_map.keys().cloned())); 

        let mut route_dispatcher = Dispatcher::<Navigate>::new();
        route_dispatcher.register_store(router_store);

        let mut terminal = setup_terminal()?;
        let events = EventHandler::new(100);

        while !{
            model_dispatcher
                .lock()
                .unwrap()
                .get_store::<Model>()
                .unwrap_or(&Model::exit_model())
                .should_quit
        } {
            match events.next()? {
               Event::Tick => {
                    let current_route = {route_dispatcher.get_store::<Router>().unwrap().current_route.clone()};
                    let view = routes_map.get(&current_route).unwrap().to_owned();
                    terminal.draw(|frame|self.render_ui(frame, view))?;
                }
                Event::Key(key_event) => {
                    let current_route = {route_dispatcher.get_store::<Router>().unwrap().current_route.clone()};
                    routes_map.get_mut(&current_route).unwrap().handle_event(&key_event, &mut route_dispatcher);
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                Event::Paste(_) => {}
            };
        }

        restore_terminal(&mut terminal)
    }
    
    fn render_ui(&self, frame: &mut Frame, view: &Box<dyn View>){ 
   
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(100),
                Constraint::Min(1),
            ])
            .split(frame.size());
        
        view.render(frame, main_layout[0]);
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
