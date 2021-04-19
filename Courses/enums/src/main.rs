enum Direction {
	Up,
	Down,
	Left,
	Right //variants
}
fn main() {
	let player_direction:Direction = Direction::Up;
	// putting :: permit access to variant

	// switch case__ match with ocaml
	match player_direction {
		Direction::Up => println!("Up"),
		Direction::Down => println!("Down"),
		Direction::Right => println!("Right"),
		Direction::Left => println!("Left"),
	}

}
