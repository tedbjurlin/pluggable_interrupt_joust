use num::Integer;
use pluggable_interrupt_os::vga_buffer::{plot, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH};

const MOVE_WIDTH: isize = BUFFER_WIDTH as isize - 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Player {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    pub score: usize,
    pub lives: usize,
    on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: BUFFER_WIDTH / 2,
            y: BUFFER_HEIGHT / 2,
            score: BUFFER_HEIGHT / 2,
            lives: 6,
            dx: 0,
            dy: 0,
            on_ground: true,
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

    pub fn update(&mut self, ground_bounding_boxes: [(usize, usize, usize, usize); 5]) -> bool {
        for (sx, sy) in self.get_quarter_steps() {
            self.x = sx;
            self.y = sy;
            if !self.on_ground && self.is_on_ground(sx, sy, ground_bounding_boxes) {
                self.on_ground = true;
                break;
            } else {
                self.on_ground = false;
            }
            if sy >= 24 {
                return self.die();
            }
        }

        if self.on_ground && self.dy > 0 {
            self.dy = 0
        } else if self.dy < 30 {
            self.dy = self.dy + 5;
        }

        false
    }

    fn is_on_ground(
        &self,
        x: usize,
        y: usize,
        ground_bounding_boxes: [(usize, usize, usize, usize); 5],
    ) -> bool {
        for (x1, y1, x2, y2) in ground_bounding_boxes {
            if y + 3 >= y1 && y + 3 < y2 && x >= x1 && x + 3 < x2 {
                return true;
            }
        }
        return false;
    }

    fn get_quarter_steps(&self) -> [(usize, usize); 4] {
        let mut steps = [(0, 0); 4];
        for i in 0..4 {
            steps[i] = (
                (self.x as isize + (self.dx * (i as isize + 1) / 40)).mod_floor(&MOVE_WIDTH)
                    as usize,
                (self.y as isize + (self.dy * (i as isize + 1) / 40)) as usize,
            )
        }
        steps
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
        } else {
            return true;
        }
        false
    }

    pub fn accel_left(&mut self) {
        if self.dx > -40 {
            self.dx = self.dx - 1;
        }
    }

    pub fn accel_right(&mut self) {
        if self.dx < 40 {
            self.dx = self.dx + 1;
        }
    }

    pub fn flap(&mut self) {
        self.dy = -25
    }
}
