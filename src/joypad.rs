use minifb::{Key, Window, KeyRepeat};
use crate::memory_map::MemoryMap;
use core::time;
use std::thread;
use crate::joypad::SelectedButtons::{Direction, Action};
use std::ops::BitXor;

#[derive(PartialEq, Clone, Copy)]
pub enum SelectedButtons { Action = 0x10, Direction = 0x20 }

pub struct Joypad {
    selected_buttons: SelectedButtons,
    action_buttons: u8,
    direction_buttons: u8,
}

#[derive(Copy, Clone)]
pub struct InputInterrupt();

impl Joypad {
    pub fn new() -> Self { Self { action_buttons: 0x0F, direction_buttons: 0x0F, selected_buttons: Action } }

    pub fn input_cycle(&mut self, window: &Window) -> Vec<InputInterrupt> {
        if window.is_key_down(Key::Escape) { return std::process::exit(0); }

        let previous_buttons = *self.buttons();

        self.action_buttons = !(if window.is_key_down(Key::Z) { 0x01 } else { 0x00 }
            + if window.is_key_down(Key::C) { 0x02 } else { 0x00 }
            + if window.is_key_down(Key::Backspace) { 0x04 } else { 0x00 }
            + if window.is_key_down(Key::Enter) { 0x08 } else { 0x00 }) & 0x0F;

        self.direction_buttons = !(if window.is_key_down(Key::Right) { 0x01 } else { 0x00 }
            + if window.is_key_down(Key::Left) { 0x02 } else { 0x00 }
            + if window.is_key_down(Key::Up) { 0x04 } else { 0x00 }
            + if window.is_key_down(Key::Down) { 0x08 } else { 0x00 }) & 0x0F;

        let size = self.buttons().bitxor(previous_buttons);
        vec![InputInterrupt(); size as usize]
    }

    fn buttons(&self) -> &u8 {
        if self.selected_buttons == Action { &self.action_buttons } else { &self.direction_buttons }
    }

    pub fn read(&self, address: usize) -> Option<u8> {
        let value = self.selected_buttons as u8 | self.buttons();
        match address {
            0xFF00 => Some(value),
            _ => None
        }
    }

    pub fn write(&mut self, address: usize, value: u8) -> bool {
        match address {
            0xFF00 => {
                self.selected_buttons = match value & 0x30 {
                    0x20 | 0x30 => Direction,
                    0x10 => Action,
                    _ => self.selected_buttons
                }
            }
            _ => return false
        };
        true
    }
}