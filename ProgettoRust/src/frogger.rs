use std::any::Any;

use crate::{actor::*};
use crate::rand::*;

//Struttura per definire il fiume 
pub struct River {
    pos: Pt,
}
impl River {
    pub fn new(pos: Pt) -> River {
        River{pos: pos}
    }
}
impl Actor for River {
    fn act(&mut self, _arena: &mut ArenaStatus) {
    }
    fn sprite(&self) -> Option<Pt> { 
        None
    }
    fn pos(&self) -> Pt { 
        self.pos 
    }
    fn size(&self) -> Pt { 
        pt(672, 224) 
    } 
    fn alive(&self) -> bool { 
        true 
    }
    fn as_any(&self) -> &dyn Any { 
        self 
    }
    fn speed(&self) -> i32 {
        0
    }
}


// Struttura per rappresentare la zattera nel gioco
pub struct Raft {
    pos: Pt,         // Posizione della zattera
    speed: i32,      // Velocità della zattera
    raft_sprite: i32,  // Tipo di zattera
    step: Pt,        // Passo della zattera
    size: Pt,        // Dimensioni della zattera
   
}

impl Raft {
    // Crea una nuova istanza di Raft
    pub fn new(pos: Pt, speed: i32, raft_sprite: i32) -> Raft {
        Raft{
            pos: pos,
            speed: speed,
            raft_sprite: raft_sprite,
            step: pt(1, 0),
            size: pt(if raft_sprite == 0 { 64 } else { 96 } , 32)
        }
    }
}

impl Actor for Raft {
    fn act(&mut self, arena: &mut ArenaStatus) {
        // Controlla se la zattera si è spostata oltre i limiti dell'arena e, in tal caso, la fa riapparire dall'altro lato
        if self.pos.x < -300 {
            self.pos.x = arena.size().x + 300;
        }
        if self.pos.x > arena.size().x + 300 {
            self.pos.x = -300;
        }

        // Imposta il passo della zattera in base alla sua velocità
        self.step.x = self.speed;

        // Aggiorna la posizione della zattera
        self.pos = self.pos + self.step;
    }

    // Restituisce la posizione della zattera
    fn pos(&self) -> Pt {
        self.pos
    }

    // Restituisce le dimensioni della zattera
    fn size(&self) -> Pt {
        self.size
    }

    // Restituisce l'immagine della zattera
    fn sprite(&self) -> Option<Pt> {
        Some(pt(if self.raft_sprite == 0 { 224 } else { 192 }, 96))
    }

    // Restituisce lo stato di vita della zattera (sempre vero)
    fn alive(&self) -> bool {
        true
    }

    // Restituisce la zattera
    fn as_any(&self) -> &dyn Any {
        self
    }

    // Restituisce la velocità della zattera
    fn speed(&self) -> i32 {
        self.speed
    }
}

// Struttura per rappresentare l'auto nel gioco
pub struct Car {
    pos: Pt,          // Posizione dell'auto
    car_or_truk: bool,        // Tipo di auto (true se è una macchina normale, false se è un camion)
    car_sprite: i32,    // Tipo di auto (identificativo per selezionare l'immagine)
    step: Pt,         // Passo dell'auto
    size: Pt,         // Dimensioni dell'auto
    speed: i32,       // Velocità dell'auto
   
}

impl Car {
    // Crea una nuova istanza di Car
    pub fn new(pos: Pt, car_or_truk: bool, speed: i32) -> Car {
        Car{
            pos: pos,
            car_or_truk: car_or_truk,
            car_sprite: randint(6, 9),
            step: pt(1, 0),
            size: pt(if car_or_truk { 32 } else { 64 }, 32),
            speed: speed
        }
    }
}

impl Actor for Car {
    fn act(&mut self, arena: &mut ArenaStatus) {
        // Controlla se l'auto si è spostata oltre i limiti dell'arena e, in tal caso, la fa riapparire dall'altro lato
        if self.pos.x < -300 {
            self.pos.x = arena.size().x + 300;
        }
        if self.pos.x > arena.size().x + 300 {
            self.pos.x = -300;
        }

        // Imposta il passo dell'auto in base alla sua velocità
        self.step.x = self.speed;

        // Aggiorna la posizione dell'auto
        self.pos = self.pos + self.step;
    }

