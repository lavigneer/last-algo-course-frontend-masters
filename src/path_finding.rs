use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn walk(
    maze: &Vec<&str>,
    wall: char,
    curr: Point,
    end: &Point,
    seen: &mut HashSet<Point>,
    path: &mut Vec<Point>,
) -> bool {
    if curr.eq(end) {
        path.push(curr);
        return true;
    } else if seen.contains(&curr) {
        return false;
    }
    seen.insert(curr.clone());
    if let Some(maze_at_y) = maze.get(curr.y as usize) {
        if let Some(maze_at_x) = maze_at_y.get(curr.x as usize..=curr.x as usize) {
            if let Some(ch) = maze_at_x.chars().next() {
                if ch == wall {
                    return false;
                }
                path.push(curr.clone());

                for (x, y) in DIRECTIONS {
                    if walk(
                        maze,
                        wall,
                        Point {
                            x: curr.x + x,
                            y: curr.y + y,
                        },
                        end,
                        seen,
                        path,
                    ) {
                        return true;
                    }
                }
                path.pop();
                return false;
            }
        }
    }
    false
}

fn solve(maze: Vec<&str>, wall: char, start: Point, end: Point) -> Vec<Point> {
    let mut seen = HashSet::new();
    let mut path = Vec::new();
    walk(&maze, wall, start, &end, &mut seen, &mut path);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let maze = vec![
            "xxxxxxxxxx x",
            "x        x x",
            "x        x x",
            "x xxxxxxxx x",
            "x          x",
            "x xxxxxxxxxx",
        ];
        let maze_result = vec![
            Point { x: 10, y: 0 },
            Point { x: 10, y: 1 },
            Point { x: 10, y: 2 },
            Point { x: 10, y: 3 },
            Point { x: 10, y: 4 },
            Point { x: 9, y: 4 },
            Point { x: 8, y: 4 },
            Point { x: 7, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 5, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 4 },
            Point { x: 2, y: 4 },
            Point { x: 1, y: 4 },
            Point { x: 1, y: 5 },
        ];
        let result = solve(maze, 'x', Point { x: 10, y: 0 }, Point { x: 1, y: 5 });
        assert_eq!(result, maze_result);
    }
}
