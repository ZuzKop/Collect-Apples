use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 25f32;
const ENEMY_SIZE: Vec2 = const_vec2!([165f32, 35f32]);
const PLAYER_SPEED: f32 = 200f32;
const APPLE_SIZE: f32 = 10f32;

pub fn draw_title_text(text: &str, font: Font)
{
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font,
            font_size: 50u16,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub enum GameState{
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

struct Player{
rect: Rect,
has_apple: bool,
}

impl Player{
    pub fn new() -> Self{
        Self {
            rect:Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE * 0.5f32,
                screen_height() - 60f32,
                PLAYER_SIZE,
                PLAYER_SIZE,
            ),
            has_apple: false,
        }
    }

    pub fn update(&mut self, dt: f32){
        let mut x_move = 0f32;
        let mut y_move = 0f32;
        if is_key_down(KeyCode::Left){
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right){
            x_move += 1f32;
        }
        if is_key_down(KeyCode::Up){
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down){
            y_move += 1f32;
        }

        self.rect.x += x_move * dt * PLAYER_SPEED;
        self.rect.y += y_move * dt * PLAYER_SPEED;
        
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }

    }

    pub fn restart_position(&mut self)
    {
        self.rect.x = screen_width() * 0.5f32 - PLAYER_SIZE * 0.5f32;
        self.rect.y = screen_height() - 60f32;

        
        self.has_apple = false;
    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Enemy {
    rect: Rect,
    vel: i32,
    speed: f32,
}

impl Enemy{
    pub fn new(pos: Vec2) -> Self{
        Self{
            rect: Rect::new(pos.x, pos.y, ENEMY_SIZE.x, ENEMY_SIZE.y),
            vel: rand::gen_range(0, 1) * 2 - 1,
            speed: 200f32,
        }
    }

    pub fn change_speed(&mut self)
    {
        self.speed = rand::gen_range(2.0, 4.0) * 100.0;

    }

    pub fn update(&mut self, dt: f32){

        self.rect.x += self.vel as f32 * dt * self.speed;

        if self.rect.x < 0f32{
            self.vel = 1i32;
            self.change_speed();
        }

        if self.rect.x > screen_width() - self.rect.w{
            self.vel = -1i32;
            self.change_speed();
        }

    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }


}

pub struct Apple{
    rect: Rect,
    picked: bool,
}

impl Apple {
    pub fn new() -> Self{
        Self{
            rect: Rect::new(
                rand::gen_range(1, 9) as f32 / 10f32 * screen_width(),
                60f32 + rand::gen_range(0, 2) as f32 * 80f32,
                APPLE_SIZE,
                APPLE_SIZE),
            picked: false,
        }
    } 

    pub fn update(&mut self, new_pos: &Rect, dt: f32)
    {
        self.rect.x = new_pos.x + 2f32;
        self.rect.y = new_pos.y + 1f32;
    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }
}

pub struct Basket{
    rect: Rect,
}

impl Basket{
    pub fn new() -> Self{
        Self{
            rect: Rect::new(screen_width()*0.5f32 -100f32 , screen_height() - 30f32, 200f32, 20f32),
        }
    }
    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, ORANGE);
    }
}

fn pick_apple_collision(a: &mut Rect, aa: &mut bool, b: &Rect, ba: &mut bool) -> bool{
    if let Some(_intersection) = a.intersect(*b) {
        
        return true;
    }
    false
}

fn return_apple_collision(a: &Rect, b: &Rect, ba: &mut bool) -> bool{
    if let Some(_intersection) = a.intersect(*b) {
        
        return true;
    }
    false
}

fn enemy_player_collision(a: &Rect, b: &Rect) -> bool{
    if let Some(_intersection) = a.intersect(*b) {
        
        return true;
    }
    false
}

fn reset_game(
    score: &mut i32,
    lives: &mut i32,
){
    *lives = 3;
    *score = 0;
}

