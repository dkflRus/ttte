use crate::game::Game;
use rand::{self, Rng};

pub trait Bot{ //Basic Bot Fucntions
    fn make_move(&self,g:&Game)->Vec<usize>;
}

pub struct RandomMove;
impl Bot for RandomMove {
    fn make_move(&self,g:&Game)->Vec<usize> where Self: Sized{
        let mut rng = rand::thread_rng();
        
        (0..g.get_layer_n())
        .map(|_| rng.gen_range(0..=8usize)) // Generate random integers in the specified range
        .collect()
    }
}