use crate::app::AppResult;
use image::Rgb;
use ratatui::crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent,
};
use ratatui_image::{Resize, picker::Picker, protocol::StatefulProtocol};
use std::sync::{
    mpsc,
};
use ratatui::prelude::Rect;
use std::thread;
use std::time::{Duration, Instant};
/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

pub enum AppEvent {
    Redraw(Box<dyn StatefulProtocol>)
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let (tx_w, rx_w) = mpsc::channel::<(Box<dyn StatefulProtocol>, Resize, Rect)>();
        let (tx_m, rx_m) = mpsc::channel();
        let tx_m_render = tx_m.clone();
        thread::spawn(move || loop {
            if let Ok((mut protocol, resize, area)) = rx_w.recv() {
                protocol.resize_encode(&resize, None, area);
                tx_m_render.send(AppEvent::Redraw(protocol)).unwrap();
            }
        });
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
