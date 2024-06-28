use std::{
    sync::mpsc::{self, RecvError},
    thread,
    time::{Duration, Instant},
};

use crossterm::event::Event as CrosstermEvent;
use crossterm::event::{self, KeyEvent, KeyEventKind, MouseEvent};

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Paste(String),
}

#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();

        let handler = {
            let sender = sender.clone();

            thread::spawn(move || {
                let mut last_tick = Instant::now();

                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).expect("Timeout polling event") {
                        match event::read().expect("Unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(())
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            CrosstermEvent::Paste(s) => sender.send(Event::Paste(s)),
                            _ => Ok(()),
                        }
                        .expect("Failed to send event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send TICK event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}
