use chrono::{Duration, Utc};
use egui;
use rand::Rng;

pub struct Secret {
    konami_sequence: Vec<egui::Key>,
    is_konami_active: bool,
    konami_start_time: Option<chrono::DateTime<Utc>>,
    snake_game: Option<SnakeGame>,
    last_update: chrono::DateTime<Utc>,
    show_game: bool,
}

impl Default for Secret {
    fn default() -> Self {
        Secret {
            konami_sequence: Vec::new(),
            is_konami_active: false,
            konami_start_time: None,
            snake_game: None,
            last_update: Utc::now(),
            show_game: false,
        }
    }
}

const KONAMI_CODE: [egui::Key; 10] = [
    egui::Key::ArrowUp,
    egui::Key::ArrowUp,
    egui::Key::ArrowDown,
    egui::Key::ArrowDown,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::B,
    egui::Key::A,
];

impl Secret {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.random_secret(ctx);

        if self.is_konami_active {
            let now = Utc::now();
            if now.signed_duration_since(self.last_update) >= Duration::milliseconds(500) {
                if let Some(game) = &mut self.snake_game {
                    game.update();
                    self.last_update = now;
                }
            }
        }
    }

    pub fn random_secret(&mut self, ctx: &egui::Context) {
        let input_state = ctx.input(|i| i.raw.clone());

        for event in &input_state.events {
            if let egui::Event::Key { key, pressed, .. } = event {
                if *pressed {
                    self.konami_sequence.push(*key);

                    if self.konami_sequence.len() > KONAMI_CODE.len() {
                        self.konami_sequence.remove(0);
                    }

                    if self.konami_sequence.len() == KONAMI_CODE.len()
                        && self.konami_sequence == KONAMI_CODE
                    {
                        self.is_konami_active = true;
                        self.snake_game = Some(SnakeGame::new());
                        self.konami_start_time = Some(Utc::now());
                        self.show_game = true;
                    }
                }
            }
        }

        if let Some(game) = &mut self.snake_game {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                game.direction = (-1, 0);
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                game.direction = (1, 0);
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                game.direction = (0, -1);
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                game.direction = (0, 1);
            }
        }
    }

    pub fn draw_snake_game(&mut self, ctx: &egui::Context) {
        if self.is_konami_active && self.show_game {
            egui::Window::new("Secret Snake Game")
                .collapsible(false)
                .show(ctx, |ui| {
                    if ui.button("Close").clicked() {
                        self.show_game = false;
                    }
                    if let Some(game) = &self.snake_game {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(200.0, 200.0), egui::Sense::hover());
                        let painter = ui.painter();

                        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);

                        for &(x, y) in &game.snake {
                            painter.rect_filled(
                                egui::Rect::from_min_size(
                                    rect.min + egui::vec2(x as f32 * 10.0, y as f32 * 10.0),
                                    egui::vec2(10.0, 10.0),
                                ),
                                0.0,
                                egui::Color32::GREEN,
                            );
                        }

                        painter.rect_filled(
                            egui::Rect::from_min_size(
                                rect.min
                                    + egui::vec2(
                                        game.food.0 as f32 * 10.0,
                                        game.food.1 as f32 * 10.0,
                                    ),
                                egui::vec2(10.0, 10.0),
                            ),
                            0.0,
                            egui::Color32::RED,
                        );

                        ui.label(format!("Score: {}", game.score));

                        if game.game_over {
                            ui.label("Game Over!");
                        }
                    }
                });
        }
    }
}

struct SnakeGame {
    snake: Vec<(i32, i32)>,
    food: (i32, i32),
    direction: (i32, i32),
    game_over: bool,
    score: i32,
}

impl SnakeGame {
    fn new() -> Self {
        let mut game = SnakeGame {
            snake: vec![(5, 5)],
            food: (0, 0),
            direction: (1, 0),
            game_over: false,
            score: 0,
        };
        game.generate_new_food();
        game
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake[0];
        let new_head = (
            (head.0 + self.direction.0 + 20) % 20,
            (head.1 + self.direction.1 + 20) % 20,
        );

        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        if new_head == self.food {
            self.score += 1;
            self.generate_new_food();
        } else {
            self.snake.pop();
        }
    }

    fn generate_new_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = (rng.gen_range(0..20), rng.gen_range(0..20));
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }
}
