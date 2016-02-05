enum Direction {
    North,
    East,
    South,
    West,
}

trait MazeObject {
    fn enter();
}

struct Room {
    room_number: u8,
    sides:       [MazeObject; 4],
}

impl Room {
    fn get_side(&self, direction: Direction) {

    }
    fn set_side(&self, direction: Direction, maze_object: &Direction) {
    }
}

struct Wall;

struct Door {
    open: bool,
    room1: Room,
    room2: Room,
}

struct Maze {
    rooms: Vec<Room>,
}

impl Maze {
    fn get_room(n: u8) {
    }
}

struct MazeGame;

#[test]
fn foo() {
}
