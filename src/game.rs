
enum cell{
	X,
	O,
	Empty
}


struct BoardLayer{
	is_base_cells:bool,
	winner:cell
	base_cells:Some([cell;9]),
	subboards:Some([BoardLayer;9])
}
impl BoardLayer{
	fn new(){}
	fn check(&self)->cell{ //Must be done only after change
		curr_state:[cell;9]={
			if self.is_base_cells{&self.base_cells}
			else{}
		}
	}
}