#[macroquad::main("projekt")]
async fn main() {
    let font = load_ttf_font("res/Comfortaa-VariableFont_wght.ttf")
        .await
        .unwrap();
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut lives = 3;
    let mut player = Player::new();
    let mut enemies = Vec::new();
    let mut apples = Vec::<Apple>::new();
    let basket = Basket::new();

    let enemy_number = 4;
    for i in 0..enemy_number{
        let pos_y =  80f32 + i as f32 * ENEMY_SIZE.y + i as f32 * 45f32;
        let pos_x = rand::gen_range(10,500) as f32;
        enemies.push(Enemy::new(vec2(pos_x, pos_y)));
    }

    apples.push(Apple::new());

    loop{
        clear_background(WHITE);
        
        match game_state{
            GameState::Menu =>{
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            },

            GameState::Game =>{
                
                player.update(get_frame_time());

                for enemy in enemies.iter_mut(){
                    enemy.update(get_frame_time());
                }
         
                for apple in apples.iter_mut(){
                    if pick_apple_collision(&mut apple.rect, &mut apple.picked, &player.rect, &mut player.has_apple){
                        apple.picked = true;
                        player.has_apple = true;
                    }
                    if apple.picked == true{
                        apple.update(&player.rect, get_frame_time());
                    }


                }
                
                if return_apple_collision(&basket.rect, &player.rect, &mut player.has_apple) && player.has_apple == true {
                    player.has_apple = false;
                    score += 1;
                    if score >= 5{
                        game_state = GameState::LevelCompleted;
                    }
                    apples.push(Apple::new());
                    apples.retain(|apple| apple.picked == false );
                }
        
                for enemy in enemies.iter_mut(){
                    if enemy_player_collision(&enemy.rect, &player.rect){
                        player.restart_position();
                        apples.retain(|apple| apple.picked == false );
                        if apples.len() < 1 {
                        apples.push(Apple::new());
                        }
                        lives -= 1;
                        if lives <=0{
                            game_state = GameState::Dead;
                        }
                    }
                }

                draw_text_ex(
                    &format!("{}",score),
                    screen_width() * 0.5f32 + 105f32,
                    screen_height() - 10f32,
                    TextParams {
                        font,
                        font_size: 24u16,
                        color: RED,
                        ..Default::default()
                    }
        
                );
                draw_text_ex(
                    &format!("lives: {}",lives),
                    15f32,
                    30f32,
                    TextParams {
                        font,
                        font_size: 24u16,
                        color: BLACK,
                        ..Default::default()
                    }
                );       

            },

            GameState::LevelCompleted =>{
                if is_key_pressed(KeyCode::Space) {
                    reset_game(&mut score, &mut lives);
                    game_state = GameState::Menu;
                }
            },

            GameState::Dead =>{
                if is_key_pressed(KeyCode::Space) {
                    reset_game(&mut score, &mut lives);
                    game_state = GameState::Menu;
                }
            },

        }

        player.draw();
        basket.draw();
        for enemy in enemies.iter(){
            enemy.draw();
        } 
               
        for apple in apples.iter(){
            apple.draw();
        }

        match game_state{
            GameState::Menu =>{
                draw_title_text("Press SPACE to start", font);
            },

            GameState::Game =>{

                draw_text_ex(
                    &format!("{}",score),
                    screen_width() * 0.5f32 + 105f32,
                    screen_height() - 10f32,
                    TextParams {
                        font,
                        font_size: 24u16,
                        color: RED,
                        ..Default::default()
                    }
        
                );
                let instruction_dims = measure_text("collect apples and put them in the basket", Some(font), 16u16, 1.0f32);
                draw_text_ex(
                    &format!("collect apples and put them in the basket"),
                    screen_width() - instruction_dims.width - 10f32,
                    20f32,
                    TextParams {
                        font,
                        font_size: 16u16,
                        color: BLUE,
                        ..Default::default()
                    }
        
                );
                draw_text_ex(
                    &format!("lives: {}",lives),
                    15f32,
                    30f32,
                    TextParams {
                        font,
                        font_size: 24u16,
                        color: BLACK,
                        ..Default::default()
                    }
                );
        

            },

            GameState::LevelCompleted =>{
                draw_title_text(&format!("you won! score: {}", score), font);
            },

            GameState::Dead =>{
                draw_title_text(&format!("you died! score: {}", score), font);
            },

        }
        
        next_frame().await
    }
}
