

#[derive(PartialEq,Clone,Copy,Debug)]
pub enum Cell{
	X,
	O,
	Empty
}

#[derive(Debug)]
pub struct BoardLayer{
  // Technically can be used as Game
	pub winner:Cell,
	board:Option<[Box<BoardLayer>;9]> // When None, BoardLayer is a base cell
}

impl BoardLayer {
  /// Creates a new `BoardLayer` instance with the specified depth (`layer_n`).
  /// If `layer_n` is 0, creates a base cell layer filled with `Cell::Empty`.
  /// Otherwise, initializes a board containing 9 sub-layers recursively.
  fn new(layer_n: usize) -> Self {
      if layer_n == 0 {
          // Base layer with no sub-layers
          BoardLayer {
              winner: Cell::Empty,
              board: None,
          }
      } else {
          // Create sub-layers recursively
          BoardLayer {
              winner: Cell::Empty,
              board: Some(
                  array_init::array_init(|_| Box::new(BoardLayer::new(layer_n - 1))),
              ),
          }
      }
  }

  pub fn check(&self)->Cell{ //Must be done only after change
    fn _check_simple(cells:&Vec<Cell>)->Cell{
      if cells[4]!=Cell::Empty&&(
          cells[0]==cells[4] && cells[4]==cells[8]||
          cells[1]==cells[4] && cells[4]==cells[7]||
          cells[2]==cells[4] && cells[4]==cells[6]||
          cells[3]==cells[4] && cells[4]==cells[5]
        )
        {cells[4]}
      else if cells[0]!=Cell::Empty&&(
          cells[0]==cells[1] && cells[1]==cells[2]||
          cells[0]==cells[3] && cells[3]==cells[6]
        )
        {cells[0]}
      else if cells[8]!=Cell::Empty&&(
          cells[6]==cells[7] && cells[7]==cells[8]||
          cells[2]==cells[5] && cells[5]==cells[8]
        )
        {cells[8]}
      else{Cell::Empty}
    }

    
    if 
      self.board.is_none()|| //If base cell
      self.winner!=Cell::Empty //If already has winner
    {self.winner}
    else{
      _check_simple(
  &self.board
        .as_ref() // Borrow the Option
        .unwrap() // Safely unwrap, since we know it's Some
        .iter()
        .map(|x| x.check()) // Call `check()` on each element
        .collect::<Vec<Cell>>() // Collect into a Vec<Cell>
        // .try_into() // Convert Vec<Cell> into [Cell; 9]
        // .expect("Board must always contain exactly 9 cells") // Ensure conversion succeeds// Collect the results into a Vec<Cell> for `_check_simpl
      )
    }
  }

  

}




pub struct Game{
  layer_n:usize,
  pub board:BoardLayer,
  pub is_now_x:bool
}
impl Game{
  pub fn new(layer_n:usize)->Self{
    Game{
      layer_n:layer_n,
      board:BoardLayer::new(layer_n),
      is_now_x:true
    }
  }

  pub fn render_cli(&self,
    divders:Option<Vec<String>>,
    cell_fillers:Option<Vec<String>> //[O,Empty,X]
  )->Result<String,&str>{
    let _dividers:Vec<String>=match divders{
      Some(d)=>{
        if d.len()!=self.layer_n+1{return Err("Size of dividers vector and board size don't match!")}else{d}
      },
      None=>{
        let mut ans:Vec<String>=(1..self.layer_n).map(|x| x.to_string()).collect();
        if let Some(first) = ans.get_mut(0) {
          *first = "".to_string();
        }
        ans
      }
    };
    let cell_fillers:Vec<String>=match cell_fillers{
      Some(d)=>{
        if d.len()!=3{return Err("Not 3 cell fillers!")}else{d}
      },
      None=>{vec!("O".to_string(),"-".to_string(),"X".to_string())}
    };


    let mut ans=String::new();
    for string_n in 0..3usize.pow(self.layer_n as u32){
      for column_n in 0..3usize.pow(self.layer_n as u32){
        //char print
        let mut _curr_str=string_n;
        let mut _curr_col=column_n;
        let mut curr_board:&BoardLayer=&self.board;
        let mut curr_grid_size:usize;
        for _curr_table_n in (0..self.layer_n).rev(){
          curr_grid_size=3usize.pow(_curr_table_n as u32);
          // print!("{} {} {};",curr_grid_size,_curr_str,_curr_col);

          
          curr_board=&(*(curr_board.board.as_ref().unwrap())[3*(_curr_str/curr_grid_size)+(_curr_col/curr_grid_size)]);

          _curr_str=_curr_str%curr_grid_size;
          _curr_col=_curr_col%curr_grid_size;
        }

        // print!("{} {} {} {:?}",curr_grid_size,_curr_str,_curr_col,&curr_board.winner);
        ans+={
          match curr_board.winner{
            Cell::O => &cell_fillers[0],
            Cell::Empty => &cell_fillers[1],
            Cell::X => &cell_fillers[2],
          }
        };

        //Border print
        if column_n%3==0 && column_n!=0{
          // let order_n:usize;
          // 'get_order for _ in 0..self.layer_n{
            
          // }
        }
        
      }
      ans+="\n";
    }



    Ok(ans)
  }

  pub fn place_mark(&mut self,
    tree:Vec<usize>,
    allow_non_empty:bool,
    recheck_board:bool
  )->Result<String,&str>{
    if *tree.iter().max().unwrap()>8usize{return Err("Too big int in `tree`!")}
    if tree.len()!=self.layer_n{return Err("Incorrect `tree` length!")}

    let mut curr_board:&mut BoardLayer=&mut self.board;
    for i in tree.clone(){
      curr_board=&mut (*curr_board.board.as_mut().unwrap()[i]);
    }
    let goal_cell=&mut curr_board.winner;

    if !(allow_non_empty || *goal_cell==Cell::Empty){
      return Err("Can not overwrite filled cell!")
    }else{
      *goal_cell=if self.is_now_x{Cell::X}else{Cell::O};
    }

    self.is_now_x=!self.is_now_x;

    if recheck_board{self.board.check();}


    Ok("".to_string())
  }
}







#[cfg(test)]
mod Game_tests_depth_is_1{
    use super::*;

    fn setup()->Game{
      let mut g=Game::new(1);

      g.place_mark(vec![0], true,true).unwrap();g.is_now_x=true;
      g.place_mark(vec![1], true,true).unwrap();g.is_now_x=true;
      g.place_mark(vec![2], true,true).unwrap();g.is_now_x=false;
      g.place_mark(vec![4], true,true).unwrap();
      g
    }

    #[test]
    fn BoardLayer_check(){
      assert_eq!(setup().board.check(),Cell::X);
    }

    #[test]
    fn BoardLayer_render_cli_borderless(){
      assert_eq!(setup().render_cli(Some(vec!["".to_string(),"".to_string()]),None).unwrap(),"XXX\n-O-\n---\n");
    }
    
}
