use piston::Key;
use rand::Rng;

use piston::ButtonEvent;
use piston::EventLoop;
use piston::ButtonState;
use piston::Button;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::collections::LinkedList;

#[derive(Clone,PartialEq)]
enum Direction {
    Up,Down,Left,Right
}

trait WorldObject {
    fn render(& self, gl : &mut GlGraphics, args : &RenderArgs);
    fn update(&mut self);
}
struct Apple {
    pos : (i32,i32),
    size : i32
}
struct Snake {
    body : LinkedList<(i32,i32)>,
    dir : Direction,
}
struct Game {
    gl : GlGraphics,
    snake : Snake,
    world : Vec<Apple>,
    time : f64,
    screen_size : (i32,i32),
    score : i32

}
impl Apple {
    fn value(&self) -> i32 {
        self.size / 20
    }
}
impl WorldObject for Apple
{
fn render(& self, gl : &mut GlGraphics, args: &piston::RenderArgs) {
     let red = [1.0,0.,0.,1.0];
     let sq = graphics::rectangle::square((self.pos.0 * self.size) as f64 , 
     (self.pos.1 * 20) as f64, self.size as f64);
     gl.draw(args.viewport(), |c,gl| {
        let transform = c.transform;
        graphics::ellipse(red, sq, transform, gl);
     })

 }
fn update(&mut self) { 
    
 }
}

impl Game
 {
    fn render(&mut self, args : &RenderArgs){
        self.screen_size = (args.window_size[0] as i32,args.window_size[1] as i32);
        let black : [f32;4] = [0.,0.,0.,1.0];
        self.gl.draw(args.viewport(), |_context, gl| {
            graphics::clear(black,gl);

        });
        self.snake.render(&mut self.gl, args);
        for apple in &self.world {
            apple.render(&mut self.gl, args);
        }
    }
    fn update(&mut self, args : &UpdateArgs) {
        
        self.snake.update();
        let mut rng = rand::thread_rng();
        self.time += args.dt;
        //println!("Snake position : {:?}. Apple Positon : {:?}",self.snake.coords(self.screen_size.0,self.screen_size.1),self.world[0].pos);
        if  self.world.iter().any(|apple| self.snake.collides(apple,self.screen_size.0,self.screen_size.1)) {
            self.world.push(Apple{pos: (rng.gen_range(0..(self.screen_size.0/20)),rng.gen_range(0..self.screen_size.1/20)), size : 20});
            self.world.remove(0);
            self.snake.extend();
            self.score += 1;
            
            
        }
        if self.snake.self_collision() {
            println!("Game Over! Score : {}",self.score);
            self.score = 0;
            self.snake = Snake::new(0,0,Direction::Right);
        }
    }
    
    
}

impl Snake {
    fn coords(&self,width : i32, height : i32) -> (i32,i32) {
        ((self.body.front().unwrap().0*20).rem_euclid(width),(self.body.front().unwrap().1 * 20).rem_euclid(height))
    }
    fn new(x : i32, y : i32, dir : Direction) -> Snake {
        let mut body = LinkedList::new();
        body.push_back((x,y));
        Snake {
            body ,
            dir 
        }
    }
    fn render(& self,gl : &mut GlGraphics,args : &RenderArgs){
        let blue = [0.1,0.44,0.1,1.0];
        
        let squares = self.body.iter().map(|&(x,y)| {
            
            graphics::rectangle::square(
                ((x * 20).rem_euclid(args.window_size[0] as i32)) as f64
                ,((y * 20).rem_euclid( args.window_size[1] as i32)) as f64 , 20_f64)
        }
        ).collect::<Vec<graphics::types::Rectangle>>();

        gl.draw(args.viewport(), |c,gl| {
            let transform = c.transform;
            squares.into_iter().for_each(|square | {
                graphics::rectangle(blue, square, transform, gl);
            })
            //

        })

    }
    fn update(&mut self) {
        let mut new_head = self.body.front().unwrap().clone();
        match self.dir {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Right => new_head.0 += 1,
            Direction::Left => new_head.0 -= 1
        }
        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
        println!("Snake : {:?}",self.body);
    }
    fn pressed(&mut self, button : &Button) {
        

        self.dir =  if self.body.len() != 1 {match button {
            &Button::Keyboard(Key::W) => {
                if self.dir == Direction::Down {
                    Direction::Down
                }else
                {Direction::Up}
            },
            &Button::Keyboard(Key::A) => {
                if self.dir == Direction::Right {
                    Direction::Right
                }else
                {Direction::Left}
            },
            &Button::Keyboard(Key::S) => {
                if self.dir == Direction::Up {
                    Direction::Up
                }else
                {Direction::Down}
            },
            &Button::Keyboard(Key::D) => {
                if self.dir == Direction::Left {
                    Direction::Left
                }else
                {Direction::Right}
            },
            _ => self.dir.clone()
        }} else {
            match button {
            &Button::Keyboard(Key::W) => Direction::Up,
            &Button::Keyboard(Key::A) => Direction::Left,
            &Button::Keyboard(Key::S) => Direction::Down,
            &Button::Keyboard(Key::D) => Direction::Right,
            _ => self.dir.clone()

            }

        };
        
        

    }
    fn collides(&self, a : &Apple, size_x : i32,size_y:i32) -> bool {
        self.body.iter().any(|&(x,y)| ((x * 20).rem_euclid(size_x) - a.pos.0 * a.size).abs() <= a.size/2 && ((y * 20).rem_euclid( size_y) - a.pos.1 * a.size).abs() <= a.size/2)
    }
    fn behind_node(&self,node : &(i32,i32) ) -> (i32,i32) {
        
        match self.dir {
            Direction::Up => (node.0,node.1 - 1),
            Direction::Down => (node.0,node.1 + 1),
            Direction::Right => (node.0 - 1, node.1),
            Direction::Left => (node.0 + 1, node.1)

        }
    }
    fn extend(&mut self) {
        self.body.push_back(self.behind_node(self.body.front().unwrap()));
    }
    fn self_collision(&self) -> bool {
        let head = &self.body.front().unwrap();
        self.body.iter().skip(1).any(|&(x,y)| x == head.0 && y == head.1 )
    }
}
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window : Window = WindowSettings::new("Snakey-Boi", [200,200]).
    graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game {
        gl : GlGraphics::new(opengl),
        snake : Snake::new(0,0,Direction::Right),world : vec![Apple{pos : (2,2), size : 20}], time : 0.0, screen_size : (200,200), score : 0
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

         if let Some(args) = e.update_args() {
            game.update(&args);
        } 
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.snake.pressed(&k.button);

            }
            
        }
    }
}
