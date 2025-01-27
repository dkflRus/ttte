// use rocket::prelude::*
mod game;


fn main() {
    println!("Hello, world!");
}


fn run_cli(is_user_x:bool)->Result<game::Cell,&'static str>{
    use std::io;

    fn get_move(splitter:Option<char>)->Result<Vec<usize>,&'static str>{
        let splitter={
            match splitter {
                Some(s)=>s,
                None=>' '
            }
        };

        let i;
        io::stdin().read_line(&mut i).expect("failed to readline");

        i.split(splitter)
            .map(|x| x.parse::<usize>()?).collect()
        

        //     let i;
        //     io::stdin().read_line(&mut i).expect("failed to readline");
    
        //     let i=i.split(splitter);
    
        //     let ans=Vec::new();
        //     for elem in i{
        //         ans.push(elem.parse::<usize>().expect({return "d";0usize}));
        //     };
    
        //     ans
        
    }
    
    let g=game::Game::new(1);
    'single_game: loop{

    }
}