    // Restituisce la posizione dell'auto
    fn pos(&self) -> Pt {
        self.pos
    }

    // Restituisce le dimensioni dell'auto
    fn size(&self) -> Pt {
        self.size
    }

    // Restituisce l'immagine dell'auto in base al suo tipo e velocità
    fn sprite(&self) -> Option<Pt> {
        if self.car_or_truk {
            Some(pt(
                self.car_sprite * 32,
                if self.speed >= 0 && self.car_sprite == 8 {
                    32
                } else if self.speed < 0 && self.car_sprite == 8 {
                    0
                } else if self.speed >= 0 {
                    0
                } else {
                    32
                },
            ))
        } else {
            Some(pt(if self.speed >= 0 { 256 } else { 192 }, 64))
        }
    }

    // Restituisce lo stato di vita dell'auto (sempre vero)
    fn alive(&self) -> bool {
        true
    }

    // Restituisce l'auto 
    fn as_any(&self) -> &dyn Any {
        self
    }

    // Restituisce la velocità dell'auto
    fn speed(&self) -> i32 {
        self.speed
    }
}

// Struttura per rappresentare la rana nel gioco
pub struct Frog {
    pos: Pt,         // Posizione della rana
    step: Pt,        // Passo della rana
    size: Pt,        // Dimensioni della rana
    count_steps: i32,// Contatore per il numero di passi
    drag: i32,       // Velocità di trascinamento (quando la rana è su una zattera)
    flag_raft: bool,   // Flag per indicare se la rana è su una zattera
    speed: i32,      // Velocità della rana
    is_drowning: bool, //rana in acqua
    lives: i32,      // Vite rimanenti della rana
}

impl Frog {
    // Crea una nuova istanza di Frog
    pub fn new(pos: Pt) -> Frog {
        Frog{
            pos: pos,
            step: pt(0, 0),
            size: pt(32, 32),
            count_steps: 0,
            drag: 0,
            flag_raft: false,
            speed: 32,
            is_drowning: false,
            lives: 3,
        }
    }

    // Restituisce il numero di vite rimanenti della rana
    fn lives(&self) -> i32 {
        self.lives
    }
}

impl Actor for Frog {
    fn act(&mut self, arena: &mut ArenaStatus) {
        // La rana non è su una zattera
        self.flag_raft = false;
        self.is_drowning =false;
        // Controlla le collisioni della rana con gli altri attori nell'arena
        for actor in arena.collisions() {
            // Se la rana ha colliso con un'auto
            if let Some(_) = actor.as_any().downcast_ref::<Car>() {
                self.lives -= 1; // Decrementa il numero di vite rimanenti

                self.pos.x = arena.size().x/2; //riposiziona rana
                self.pos.y = 416; 
            }
            // Se la rana ha colliso con una zattera
            if let Some(_) = actor.as_any().downcast_ref::<Raft>() {
                self.flag_raft = true; 
                if self.count_steps == 0 {
                    self.drag = actor.speed();
                }
            }
            //se la rana è in acqua
            if let Some(_) = actor.as_any().downcast_ref::<River>() {
                self.is_drowning = true;
            }
        }

        // Ottiene gli input dell'utente
        let keys = arena.current_keys();
        self.step = pt(0, 0);
        // Controlla gli input dell'utente e imposta il passo della rana
        if self.count_steps == 0 {
            
            if keys.contains(&"ArrowUp") {
                self.count_steps = self.speed;
                self.step.y = -self.speed;
                self.step.x = 0;
            } 
            if keys.contains(&"ArrowDown") {
                self.count_steps = self.speed;
                self.step.y = self.speed;
                self.step.x = 0;
            }
            if keys.contains(&"ArrowLeft") {
                self.count_steps = self.speed;
                self.step.x = -self.speed;
                self.step.y = 0;
            } 
            if keys.contains(&"ArrowRight") {
                self.count_steps = self.speed;
                self.step.x = self.speed;
                self.step.y = 0;
            }
        }

        if self.count_steps > 0 {
            // Aggiorna la posizione della rana
            self.pos.x += self.step.x;
            self.pos.y += self.step.y;
            self.count_steps -= 16;
        }

        // Aggiunge la velocità di trascinamento alla posizione della rana
        self.pos.x += self.drag;
        self.drag = 0;

        // Controlla se la rana è uscita dai limiti dell'arena
        if self.pos.x < 0 {
            self.pos.x = 0;
        }
        if self.pos.x > arena.size().x - self.size.x {
            self.pos.x = arena.size().x - self.size.x;
        }
        if self.pos.y < 0 {
            self.pos.y = 0;
        }
        if self.pos.y > arena.size().y - self.size.y {
            self.pos.y = arena.size().y - self.size.y;
        }

          //controllo se si trova in acqua
        if self.is_drowning && !self.flag_raft{
            self.lives -= 1;  //decrementa le vite

            self.pos.x = arena.size().x/2; //riposiziona rana
            self.pos.y = 416;
        }
        
    }
    //Restituisce posizione della rana
    fn pos(&self) -> Pt { 
        self.pos 
    }  
    //Restituisce dimensione della rana
    fn size(&self) -> Pt {
         self.size 
    }
    //Restituisce immagine della rana
    fn sprite(&self) -> Option<Pt> {
        Some(pt(0, 0)) 
    }
    //Restiuisce lo stato della rana
    fn alive(&self) -> bool { 
        self.lives > 0 
    }
    //Restituisce la rana
    fn as_any(&self) -> &dyn Any { 
        self 
    }
    //Restituisce la velocità della rana
    fn speed(&self) -> i32 {
         self.speed 
    }
}

