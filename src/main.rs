use raylib::{ffi::HideCursor, prelude::*};

const BALL_GROW_RATE: f32 = 5.;
const MAX_SPEED: i32 = 400;
const MIN_SPEED: i32 = -400;

#[derive(Debug)]
enum GruvBox {
    FG,
    BG,
    RED,
    GREEN,
    YELLOW,
    ORANGE,
    BLUE,
    PURPLE,
}

impl GruvBox {
    fn get_color(&self) -> Color {
        match self {
            GruvBox::FG => {
                Color::from_hex("ebdbb2").expect("Could not create color from hex value.")
            }
            GruvBox::BG => {
                Color::from_hex("282828").expect("Could not create color from hex value.")
            }
            GruvBox::RED => {
                Color::from_hex("fb4934").expect("Could not create color from hex value.")
            }
            GruvBox::GREEN => {
                Color::from_hex("b8bb26").expect("Could not create color from hex value.")
            }
            GruvBox::YELLOW => {
                Color::from_hex("fabd2f").expect("Could not create color from hex value.")
            }
            GruvBox::ORANGE => {
                Color::from_hex("fe8019").expect("Could not create color from hex value.")
            }
            GruvBox::BLUE => {
                Color::from_hex("83a598").expect("Could not create color from hex value.")
            }
            GruvBox::PURPLE => {
                Color::from_hex("d3869b").expect("Could not create color from hex value.")
            }
            _ => unimplemented!(
                "Create background color for {:?} in GruvBox::get_color().",
                self
            ),
        }
    }
    fn get_random_color() -> Color {
        let colors = [
            GruvBox::RED.get_color(),
            GruvBox::GREEN.get_color(),
            GruvBox::YELLOW.get_color(),
            GruvBox::ORANGE.get_color(),
            GruvBox::BLUE.get_color(),
            GruvBox::PURPLE.get_color(),
        ];
        colors[get_random_value::<i32>(0, colors.len() as i32 - 1) as usize]
    }
}

#[derive(Clone)]
struct Ball {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
    color: Color,
}

impl Ball {
    fn new(position: Vector2, velocity: Vector2, radius: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            radius,
            color,
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("raylib rust ball game")
        .msaa_4x()
        .build();

    unsafe {
        HideCursor();
    }

    let w = rl.get_screen_width() as f32;
    let h = rl.get_screen_height() as f32;
    let mut balls: Vec<Ball> = get_random_balls(2, w, h);
    let window_color = GruvBox::BG.get_color();
    let mut time_elapsed = 0.;
    let mut spawn_timer = 0.;

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();
        time_elapsed += delta;
        spawn_timer += delta;

        if spawn_timer > 10. {
            spawn_timer = 0.;
            balls.push(get_random_ball(w, h));
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(0, 0, 0, 0));
        d.draw_rectangle_rounded(Rectangle::new(0., 0., w, h), 0.025, 10, window_color);

        let cc: String = std::env::var("CIRCLE_COLLISION").unwrap_or("false".to_string());
        if cc == "true" {
            let balls_clone = balls.clone();
            balls = balls
                .into_iter()
                .map(|mut ball| {
                    for other_ball in balls_clone.iter() {
                        if ball.position == other_ball.position {
                            continue;
                        }
                        if check_collision_circles(
                            ball.position,
                            ball.radius,
                            other_ball.position,
                            other_ball.radius,
                        ) {
                            ball.velocity = other_ball.velocity;
                        }
                    }
                    ball
                })
                .collect();
        }

        balls = balls
            .into_iter()
            .map(|mut ball| {
                let shadow_position = Vector2::new(ball.position.x + 2., ball.position.y + 2.);
                d.draw_circle_v(shadow_position, ball.radius, Color::BLACK.fade(0.25));
                d.draw_circle_v(ball.position, ball.radius, ball.color);
                let mut new_position = ball.position + ball.velocity * delta;
                if new_position.x - ball.radius < 0. || new_position.x + ball.radius > w {
                    ball.velocity.x *= -1.;
                    ball.radius += BALL_GROW_RATE;
                }
                if new_position.y - ball.radius < 0. || new_position.y + ball.radius > h {
                    ball.velocity.y *= -1.;
                    ball.radius += BALL_GROW_RATE;
                }

                if ball.radius > h {
                    panic!("Ball is too big!");
                }

                if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    let mouse = d.get_mouse_position();
                    if ball.position.distance_to(mouse) < ball.radius {
                        ball.velocity = (ball.position - mouse).normalized() * MAX_SPEED as f32;
                        ball.radius -= 10.;
                    }
                }

                new_position = Vector2::new(
                    new_position.x.max(ball.radius).min(w - ball.radius),
                    new_position.y.max(ball.radius).min(h - ball.radius),
                );
                ball.position = new_position;
                ball
            })
            .collect();

        d.draw_text(
            format!("Time: {time_elapsed:.2}").as_str(),
            5,
            5,
            20,
            GruvBox::FG.get_color(),
        );
        d.draw_text(
            format!("Balls: {}", balls.len()).as_str(),
            5,
            25,
            20,
            GruvBox::FG.get_color(),
        );
        if draw_button(
            &mut d,
            Vector2::new(w - 60., 5.),
            "EXIT",
            20,
            GruvBox::FG.get_color(),
        ) || d.is_key_pressed(KeyboardKey::KEY_ESCAPE)
        {
            break;
        }

        let mouse = d.get_mouse_position();
        d.draw_text(
            "+",
            mouse.x as i32 - 7,
            mouse.y as i32 - 7,
            30,
            Color::BLACK.fade(0.25),
        );
        d.draw_text(
            "+",
            mouse.x as i32 - 7,
            mouse.y as i32 - 7,
            20,
            GruvBox::FG.get_color(),
        );
    }
}

fn get_random_ball(w: f32, h: f32) -> Ball {
    let x: i32 = get_random_value(0, w as i32);
    let y: i32 = get_random_value(0, h as i32);
    let vx: i32 = get_random_value(MIN_SPEED, MAX_SPEED);
    let vy: i32 = get_random_value(MIN_SPEED, MAX_SPEED);
    let r: i32 = get_random_value(10, 40);
    Ball::new(
        Vector2::new(x as f32, y as f32),
        Vector2::new(vx as f32, vy as f32),
        r as f32,
        GruvBox::get_random_color(),
    )
}

fn get_random_balls(n: i32, w: f32, h: f32) -> Vec<Ball> {
    let mut balls = vec![];
    for _ in 0..n {
        balls.push(get_random_ball(w, h));
    }
    balls
}

fn draw_button(
    d: &mut RaylibDrawHandle,
    position: Vector2,
    text: &str,
    font_size: i32,
    color: Color,
) -> bool {
    let pad = 2.5;
    let mouse = d.get_mouse_position();
    let text_width = measure_text(text, font_size) as f32;
    let rec = Rectangle::new(
        position.x,
        position.y,
        text_width + 2. * pad,
        font_size as f32 + 2. * pad,
    );
    let is_hovered = rec.check_collision_point_rec(mouse);
    let hov_color = match is_hovered {
        true => Color::GRAY.fade(0.3),
        false => Color::GRAY.fade(0.5),
    };
    d.draw_rectangle_rounded_lines(rec, 0.2, 10, 2, color);
    d.draw_rectangle_rounded(rec, 0.1, 10, hov_color);
    d.draw_text(
        text,
        position.x as i32 + pad as i32,
        position.y as i32 + 2 * pad as i32,
        font_size,
        color,
    );
    is_hovered && d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
}
