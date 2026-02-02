use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
struct Piece {
    width: usize,
    height: usize,
    shape: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

struct FillerAI {
    player: usize,
    my_char: char,
    enemy_char: char,
}

impl FillerAI {
    fn new() -> Self {
        FillerAI {
            player: 0,
            my_char: '\0',
            enemy_char: '\0',
        }
    }

    fn parse_player_info(&mut self, line: &str) {
        if line.contains("p1") {
            self.player = 1;
            self.my_char = 'O';
            self.enemy_char = 'X';
        } else if line.contains("p2") {
            self.player = 2;
            self.my_char = 'X';
            self.enemy_char = 'O';
        }
    }

    fn read_line(&self) -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(_) => String::new(),
        }
    }

    fn parse_map(&mut self) -> Map {
        // Read until we find "Anfield"
        let mut line = self.read_line();
        while !line.contains("Anfield") {
            line = self.read_line();
        }

        // Parse dimensions
        let parts: Vec<&str> = line.split_whitespace().collect();
        let width = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let height = parts.get(2).and_then(|s| s.trim_end_matches(':').parse().ok()).unwrap_or(0);

        // Skip the line with column numbers
        self.read_line();

        let mut grid = Vec::with_capacity(height);
        for _ in 0..height {
            let line = self.read_line();
            // Skip the line number at the beginning
            let grid_line = if let Some(space_pos) = line.find(' ') {
                &line[space_pos + 1..]
            } else {
                &line
            };
            
            let row: Vec<char> = grid_line.chars().take(width).collect();
            grid.push(row);
        }

        Map { width, height, grid }
    }

    fn parse_piece(&mut self) -> Piece {
        // Find "Piece" line
        let mut line = self.read_line();
        while !line.contains("Piece") {
            line = self.read_line();
        }

        // Parse dimensions
        let parts: Vec<&str> = line.split_whitespace().collect();
        let width = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let height = parts.get(2).and_then(|s| s.trim_end_matches(':').parse().ok()).unwrap_or(0);

        let mut shape = Vec::with_capacity(height);
        for _ in 0..height {
            let line = self.read_line();
            let row: Vec<char> = line.chars().take(width).collect();
            shape.push(row);
        }

        Piece { width, height, shape }
    }

    fn is_valid_placement(&self, map: &Map, piece: &Piece, x: isize, y: isize) -> bool {
        // Check if piece is within bounds
        if x < 0 || y < 0 || x + piece.width as isize > map.width as isize || y + piece.height as isize > map.height as isize {
            return false;
        }

        let mut touch_count = 0;

        for py in 0..piece.height {
            for px in 0..piece.width {
                if piece.shape[py][px] == '.' {
                    continue;
                }

                let map_x = (x + px as isize) as usize;
                let map_y = (y + py as isize) as usize;

                let cell = map.grid[map_y][map_x];
                
                // Check if overlapping with enemy
                if cell == self.enemy_char || cell == 'x' || cell == 'o' {
                    return false;
                }

                // Check if touching my territory
                if cell == self.my_char || cell == 'o' || cell == 'x' {
                    touch_count += 1;
                }
            }
        }

        touch_count == 1
    }

    fn find_best_placement(&self, map: &Map, piece: &Piece) -> Coord {
        let mut best = Coord { x: -1, y: -1 };
        let mut max_score = -1;

        for y in 0..=(map.height as isize - piece.height as isize) {
            for x in 0..=(map.width as isize - piece.width as isize) {
                if self.is_valid_placement(map, piece, x, y) {
                    // Simple scoring: count the number of non-dot cells in the piece
                    let mut score = 0;
                    for py in 0..piece.height {
                        for px in 0..piece.width {
                            if piece.shape[py][px] != '.' {
                                score += 1;
                            }
                        }
                    }

                    if score > max_score {
                        max_score = score;
                        best.x = x;
                        best.y = y;
                    }
                }
            }
        }

        best
    }

    fn run(&mut self) {
        // Read player info
        let line = self.read_line();
        self.parse_player_info(&line);

        loop {
            let map = self.parse_map();
            let piece = self.parse_piece();

            let best = self.find_best_placement(&map, &piece);

            if best.x == -1 || best.y == -1 {
                println!("0 0");
            } else {
                println!("{} {}", best.x, best.y);
            }

            io::stdout().flush().unwrap();
        }
    }
}

fn main() {
    let mut ai = FillerAI::new();
    ai.run();
}
