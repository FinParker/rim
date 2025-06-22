use crate::editor::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::convert::TryFrom;
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

#[derive(Debug)]
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Help,
    Quit,
    OtherKeyCommand(String),
    OtherEvent(String),
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            // 处理KeyPress
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char('h'), KeyModifiers::CONTROL) => Ok(Self::Help),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                _ => {
                    if modifiers == KeyModifiers::empty() {
                        Err(format!("Press <{code}>"))
                    } else {
                        Err(format!("Press <{modifiers} {code}>"))
                    }
                }
            },
            // 处理其他的KeyEvent, 包括KeyRelease和KeyRepeat
            Event::Key(key_event) if key_event.kind != KeyEventKind::Press => {
                Ok(Self::OtherKeyCommand(format!(
                    "KeyEvent: code={},modifiers={},kind={:?},state={:?}",
                    key_event.code, key_event.modifiers, key_event.kind, key_event.state
                )))
            }
            // 处理ResizeEvent
            Event::Resize(width, height) =>
            {
                #[allow(clippy::as_conversions)]
                Ok(Self::Resize(Size {
                    height: height as usize,
                    width: width as usize,
                }))
            }
            _ => Ok(Self::OtherEvent(format!("{event:?}"))),
        }
    }
}
