use crate::app::AppResult;
use ratatui::crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent,
};
use std::time::Duration;
use std::{sync::mpsc, thread};
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || loop {
                if event::poll(Duration::from_millis(250)).expect("failed to poll new events") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => {
                            if e.kind == KeyEventKind::Press {
                                sender.send(Event::Key(e))
                            } else {
                                Ok(())
                            }
                        }
                        CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                        CrosstermEvent::FocusGained => Ok(()),
                        CrosstermEvent::FocusLost => Ok(()),
                        CrosstermEvent::Paste(_) => unimplemented!(),
                    }
                    .expect("failed to send terminal event")
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
