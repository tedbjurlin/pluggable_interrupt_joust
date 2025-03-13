use num::Integer;
use pluggable_interrupt_os::vga_buffer::{plot, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH};

const MOVE_WIDTH: isize = BUFFER_WIDTH as isize - 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub dx: isize,
    pub dy: isize,
    pub score: usize,
    pub lives: usize,
    pub on_ground: bool,
    pub dead: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 39,
            y: 20,
            score: 0,
            lives: 6,
            dx: 0,
            dy: 0,
            on_ground: true,
            dead: false,
        }
    }
}

impl Player {
    pub fn clear(&self) {
        for x in 0..4 {
            for y in 0..3 {
                plot(
                    ' ',
                    x + self.x,
                    y + self.y,
                    ColorCode::new(Color::Black, Color::Black),
                )
            }
        }
    }
    fn is_on_ground(
        &self,
        sx: usize,
        sy: usize,
        ground_bounding_boxes: [(usize, usize, usize, usize); 5],
    ) -> bool {
        for (x1, y1, x2, y2) in ground_bounding_boxes {
            if sy + 3 >= y1 && sy + 3 < y2 && sx >= x1 && sx + 3 < x2 {
                return true;
            }
        }
        return false;
    }

    pub fn update_quarter_step(&mut self, quarter: isize, ground_bounding_boxes: [(usize, usize, usize, usize); 5]) -> Option<(isize, isize)> {
        let sx = (self.x as isize + (self.dx * quarter / 40)).mod_floor(&MOVE_WIDTH);
        let mut sy = self.y as isize + (self.dy * quarter / 40);

        if !self.dead {
            if sy >= 23 {
                if self.die() {
                    return None
                }
            } else if sy < 0 {
                self.dy = -self.dy;
                sy = 0
            }
            
            self.on_ground =  self.is_on_ground(sx as usize, sy as usize, ground_bounding_boxes);
    
            if self.on_ground && self.dy > 0 {
                self.dy = 0
            }
        }
        Some((sx, sy))

    }

    pub fn draw(&self) {
        if self.dx < 0 {
            plot(
                '_',
                self.x,
                self.y,
                ColorCode::new(Color::White, Color::Black),
            );
            plot(
                184u8 as char,
                self.x + 1,
                self.y,
                ColorCode::new(Color::Brown, Color::Black),
            );
            plot(
                2u8 as char,
                self.x + 2,
                self.y,
                ColorCode::new(Color::Yellow, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 1,
                self.y + 1,
                ColorCode::new(Color::Cyan, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 2,
                self.y + 1,
                ColorCode::new(Color::Cyan, Color::Black),
            );
            plot(
                16u8 as char,
                self.x + 3,
                self.y + 1,
                ColorCode::new(Color::Cyan, Color::Black),
            );
            if self.on_ground {
                plot(
                    '|' as char,
                    self.x + 1,
                    self.y + 2,
                    ColorCode::new(Color::Brown, Color::Black),
                );
                plot(
                    '\\' as char,
                    self.x + 2,
                    self.y + 2,
                    ColorCode::new(Color::Brown, Color::Black),
                );
            } else {
                plot(
                    14u8 as char,
                    self.x + 1,
                    self.y + 2,
                    ColorCode::new(Color::Brown, Color::Black),
                );
            }
        } else {
            plot(
                '_',
                self.x + 3,
                self.y,
                ColorCode::new(Color::White, Color::Black),
            );
            plot(
                213u8 as char,
                self.x + 2,
                self.y,
                ColorCode::new(Color::Brown, Color::Black),
            );
            plot(
                2u8 as char,
                self.x + 1,
                self.y,
                ColorCode::new(Color::Yellow, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 2,
                self.y + 1,
                ColorCode::new(Color::Cyan, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 1,
                self.y + 1,
                ColorCode::new(Color::Cyan, Color::Black),
            );
            plot(
                17u8 as char,
                self.x,
                self.y + 1,
                ColorCode::new(Color::Cyan, Color::Black),
            );
            if self.on_ground {
                plot(
                    '|' as char,
                    self.x + 2,
                    self.y + 2,
                    ColorCode::new(Color::Brown, Color::Black),
                );
                plot(
                    '/' as char,
                    self.x + 1,
                    self.y + 2,
                    ColorCode::new(Color::Brown, Color::Black),
                );
            } else {
                plot(
                    14u8 as char,
                    self.x + 2,
                    self.y + 2,
                    ColorCode::new(Color::Brown, Color::Black),
                );
            }
        }
    }

    pub fn die(&mut self) -> bool {
        if self.lives > 0 {
            self.lives -= 1;
            self.x = BUFFER_WIDTH / 2;
            self.y = BUFFER_HEIGHT / 2;
            self.dx = 0;
            self.dy = 0;
            self.score += 50;
            self.dead = true;
        } else {
            return true;
        }
        false
    }

    pub fn accel_left(&mut self) {
        if self.on_ground {
            self.dx = self.dx - 3
        } else {
            self.dx = self.dx - 2;
        }
    }

    pub fn accel_right(&mut self) {
        if self.dx < 40 {
            if self.on_ground {
                self.dx = self.dx + 3
            } else {
                self.dx = self.dx + 2;
            }
        }
    }

    pub fn flap(&mut self) {
        self.dy = -10
    }
}
