
use num::Integer;
use pluggable_interrupt_os::vga_buffer::{plot, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH};

const MOVE_WIDTH: isize = BUFFER_WIDTH as isize - 4;

#[derive(Copy, Clone, Eq, PartialEq)]

pub enum EnemyType {
    Bounder,
    Hunter,
    ShadowLord,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub dx: isize,
    pub dy: isize,
    pub etype: EnemyType,
    pub score: usize,
    pub lives: usize,
    pub on_ground: bool,
    pub dead: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            x: BUFFER_WIDTH / 2,
            y: BUFFER_HEIGHT / 2,
            score: 0,
            lives: 6,
            etype: EnemyType::Bounder,
            dx: 0,
            dy: 0,
            on_ground: false,
            dead: true,
        }
    }
}

impl Enemy {
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

        if sy >= 23 {
            if self.die() {
                self.dead = true;
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
        Some((sx, sy))

    }

    pub fn draw(&self) {
        const BIRD_COLOR: Color = Color::Green;
        let mut rider_color: Color = Color::Red;
        if self.etype == EnemyType::Hunter {
            rider_color = Color::LightGray;
        } else if self.etype == EnemyType::ShadowLord {
            rider_color = Color::Blue;
        }
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
                ColorCode::new(rider_color, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 1,
                self.y + 1,
                ColorCode::new(BIRD_COLOR, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 2,
                self.y + 1,
                ColorCode::new(BIRD_COLOR, Color::Black),
            );
            plot(
                16u8 as char,
                self.x + 3,
                self.y + 1,
                ColorCode::new(BIRD_COLOR, Color::Black),
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
                ColorCode::new(rider_color, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 2,
                self.y + 1,
                ColorCode::new(BIRD_COLOR, Color::Black),
            );
            plot(
                254u8 as char,
                self.x + 1,
                self.y + 1,
                ColorCode::new(BIRD_COLOR, Color::Black),
            );
            plot(
                17u8 as char,
                self.x,
                self.y + 1,
                ColorCode::new(BIRD_COLOR, Color::Black),
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
        true
    }

    pub fn think(&mut self, player_x: usize, player_y: usize) {
        if self.y > 18 || self.y > player_y {
            self.flap();
        }
        match self.etype {
            EnemyType::Bounder => {
                if self.x > player_x {
                    self.accel_left(10);
                } else if self.x < player_x {
                    self.accel_right(10);
                }
                if self.dx > 10 {
                    self.dx = 10;
                } else if self.dx < -10 {
                    self.dx = -10
                }
            },
            EnemyType::Hunter => {
                if self.x > player_x  {
                    self.accel_left(25);
                } else if self.x < player_x {
                    self.accel_right(25);
                }
                if self.dx > 25 {
                    self.dx = 25;
                } else if self.dx < -25 {
                    self.dx = -25
                }
            },
            EnemyType::ShadowLord => {
                if self.x > player_x {
                    self.accel_left(40);
                } else if self.x < player_x {
                    self.accel_right(40);
                }
                if self.dx > 40 {
                    self.dx = 40;
                } else if self.dx < -40 {
                    self.dx = -40
                }
            },
        }
    }

    fn accel_left(&mut self, speed_limit: isize) {
        if self.dx < speed_limit {
            if self.on_ground {
                self.dx = self.dx - 3
            } else {
                self.dx = self.dx - 2;
            }
        }
    }

    fn accel_right(&mut self, speed_limit: isize) {
        if self.dx < speed_limit {
            if self.on_ground {
                self.dx = self.dx + 3
            } else {
                self.dx = self.dx + 2;
            }
        }
    }

    fn flap(&mut self) {
        self.dy = -15
    }
}
