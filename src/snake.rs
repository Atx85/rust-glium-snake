pub struct Coord {
    pub x: f32,
    pub y: f32,
    pub sprite: String
}
pub struct Snake  {
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: u8,
    direction: (f32, f32),
    next_direction: (f32, f32),
    speed: f32
}

impl Snake  {
    pub fn new(x: f32, y: f32) -> Snake {
        let body: Vec<Coord> = Vec::new();
        Snake {
            head: Coord{x, y, sprite: "head_right".to_string()},
            body,
            length: 1,
            direction: (0.01, 0.0),
            next_direction: (0.01, 0.0),
            speed: 0.01
        }
    }
    pub fn move_up(&mut self) { 
        self.next_direction = (0.0, self.speed);
    }
    pub fn move_down(&mut self) { 
        self.next_direction = (0.0, -self.speed);
    }
    pub fn move_left(&mut self) { 
        self.next_direction = (-self.speed, 0.0);
    }
    pub fn move_right(&mut self) { 
        self.next_direction = (self.speed, 0.0);
    }

    pub fn update(&mut self) {
        let (dir_x, dir_y) = self.next_direction;
        self.direction = self.next_direction;
        self.head.x += dir_x;
        self.head.y += dir_y;
        
        match self.direction {
            (_, y) if y < 0.0 => self.head.sprite = "head_down".to_string(),
            (_, y) if y > 0.0 => self.head.sprite = "head_up".to_string(),
            (x,  _) if x < 0.0 => self.head.sprite = "head_left".to_string(),
            (x,  _) if x > 0.0 => self.head.sprite = "head_right".to_string(),
            _ => (),
        }
    }
}