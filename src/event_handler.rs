use color_eyre::eyre::Ok;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use crossterm::event::MouseEvent;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use color_eyre::Result;

#[derive(Debug, Clone, Copy)]
pub enum GameEvent { // https://ratatui.rs/tutorials/counter-app/_multiple-files/event/#_top
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct GameEventHandler { // https://ratatui.rs/tutorials/counter-app/_multiple-files/event/#_top
    // Event sender 
    #[allow(dead_code)]
    sender: mpsc::Sender<GameEvent>,
    reciever: mpsc::Receiver<GameEvent>,
    // Event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl GameEventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, reciever) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate); 

                    if event::poll(timeout).expect("Unable to poll for event") {
                        match event::read().expect("Unable to read event") {
                            Event::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(GameEvent::Key(e)).expect("Unable to send key event");
                                }
                            },
                            Event::Mouse(e) => sender.send(GameEvent::Mouse(e)).expect("Unable to send mouse event"),
                            Event::Resize(w, h) => sender.send(GameEvent::Resize(w, h)).expect("Unable to send resize event"),
                            _ => {}
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(GameEvent::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            reciever,
            handler,
        }
    }

    pub fn next(&self) -> Result<GameEvent> {
        Ok(self.reciever.recv()?)
    }
}