#![no_std]

use num::Integer;
use pc_keyboard::{DecodedKey, KeyCode};
use player::Player;
use pluggable_interrupt_os::vga_buffer::{plot, Color, ColorCode};

use core::
    prelude::rust_2024::derive
;

mod enemy;
mod player;

// Stretch Goals
// 1. Two players
// 2. Bigger logo
// 3. Birds laughing at you on the game over screen

const GROUND_BOUNDING_BOXES: [(usize, usize, usize, usize); 5] = [
    (0, 12, 15, 13),
    (70, 12, 80, 13),
    (60, 11, 70, 12),
    (20, 23, 60, 24),
    (0, 0, 0, 0),
];

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    TitleScreen,
    Playing,
    GameOver,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Joust {
    player: Player,
    state: State,
}

pub fn safe_add<const LIMIT: usize>(a: usize, b: usize) -> usize {
    (a + b).mod_floor(&LIMIT)
}

pub fn add1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, 1)
}

pub fn sub1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, LIMIT - 1)
}

impl Default for Joust {
    fn default() -> Self {
        Self {
            player: Player::default(),
            state: State::TitleScreen,
        }
    }
}

impl Joust {
    pub fn tick(&mut self) {
        self.clear();
        self.update_all();
        self.draw_all();
    }

    fn state_transition(&mut self, new_state: State) {
        if new_state == State::Playing {
            self.player = Player::default();
        }
        self.state = new_state;
        for x in 0..80 {
            for y in 0..25 {
                plot(' ', x, y, ColorCode::new(Color::Black, Color::Black));
            }
        }
    }

    fn clear(&self) {
        match self.state {
            State::TitleScreen => (),
            State::Playing => {
                self.player.clear();
            }
            State::GameOver => (),
        }
    }

    fn update_all(&mut self) {
        match self.state {
            State::TitleScreen => (),
            State::Playing => {
                if self.player.update(GROUND_BOUNDING_BOXES) {
                    self.state_transition(State::GameOver);
                };
            }
            State::GameOver => (),
        }
    }

    fn draw_all(&self) {
        match self.state {
            State::TitleScreen => {
                self.draw_titlescreen();
            }
            State::Playing => {
                self.player.draw();

                self.draw_platforms();
                self.draw_lava();
                self.draw_ui(self.player.score, self.player.lives);
            }
            State::GameOver => {
                self.draw_game_over();
            },
        }
    }

    fn draw_platforms(&self) {
        for x in 0..15 {
            plot(' ', x, 12, ColorCode::new(Color::Brown, Color::Brown));
        }
        for x in 70..80 {
            plot(' ', x, 12, ColorCode::new(Color::Brown, Color::Brown));
        }
        for x in 60..70 {
            plot(' ', x, 11, ColorCode::new(Color::Brown, Color::Brown));
        }
        for x in 20..60 {
            plot(' ', x, 23, ColorCode::new(Color::Brown, Color::Brown));
        }
    }

    fn draw_lava(&self) {
        for x in 0..21 {
            plot(
                178u8 as char,
                x,
                24,
                ColorCode::new(Color::Red, Color::Yellow),
            );
        }
        for x in 59..80 {
            plot(
                178u8 as char,
                x,
                24,
                ColorCode::new(Color::Red, Color::Yellow),
            );
        }
    }

    fn draw_ui(&self, score: usize, lives: usize) {
        for x in 21..59 {
            plot(' ', x, 24, ColorCode::new(Color::Brown, Color::Brown))
        }
        self.draw_score(23, 24, score);
        for x in 0..lives {
            plot(
                1u8 as char,
                x + 35,
                24,
                ColorCode::new(Color::Yellow, Color::Brown),
            );
        }
    }

    fn draw_score(&self, sx: usize, sy: usize, score: usize) {
        for x in 0usize..9 {
            let d = 10_usize.pow(9 - x as u32);
            let v = 10_usize.pow(8 - x as u32);
            let num = score.mod_floor(&d) / v + 48;
            plot(
                num as u8 as char,
                x + sx,
                sy,
                ColorCode::new(Color::Yellow, Color::Brown),
            );
        }
    }

