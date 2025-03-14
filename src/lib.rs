#![no_std]

use enemy::{Enemy, EnemyType};
use level_draw::{draw_game_over, draw_lava, draw_platforms, draw_titlescreen, draw_ui};
use num::Integer;
use pc_keyboard::{DecodedKey, KeyCode};
use player::Player;
use pluggable_interrupt_os::vga_buffer::{plot, Color, ColorCode};

use core::
    prelude::rust_2024::derive
;

mod enemy;
mod player;
mod level_draw;

// Stretch Goals
// 1. Two players
// 2. Bigger logo
// 3. Birds laughing at you on the game over screen

const GROUND_BOUNDING_BOXES: [(usize, usize, usize, usize); 5] = [
    (0, 12, 15, 13),
    (70, 12, 80, 13),
    (60, 11, 70, 12),
    (20, 23, 60, 24),
    (35, 7, 50, 8),
];

const SPAWN_POINTS: [(usize, usize); 4] = [
    (7, 9),
    (74, 9),
    (39, 20),
    (42, 4),
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
    enemies: [Enemy; 10],
    spawned_enemies: usize,
    wave: usize,
    ui_drawn: bool,
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
            enemies: [Enemy::default(); 10],
            spawned_enemies: 0,
            wave: 1,
            ui_drawn: false,
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
            self.enemies = Default::default();
            self.spawned_enemies = 0;
            self.wave = 1;
        } else {
            self.ui_drawn = false;
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
                for enemy in self.enemies {
                    enemy.clear();
                }
            }
            State::GameOver => (),
        }
    }

    fn update_all(&mut self) {
        match self.state {
            State::TitleScreen => (),
            State::Playing => {
                if self.spawned_enemies < self.wave && self.spawned_enemies < 10 {
                    if let Some((sx, sy)) = self.get_spawn_point(false) {
                        self.enemies[self.spawned_enemies] = Enemy::default();
                        self.enemies[self.spawned_enemies].dead = false;
                        self.enemies[self.spawned_enemies].x = sx;
                        self.enemies[self.spawned_enemies].y = sy;
                        if self.spawned_enemies >= 3 && self.spawned_enemies < 5 {
                            self.enemies[self.spawned_enemies].etype = EnemyType::Hunter
                        } else if self.spawned_enemies >= 5 {
                            self.enemies[self.spawned_enemies].etype = EnemyType::ShadowLord
                        }
                        self.spawned_enemies += 1;
                    }
                }
                if self.player.dead {
                    if let Some((sx, sy)) = self.get_spawn_point(true) {
                        self.player.x = sx;
                        self.player.y = sy;
                        self.player.dead = false;
                    }
                }

                if self.spawned_enemies == self.wave || self.spawned_enemies == 10 {
                    let mut all_dead = true;
                    for i in 0..self.enemies.len() {
                        if !self.enemies[i].dead {
                            all_dead = false;
                        }
                    }
                    if all_dead {
                        self.wave += 1;
                        self.spawned_enemies = 0;
                    }
                }



                for i in 0..self.enemies.len() {
                    if !self.enemies[i].dead {
                        self.enemies[i].think(self.player.x, self.player.y);
                    }
                }
                let mut sx = 0;
                let mut sy = 0;
                for i in 1..5 {
                    if let Some(sv) = self.player.update_quarter_step(i, GROUND_BOUNDING_BOXES) {
                        (sx, sy) = sv;
                        for j in 0..self.enemies.len() {
                            if !self.enemies[j].dead {
                                if let Some((ex, ey)) = self.enemies[j].update_quarter_step(i, GROUND_BOUNDING_BOXES) {
                                    if self.do_overlap((sx, sy), (sx + 3, sy + 2), (ex, ey), (ex + 3, ey + 2)) && !self.player.dead {
                                        if sy < ey {
                                            if self.enemies[j].die() {
                                                self.enemies[j].dead = true;
                                            }
                                            match self.enemies[j].etype {
                                                EnemyType::Bounder => self.player.score += 250,
                                                EnemyType::Hunter => self.player.score += 500,
                                                EnemyType::ShadowLord => self.player.score += 1000,
                                            };
                                        } else if ey < sy {
                                            if self.player.die() {
                                                self.state_transition(State::GameOver);
                                                return;
                                            }
                                        } else {
                                            self.player.dx *= -1;
                                            self.enemies[j].dx *= -1;
                                        }
                                    }
                                    if i == 4 {
                                        self.enemies[j].x = ex as usize;
                                        self.enemies[j].y = ey as usize;

                                        if self.enemies[j].dy < 30 && !self.enemies[j].on_ground {
                                            self.enemies[j].dy += 5;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self.state_transition(State::GameOver);
                        break;
                    }
                }
                if !self.player.dead {
                    self.player.x = sx as usize;
                    self.player.y = sy as usize;
    
                    if self.player.dy < 30 && !self.player.on_ground {
                        self.player.dy += 5;
                    }
                }
                // for enemy in self.enemies {
                //     if let Some(mut e)  = enemy {
                //         e.think(self.player.x, self.player.y);
                //         e.update(GROUND_BOUNDING_BOXES);
                //         self.player.score = e.score;
                //     }
                // }
                // if self.player.update(GROUND_BOUNDING_BOXES) {
                //     self.state_transition(State::GameOver);
                // };
            }
            State::GameOver => (),
        }
    }

    fn get_spawn_point(&self, is_player: bool) -> Option<(usize, usize)> {
        'outer: for point in SPAWN_POINTS {
            for enemy in self.enemies {
                if !enemy.dead {
                    if self.do_overlap((enemy.x as isize, enemy.y as isize), (enemy.x as isize + 5, enemy.y as isize + 5), (point.0 as isize, point.1 as isize), (point.0 as isize + 5, point.1 as isize + 3)) {
                        continue 'outer
                    }
                }
            }
            if !is_player && !self.player.dead {
                if self.do_overlap((self.player.x as isize, self.player.y as isize), (self.player.x as isize + 5, self.player.y as isize + 5), (point.0 as isize, point.1 as isize), (point.0 as isize + 5, point.1 as isize + 3)) {
                    continue 'outer
                }
            }
            return Some(point)
        }
        None
    }

    fn draw_all(&mut self) {
        match self.state {
            State::TitleScreen => {
                if !self.ui_drawn {
                    draw_titlescreen();
                    self.ui_drawn = true;
                }
            }
            State::Playing => {
                if !self.player.dead {
                    self.player.draw();
                }
                for enemy in self.enemies {
                    if !enemy.dead {
                        enemy.draw()
                    }
                }

                draw_platforms();
                draw_lava();
                draw_ui(self.player.score, self.player.lives, self.wave);
            }
            State::GameOver => {
                if !self.ui_drawn {
                    draw_game_over(self.player.score);
                    self.ui_drawn = true;
                }
            },
        }
    }

    fn do_overlap(&self, l1: (isize, isize), r1: (isize, isize), l2: (isize, isize), r2: (isize, isize)) -> bool {
        if l1.0 > r2.0 || l2.0 > r1.0 {
            return false
        }

        if r1.1 < l2.1 || r2.1 < l1.1 {
            return false
        }

        true
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
                if key == 'z' {
                    self.state_transition(State::Playing);
                }
            },
            State::Playing => {
                if key == 'x' {
                    self.player.flap();
                }
            },
            State::GameOver => {
                if key == 'z' {
                    self.state_transition(State::Playing);
                }
                if key == 'q' {
                    self.state_transition(State::TitleScreen);
                }
            },
        }
    }
}