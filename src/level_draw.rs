use num::Integer;
use pluggable_interrupt_os::vga_buffer::{plot, Color, ColorCode};

pub fn draw_platforms() {
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
    for x in 35..50 {
        plot(' ', x, 7, ColorCode::new(Color::Brown, Color::Brown));
    }
}

pub fn draw_lava() {
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

pub fn draw_ui(score: usize, lives: usize, wave: usize) {
    for x in 21..59 {
        plot(' ', x, 24, ColorCode::new(Color::Brown, Color::Brown))
    }
    draw_score(23, 24, score, Color::Brown);
    for x in 0..lives {
        plot(
            1u8 as char,
            x + 35,
            24,
            ColorCode::new(Color::Yellow, Color::Brown),
        );
    }
    draw_wave(45, 24, wave);
}

pub fn draw_score(sx: usize, sy: usize, score: usize, back_color: Color) {
    for x in 0usize..9 {
        let d = 10_usize.pow(9 - x as u32);
        let v = 10_usize.pow(8 - x as u32);
        let num = score.mod_floor(&d) / v + 48;
        plot(
            num as u8 as char,
            x + sx,
            sy,
            ColorCode::new(Color::Yellow, back_color),
        );
    }
}

pub fn draw_wave(sx: usize, sy: usize, wave: usize) {
    plot('W', sx, sy, ColorCode::new(Color::Yellow, Color::Brown));
    plot('a', sx+1, sy, ColorCode::new(Color::Yellow, Color::Brown));
    plot('v', sx+2, sy, ColorCode::new(Color::Yellow, Color::Brown));
    plot('e', sx+3, sy, ColorCode::new(Color::Yellow, Color::Brown));
    plot(':', sx+4, sy, ColorCode::new(Color::Yellow, Color::Brown));
    for x in 0usize..2 {
        let d = 10_usize.pow(2 - x as u32);
        let v = 10_usize.pow(1 - x as u32);
        let num = wave.mod_floor(&d) / v + 48;
        plot(
            num as u8 as char,
            x + sx + 6,
            sy,
            ColorCode::new(Color::Yellow, Color::Brown),
        );
    }
}

pub fn draw_titlescreen() {
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
        'P',
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

pub fn draw_game_over(player_score: usize) {
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

    const SCORE_X: usize = 30;
    const SCORE_Y: usize = 12;
    // Game Over
    plot(
        'Y',
        SCORE_X,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'o',
        SCORE_X+1,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'u',
        SCORE_X+2,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'r',
        SCORE_X+3,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        's',
        SCORE_X+5,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'c',
        SCORE_X+6,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'o',
        SCORE_X+7,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'r',
        SCORE_X+8,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        'e',
        SCORE_X+9,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );
    plot(
        ':',
        SCORE_X+10,
        SCORE_Y,
        ColorCode::new(Color::Yellow, Color::Black),
    );

    draw_score(SCORE_X+12, SCORE_Y, player_score, Color::Black);

    // Play again
    const OPTIONS_X: usize = 34;
    const OPTIONS_Y: usize = 15;

    plot('P', OPTIONS_X, OPTIONS_Y, ColorCode::new(Color::White, Color::Green));
    plot('P', OPTIONS_X+2, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('l', OPTIONS_X+3, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('a', OPTIONS_X+4, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('y', OPTIONS_X+5, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('A', OPTIONS_X+7, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('g', OPTIONS_X+8, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('a', OPTIONS_X+9, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('i', OPTIONS_X+10, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));
    plot('n', OPTIONS_X+11, OPTIONS_Y, ColorCode::new(Color::White, Color::Black));

    // Quit
    plot('Q', OPTIONS_X, OPTIONS_Y+2, ColorCode::new(Color::White, Color::Red));
    plot('Q', OPTIONS_X+2, OPTIONS_Y+2, ColorCode::new(Color::White, Color::Black));
    plot('u', OPTIONS_X+3, OPTIONS_Y+2, ColorCode::new(Color::White, Color::Black));
    plot('i', OPTIONS_X+4, OPTIONS_Y+2, ColorCode::new(Color::White, Color::Black));
    plot('t', OPTIONS_X+5, OPTIONS_Y+2, ColorCode::new(Color::White, Color::Black));
    
}