    fn draw_titlescreen(&self) {
        const TITLE_X: usize = 35;
        const TITLE_Y: usize = 10;

        // J
        plot(
            205u8 as char,
            TITLE_X,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            187u8 as char,
            TITLE_X + 1,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 1,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            200u8 as char,
            TITLE_X,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            188u8 as char,
            TITLE_X + 1,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );

        // O
        plot(
            201u8 as char,
            TITLE_X + 2,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            187u8 as char,
            TITLE_X + 3,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 2,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 3,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            200u8 as char,
            TITLE_X + 2,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            188u8 as char,
            TITLE_X + 3,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );

        // U
        plot(
            186u8 as char,
            TITLE_X + 4,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 5,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 4,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 5,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            200u8 as char,
            TITLE_X + 4,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            188u8 as char,
            TITLE_X + 5,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );

        // S
        plot(
            201u8 as char,
            TITLE_X + 6,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            205u8 as char,
            TITLE_X + 7,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            200u8 as char,
            TITLE_X + 6,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            187u8 as char,
            TITLE_X + 7,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            205u8 as char,
            TITLE_X + 6,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            188u8 as char,
            TITLE_X + 7,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );

        // T
        plot(
            201u8 as char,
            TITLE_X + 8,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            205u8 as char,
            TITLE_X + 9,
            TITLE_Y,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 8,
            TITLE_Y + 1,
            ColorCode::new(Color::Yellow, Color::Red),
        );
        plot(
            186u8 as char,
            TITLE_X + 8,
            TITLE_Y + 2,
            ColorCode::new(Color::Yellow, Color::Red),
        );


        //prompt

        const PROMPT_X: usize = 32;
        const PROMPT_Y: usize = 16;

        plot(
            'P',
            PROMPT_X,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            'r',
            PROMPT_X+1,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            'e',
            PROMPT_X+2,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            's',
            PROMPT_X+3,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            's',
            PROMPT_X+4,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            'X',
            PROMPT_X+6,
            PROMPT_Y,
            ColorCode::new(Color::White, Color::LightBlue),
        );
        plot(
            't',
            PROMPT_X+8,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            'o',
            PROMPT_X+9,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            's',
            PROMPT_X+11,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            't',
            PROMPT_X+12,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            'a',
            PROMPT_X+13,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            'r',
            PROMPT_X+14,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );
        plot(
            't',
            PROMPT_X+15,
            PROMPT_Y,
            ColorCode::new(Color::LightBlue, Color::Black),
        );

    }

    fn draw_game_over(&self) {
        const GAME_OVER_X: usize = 35;
        const GAME_OVER_Y: usize = 10;
        // Game Over
        plot(
            'G',
            GAME_OVER_X,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'A',
            GAME_OVER_X+1,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'M',
            GAME_OVER_X+2,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'E',
            GAME_OVER_X+3,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'O',
            GAME_OVER_X+5,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'V',
            GAME_OVER_X+6,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'E',
            GAME_OVER_X+7,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            'R',
            GAME_OVER_X+8,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        plot(
            '!',
            GAME_OVER_X+9,
            GAME_OVER_Y,
            ColorCode::new(Color::Red, Color::Black),
        );
        
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c),
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        if self.state == State::Playing {
            match key {
                KeyCode::ArrowLeft => {
                    self.player.accel_left();
                }
                KeyCode::ArrowRight => {
                    self.player.accel_right();
                }
                _ => {}
            }
        }
    }

    fn handle_unicode(&mut self, key: char) {
        match self.state {
            State::TitleScreen => {
                if key == 'x' {
                    self.state_transition(State::Playing);
                }
            },
            State::Playing => {
                if key == 'x' {
                    self.player.flap();
                }
            },
            State::GameOver => {
                if key == 'x' {
                    self.state_transition(State::TitleScreen);
                }
            },
        }
    }
}