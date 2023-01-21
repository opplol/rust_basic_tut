use std::{
    io::stdin,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::{Duration, SystemTime},
};

const COAT_SIZE: i32 = 64;
const HALF_PADDLE_SIZE: f64 = 0.2 / 2.0;

struct Game {
    pub ball: f64,
    pub speed: f64,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball: 0.0,
            speed: 0.01,
        }
    }

    pub fn update(&mut self, is_swing: &Mutex<bool>) -> bool {
        self.ball += self.speed;
        let mut is_swing = is_swing.lock().unwrap();
        println!("{:?}", HALF_PADDLE_SIZE);
        let is_hit_left = *is_swing && (-HALF_PADDLE_SIZE..HALF_PADDLE_SIZE).contains(&self.ball);
        let is_hit_right =
            *is_swing && (1.0 - HALF_PADDLE_SIZE..1.0 + HALF_PADDLE_SIZE).contains(&self.ball);
        if is_hit_left || is_hit_right {
            self.strike_back();
        }
        *is_swing = false;
        let out_left = self.ball < -HALF_PADDLE_SIZE;
        let out_right = 1.0 + HALF_PADDLE_SIZE < self.ball;
        !out_left && !out_right
    }

    fn strike_back(&mut self) {
        let is_left = self.ball < 0.5;
        let judgement = if is_left {
            self.ball.abs()
        } else {
            (self.ball - 1.0).abs()
        };
        let is_perfect_timing = judgement < HALF_PADDLE_SIZE / 2.0;

        self.speed *= -1.0 * if is_perfect_timing { 1.1 } else { 1.0 };
    }
}
fn draw(ball: f64) {
    let ball = (COAT_SIZE as f64 * ball).round() as i32;
    let mut buf = String::from(" ");
    buf += "|";
    for i in 0..COAT_SIZE {
        buf += if i == ball { "@" } else { " " };
    }
    buf += "|";
    println!("\x1B[1;1H{}", buf);
}

fn game_loop(game: &mut Game, is_swing: &Mutex<bool>) {
    let mut time = SystemTime::now();
    loop {
        if !game.update(is_swing) {
            break;
        }
        draw(game.ball);
        time += Duration::from_nanos(16_666_667);
        if let Ok(dur) = time.duration_since(SystemTime::now()) {
            sleep(dur);
        }
    }
    println!("Game Over");
}

fn sub_main(is_swing: &Mutex<bool>) -> ! {
    let input = stdin();
    let mut buf = String::new();
    loop {
        input.read_line(&mut buf).unwrap();
        *is_swing.lock().unwrap() = true;
    }
}

fn main() {
    println!("\x1B[2J");
    let is_swing: Arc<Mutex<bool>> = Default::default();
    {
        let is_swing = is_swing.clone();
        spawn(move || sub_main(&is_swing));
    }
    let mut game = Game::new();
    game_loop(&mut game, &is_swing);
}
