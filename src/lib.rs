#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct LetterMover {
    letters: [char; BUFFER_WIDTH],
    num_letters: ModNum<usize>,
    next_letter: ModNum<usize>,
    col: ModNum<usize>,
    row: ModNum<usize>,
    dx: ModNum<usize>,
    dy: ModNum<usize>
}

impl LetterMover {
    pub fn new() -> Self {
        LetterMover {
            letters: ['A'; BUFFER_WIDTH],
            num_letters: ModNum::new(1, BUFFER_WIDTH),
            next_letter: ModNum::new(1, BUFFER_WIDTH),
            col: ModNum::new(BUFFER_WIDTH / 2, BUFFER_WIDTH),
            row: ModNum::new(BUFFER_HEIGHT / 2, BUFFER_HEIGHT),
            dx: ModNum::new(0, BUFFER_WIDTH),
            dy: ModNum::new(0, BUFFER_HEIGHT)
        }
    }

    fn letter_columns(&self) -> impl Iterator<Item=usize> {
        ModNumIterator::new(self.col)
            .take(self.num_letters.a())
            .map(|m| m.a())
    }

    pub fn tick(&mut self) {
        self.clear_current();
        self.update_location();
        self.draw_current();
    }

    fn clear_current(&self) {
        for x in self.letter_columns() {
            plot(' ', x, self.row.a(), ColorCode::new(Color::Black, Color::Black));
        }
    }

    fn update_location(&mut self) {
        self.col += self.dx;
        self.row += self.dy;
    }

    fn draw_current(&self) {
        for (i, x) in self.letter_columns().enumerate() {
            plot(self.letters[i], x, self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.dx -= 1;
            }
            KeyCode::ArrowRight => {
                self.dx += 1;
            }
            KeyCode::ArrowUp => {
                self.dy -= 1;
            }
            KeyCode::ArrowDown => {
                self.dy += 1;
            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            self.letters[self.next_letter.a()] = key;
            self.next_letter += 1;
            self.num_letters = self.num_letters.saturating_add(&ModNum::new(1, self.num_letters.m()));
        }
    }
}