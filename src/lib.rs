
/// A Struct representing a printable table
pub struct Table {
	num_cols: usize,
	titles: Vec<String>,
	rows: Vec<Vec<String>>
}

impl Table {
	/// Create a new table with the number of columns equals to the length of `titles`
	pub fn new(titles: Vec<String>) -> Table {
		let n = titles.len();
		return Table {
			num_cols: n,
			titles: titles, 
			rows: Vec::new()
		};
	}
	
	/// Get the number of column
	pub fn get_column_num(&self) -> usize {
		return self.num_cols;
	}
	
	/// Get the number of rows
	pub fn get_rows_number(&self) -> usize {
		return self.rows.len();
	}
	
	/// Get a mutable reference to a row
	pub fn get_mut_row(&mut self, row: usize) -> &mut Vec<String> {
		return &mut self.rows[row];
	}
	
	/// Get an immutable reference to a row
	pub fn get_row(&self, row: usize) -> &Vec<String> {
		return &self.rows[row];
	}
	
	/// Append a row in the table, transferring ownership of this row to the table
	/// and returning a mutable reference to the row
	pub fn add_row(&mut self, row: Vec<String>) -> Result<&mut Vec<String>, &str> {
		if row.len() != self.num_cols {
			return Err("Row does not have the proper number of column");
		}
		self.rows.push(row);
		let l = self.rows.len()-1;
		return Ok(self.get_mut_row(l));
	}
	
	/// Append an empty row in the table. Return a mutable reference to this new row.
	pub fn add_empty_row(&mut self) -> Result<&mut Vec<String>, &str> {
		let n = self.num_cols;
		return Ok(try!(self.add_row(vec!["".to_string(); n])));	
	}
	
	/// Modify a single element in the table
	pub fn set_element(&mut self, element: String, column: usize, row: usize) -> Result<(), &str> {
		if column >= self.num_cols {
			return Err("Column index is higher than expected");
		}
		let rowline: &mut Vec<String>;
		if row > self.rows.len() {
			rowline = try!(self.add_empty_row());
		}
		else {
			rowline = self.get_mut_row(row);
		}
		rowline[column] = element;
		return Ok(());
	}
	
	/// Remove a row
	pub fn remove_row(&mut self, row: usize) {
		self.rows.remove(row);
	}
	
	fn get_col_width(&self, col_idx: usize) -> Result<usize, &str> {
		if col_idx >= self.num_cols {
			return Err("Column index is too high");
		}
		let mut width = self.titles[col_idx].len();
		for r in &self.rows {
			let l = r[col_idx].len();
			if l > width {
				width = l;
			}
		}
		return Ok(width);
	}
	
	fn print_line_separator(&self, col_width: &[usize]) {
		print!("+");
		for i in 0..self.num_cols {
			for _ in 0..(col_width[i] + 2) {
				print!("-");
			}
			print!("+");
		}
		println!("");
	}
	
	fn print_line(&self, line: &[String], col_width: &[usize]) {
		print!("|");
		for i in 0..self.num_cols {
			print!(" {} ", line[i]);
			for _ in 0..(col_width[i] - line[i].len()) {
				print!(" ");
			}
			print!("|");
		}
		println!("");
	}
	
	/// Print the table to `stdout`
	pub fn print(&self) {
		let mut col_width = vec![0usize; self.num_cols];
		for i in 0..self.num_cols {
			col_width[i] = self.get_col_width(i).unwrap();
		}
		self.print_line_separator(&col_width);
		self.print_line(&self.titles, &col_width);
		self.print_line_separator(&col_width);
		for r in &self.rows {
			self.print_line(r, &col_width);
			self.print_line_separator(&col_width);
		}
	}
}