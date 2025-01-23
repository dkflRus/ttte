
enum cell{
	X,
	O,
	Empty
}


struct BoardLayer{
	is_cell:bool,
	winner:cell,
	subboards:Some([BoardLayer;9])
}
impl BoardLayer{
	fn new(){

  }
	fn check(&self)->cell{ //Must be done only after change
    if self.is_cell{self.cell}
		curr_state:[cell;9]={
			if self.is_base_cells{&self.base_cells}
			else{
          if (self::subboards[4]!=cell::Empty&&(
                  self::subboards[0]==self::subboards[4]==self::subboards[8]||
                  self::subboards[1]==self::subboards[4]==self::subboards[7]||
                  self::subboards[2]==self::subboards[4]==self::subboards[6]||
                  self::subboards[3]==self::subboards[4]==self::subboards[5]||
                  )
              ){
              self::subboards
          }else if (self::subboards[0]!=cell::Empty&&(
                  self::subboards[0]==self::subboards[1]==self::subboards[2]||
                  self::subboards[0]==self::subboards[3]==self::subboards[6]||
                  )
              ){
              self::subboards
          }
      }
		}
	}
}