//Struttura del gioco 
pub struct FroggerGame {
    arena: Arena, //arena di gioco
    playtime: i32 //tempo di gioco
}
impl FroggerGame {
    pub fn new(size: Pt, ncars: i32, nrafts: i32) -> FroggerGame {
        let mut arena = Arena::new(size);                               //crea arena
        arena.spawn(Box::new(Frog::new(pt(arena.size().x/2, 416)))); //crea rana
        arena.spawn(Box::new(River::new(pt(0,0))));                 //crea fiume
        
        // Creazione zattere
        for i in 0..nrafts {
            let mut newpos = 0;
            let speed = randint(1, 5);
            
            for _ in 0..4 {
                arena.spawn(Box::new(Raft::new(pt(newpos, 192-(32*i)), speed, randint(0, 1)))); 
                newpos += randint(100, 450);
            }
        }
        
        //Creazione macchine
        for i in 0..5 {
            let mut newpos = 0;
            let mut speed = randint(2, 7);
            
            if  i%2 != 0 {
                speed = - speed
            }
            for _ in 0..ncars {
                let car = randint(0, 1);
                arena.spawn(Box::new(Car::new(pt(newpos, 384-(32*i)), if car == 1 {true} else {false}, speed)));
                newpos += randint(70, 300);
            }
        }

        FroggerGame{arena: arena, playtime: 120}
    }
    pub fn game_over(&self) -> bool { self.remaining_lives() <= 0 || self.remaining_time()<=0} 
    pub fn game_won(&self) -> bool {self.win_position()==1}

    // Condizione di vittoria
    pub fn win_position(&self) -> i32{
        let mut pos_win:i32=0;
        let actors = self.actors();
        if let Some(b) = actors.first() {
            if let Some(hero) = b.as_any().downcast_ref::<Frog>() {
                if hero.pos.y < 40 && (hero.pos.x>30 && hero.pos.x<80 || hero.pos.x>150 && hero.pos.x<210 || hero.pos.x>280 && hero.pos.x<340 || hero.pos.x>410 && hero.pos.x<470 || hero.pos.x>540 &&  hero.pos.x<600){
                   pos_win = 1;   
                } else{
                   pos_win = 0;
                }
            }
        }
        pos_win
    } 

    //Calcolo tempo rimanente
    pub fn remaining_time(&self) -> i32 {
        self.playtime - self.arena.count() / 30
    }

    //Calcolo vite rimaste alla rana
    pub fn remaining_lives(&self) -> i32 {
        let mut lives = 0;
        let actors = self.actors();
        if let Some(b) = actors.first() {
            if let Some(hero) = b.as_any().downcast_ref::<Frog>() {
                lives = hero.lives();
            }
        }
        lives
    }
    pub fn tick(&mut self, keys: String) { self.arena.tick(keys); }
    pub fn size(&self) -> Pt { self.arena.size() }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { self.arena.actors() }
}
