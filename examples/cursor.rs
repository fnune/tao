// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use simple_logger::SimpleLogger;
use tao::{
  event::{ElementState, Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{CursorIcon, WindowBuilder},
};

fn main() {
  SimpleLogger::new().init().unwrap();
  let event_loop = EventLoop::new();

  let window = WindowBuilder::new().build(&event_loop).unwrap();
  window.set_title("A fantastic window!");

  let mut cursor_idx = 0;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event:
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                state: ElementState::Pressed,
                ..
              },
            ..
          },
        ..
      } => {
        println!("Setting cursor to \"{:?}\"", CURSORS[cursor_idx]);
        window.set_cursor_icon(CURSORS[cursor_idx]);
        if cursor_idx < CURSORS.len() - 1 {
          cursor_idx += 1;
        } else {
          cursor_idx = 0;
        }
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}

const CURSORS: &[CursorIcon] = &[
  CursorIcon::Default,
  CursorIcon::Crosshair,
  CursorIcon::Hand,
  CursorIcon::Arrow,
  CursorIcon::Move,
  CursorIcon::Text,
  CursorIcon::Wait,
  CursorIcon::Help,
  CursorIcon::Progress,
  CursorIcon::NotAllowed,
  CursorIcon::ContextMenu,
  CursorIcon::Cell,
  CursorIcon::VerticalText,
  CursorIcon::Alias,
  CursorIcon::Copy,
  CursorIcon::NoDrop,
  CursorIcon::Grab,
  CursorIcon::Grabbing,
  CursorIcon::AllScroll,
  CursorIcon::ZoomIn,
  CursorIcon::ZoomOut,
  CursorIcon::EResize,
  CursorIcon::NResize,
  CursorIcon::NeResize,
  CursorIcon::NwResize,
  CursorIcon::SResize,
  CursorIcon::SeResize,
  CursorIcon::SwResize,
  CursorIcon::WResize,
  CursorIcon::EwResize,
  CursorIcon::NsResize,
  CursorIcon::NeswResize,
  CursorIcon::NwseResize,
  CursorIcon::ColResize,
  CursorIcon::RowResize,
];
