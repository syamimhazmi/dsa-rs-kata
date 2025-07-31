#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn walk(
    maze: &Vec<String>,
    wall: char,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    // 1. Base case
    // off the map
    if curr.x >= maze[0].len() || curr.y >= maze.len() {
        return false;
    }

    // on a wall
    if maze[curr.y].chars().nth(curr.x).unwrap() == wall {
        return false;
    }

    // are we at the end
    if curr == end {
        path.push(curr);
        return true;
    }

    // seen this position
    if seen[curr.y][curr.x] {
        return false;
    }

    seen[curr.y][curr.x] = true;

    // Recurse
    // pre
    path.push(curr);

    let directions = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];

    // recurse
    for (dx, dy) in directions.iter() {
        let new_x = curr.x as i32 + dx;
        let new_y = curr.y as i32 + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_current = Point::new(new_x as usize, new_y as usize);

            if walk(maze, wall, new_current, end, seen, path) {
                return true;
            }
        }
    }

    // post
    path.pop();

    false
}

fn solver(maze: &Vec<String>, wall: char, start: Point, end: Point) -> Vec<Point> {
    let mut seen = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = Vec::new();

    walk(maze, wall, start, end, &mut seen, &mut path);

    path
}

#[cfg(test)]
mod tests {
    use super::{Point, solver};

    #[test]
    fn test_simple_maze() {
        let maze = vec![
            "xxxxxxxxxx x".to_string(),
            "x        x x".to_string(),
            "x        x x".to_string(),
            "x xxxxxxxx x".to_string(),
            "x          x".to_string(),
            "x xxxxxxxxxx".to_string(),
        ];

        let wall = 'x';
        let start = Point::new(10, 0);
        let end = Point::new(1, 5);

        let path = solver(&maze, wall, start, end);

        assert!(!path.is_empty(), "Should find a path");
        assert_eq!(path[0], start, "Path should start at starting point");
        assert_eq!(path[path.len() - 1], end, "Path should end at end point");
    }

    #[test]
    fn test_no_solution() {
        let maze = vec!["xxx".to_string(), "x x".to_string(), "xxx".to_string()];

        let wall = 'x';
        let start = Point::new(1, 1);
        let end = Point::new(0, 0);

        let path = solver(&maze, wall, start, end);

        assert!(path.is_empty(), "Should not find any path when blocked");
    }

    #[test]
    fn test_start_equals_end() {
        let maze = vec![
            "xxxxx".to_string(),
            "x   x".to_string(),
            "x   x".to_string(),
            "x   x".to_string(),
            "xxxxx".to_string(),
        ];

        let wall = 'x';
        let start = Point::new(2, 2);
        let end = Point::new(2, 2);

        let path = solver(&maze, wall, start, end);

        assert_eq!(path.len(), 1, "Path should contain exactly one point");
        assert_eq!(path[0], start, "Path should contain the start/end point");
    }

    #[test]
    fn test_complex_maze_multiple_paths() {
        let maze = vec![
            "xxxxxxxxxxxxxxxxx".to_string(),
            "x     x         x".to_string(),
            "x xxx x xxxxxxx x".to_string(),
            "x   x x x     x x".to_string(),
            "xxx x x x xxx x x".to_string(),
            "x   x   x   x   x".to_string(),
            "x xxxxxxx xxx xxx".to_string(),
            "x               x".to_string(),
            "xxxxxxxxxxxxxxxxx".to_string(),
        ];

        let wall = 'x';
        let start = Point::new(1, 1);
        let end = Point::new(15, 7);

        let path = solver(&maze, wall, start, end);

        assert!(!path.is_empty(), "Should find a path complex maze");
        assert_eq!(path[0], start, "Path should start at start point");
        assert_eq!(path[path.len() - 1], end, "Path should end at end point");

        // verify path doesn't go through walls
        for point in &path {
            let ch = maze[point.y].chars().nth(point.x).unwrap();

            assert_ne!(ch, wall, "Path should not go through wall");
        }
    }

    #[test]
    fn test_edge_boundries() {
        let maze = vec![
            "     ".to_string(),
            "     ".to_string(),
            "     ".to_string(),
        ];

        let wall = 'x';
        let start = Point::new(0, 0);
        let end = Point::new(4, 2);

        let path = solver(&maze, wall, start, end);

        assert!(!path.is_empty(), "Should find path from corner to corner");
        assert_eq!(path[0], start, "Path should start at top-left");
        assert_eq!(path[path.len() - 1], end, "Path should end at bottom-right");

        // Should find a valid path (could be many valid solutions)
        assert!(
            path.len() >= 3,
            "Path should have reasonable length for this distance"
        );
    }
}
