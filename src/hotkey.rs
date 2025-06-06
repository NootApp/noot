use iced::futures::{SinkExt, Stream, StreamExt};
use iced::{stream};
use iced::futures::channel::mpsc;
use crate::runtime::messaging::Message;

use rdev::{listen, EventType, Key};

pub fn start() -> impl Stream<Item = Message> {
    stream::channel(1, async |mut output| {
        info!("Spawning keybind thread");
        let (mut sender, mut receiver) = mpsc::channel(1);

        std::thread::spawn(move || {
           let _= listen(move |event| {
               let _ = sender.try_send(event);
           });
        });


        let mut alt_pressed = false;
        let mut altgr_pressed = false;
        let mut lshift_pressed = false;
        let mut rshift_pressed = false;
        //let mut lmeta_pressed= false;
        //let mut rmeta_pressed = false;
        loop {
            let event = receiver.select_next_some().await;
            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        Key::Alt => alt_pressed = true,
                        Key::AltGr => altgr_pressed = true,
                        Key::ShiftLeft => lshift_pressed = true,
                        Key::ShiftRight => rshift_pressed = true,
                        //Key::MetaLeft => lmeta_pressed = true,
                        //Key::MetaRight => rmeta_pressed = true,
                        Key::KeyN => {
                            if altgr_pressed || alt_pressed {
                                info!("Hotkey OpenLastEditor");
                                let _ = output.send(Message::hotkey(Keybind::OpenLastEditor)).await;
                            }
                        }
                        Key::KeyD => {
                            if (altgr_pressed || alt_pressed) && (lshift_pressed || rshift_pressed){
                                info!("Hotkey OpenLastEditor");
                                let _ = output.send(Message::hotkey(Keybind::ToggleDyslexia)).await;
                            }
                        }
                        _ => ()
                    }
                },
                EventType::KeyRelease(key) =>  {
                    match key {
                        Key::Alt => alt_pressed = false,
                        Key::AltGr => altgr_pressed = false,
                        Key::ShiftLeft => lshift_pressed = false,
                        Key::ShiftRight => rshift_pressed = false,
                        //Key::MetaLeft => lmeta_pressed = false,
                        //Key::MetaRight => rmeta_pressed = false,
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    })
}


#[derive(Clone, Debug)]
pub enum Keybind {
    OpenLastEditor,
    ToggleDyslexia
}
