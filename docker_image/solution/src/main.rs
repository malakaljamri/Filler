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
    my_last_char: char,
    enemy_char: char,
    enemy_last_char: char,
}

impl FillerAI {
    fn new() -> Self {
        FillerAI {
            player: 0,
            my_char: '\0',
            my_last_char: '\0',
            enemy_char: '\0',
            enemy_last_char: '\0',
        }
    }

    fn parse_player_info(&mut self, line: &str) {
        if line.contains("p1") {
            self.player = 1;
            self.my_char = '@';
            self.my_last_char = 'a';
            self.enemy_char = '$';
            self.enemy_last_char = 's';
        } else if line.contains("p2") {
            self.player = 2;
            self.my_char = '$';
            self.my_last_char = 's';
            self.enemy_char = '@';
            self.enemy_last_char = 'a';
        }
    }

    fn read_line(&self) -> Option<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => None,
            Ok(_) => Some(input.trim().to_string()),
            Err(_) => None,
        }
    }

    fn parse_map_row(&self, line: &str, width: usize) -> Vec<char> {
        let grid_line = line
            .split_whitespace()
            .nth(1)
            .unwrap_or(line);
        grid_line.chars().take(width).collect()
    }

    fn parse_map(&mut self) -> Map {
        // Read until we find "Anfield"
        let mut line = match self.read_line() {
            Some(value) => value,
            None => {
                return Map { width: 0, height: 0, grid: Vec::new() };
            }
        };
        while !line.contains("Anfield") {
            line = match self.read_line() {
                Some(value) => value,
                None => {
                    return Map { width: 0, height: 0, grid: Vec::new() };
                }
            };
        }

        if line.is_empty() {
            return Map { width: 0, height: 0, grid: Vec::new() };
        }

        // Parse dimensions
        let parts: Vec<&str> = line.split_whitespace().collect();
        let width = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let height = parts.get(2).and_then(|s| s.trim_end_matches(':').parse().ok()).unwrap_or(0);

        let mut grid = Vec::with_capacity(height);
        while grid.len() < height {
            let line = match self.read_line() {
                Some(value) => value,
                None => break,
            };

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let is_header = grid.is_empty()
                && trimmed.chars().all(|c| c.is_ascii_digit());
            if is_header {
                continue;
            }

            let row = self.parse_map_row(trimmed, width);
            grid.push(row);
        }

        Map { width, height, grid }
    }

    fn parse_piece(&mut self) -> Piece {
        let mut line = match self.read_line() {
            Some(value) => value,
            None => {
                return Piece { width: 0, height: 0, shape: Vec::new() };
            }
        };

        // Read until we find "Piece" or end of input
        while !line.contains("Piece") {
            line = match self.read_line() {
                Some(value) => value,
                None => {
                    return Piece { width: 0, height: 0, shape: Vec::new() };
                }
            };
        }

        if line.is_empty() || !line.contains("Piece") {
            return Piece { width: 0, height: 0, shape: Vec::new() };
        }

        // Parse dimensions
        let parts: Vec<&str> = line.split_whitespace().collect();
        let width = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let height = parts.get(2).and_then(|s| s.trim_end_matches(':').parse().ok()).unwrap_or(0);

        let mut shape = Vec::with_capacity(height);
        while shape.len() < height {
            let line = match self.read_line() {
                Some(value) => value,
                None => break,
            };
            if line.is_empty() {
                continue;
            }
            let row: Vec<char> = line.chars().take(width).collect();
            shape.push(row);
        }

        Piece { width, height, shape }
    }

    fn is_valid_placement(&self, map: &Map, piece: &Piece, x: isize, y: isize) -> bool {
        // Check if piece is within bounds
        if x < 0 || y < 0 || 
           x + piece.width as isize > map.width as isize || 
           y + piece.height as isize > map.height as isize {
            return false;
        }

        let mut overlap_count = 0;
        let mut has_non_dot = false;

        for py in 0..piece.height {
            for px in 0..piece.width {
                if piece.shape[py][px] == '.' {
                    continue;
                }
                
                has_non_dot = true;
                let map_x = (x + px as isize) as usize;
                let map_y = (y + py as isize) as usize;

                let cell = map.grid[map_y][map_x];
                
                // Check if overlapping with enemy territory
                if cell == self.enemy_char || cell == self.enemy_last_char {
                    return false;
                }

                // Check if overlapping with my territory
                if cell == self.my_char || cell == self.my_last_char {
                    overlap_count += 1;
                }
            }
        }

        // Must have at least one non-dot cell and overlap exactly once
        has_non_dot && overlap_count == 1
    }

    fn calculate_score(&self, map: &Map, piece: &Piece, x: isize, y: isize) -> i32 {
        // Heuristic components
        let mut coverage = 0;          // how many new cells we add
        let mut min_enemy_dist = i32::MAX; // min distance from any placed cell to enemy

        // Pre-collect enemy coordinates once
        let mut enemy_coords: Vec<(isize, isize)> = Vec::new();
        for (yy, row) in map.grid.iter().enumerate() {
            for (xx, ch) in row.iter().enumerate() {
                if *ch == self.enemy_char || *ch == self.enemy_last_char {
                    enemy_coords.push((xx as isize, yy as isize));
                }
            }
        }

        for py in 0..piece.height {
            for px in 0..piece.width {
                if piece.shape[py][px] == '.' {
                    continue;
                }
                coverage += 1;

                // Compute distance to closest enemy cell
                let cell_x = x + px as isize;
                let cell_y = y + py as isize;
                for &(ex, ey) in &enemy_coords {
                    let dist = (cell_x - ex).abs() + (cell_y - ey).abs();
                    if dist < min_enemy_dist as isize {
                        min_enemy_dist = dist as i32;
                    }
                }
            }
        }

        // If no enemy cells, stay near center to speed filling
        if enemy_coords.is_empty() {
            let center_x = map.width as isize / 2;
            let center_y = map.height as isize / 2;
            min_enemy_dist = ((x - center_x).abs() + (y - center_y).abs()) as i32;
        }

        // Scoring: more coverage is good, smaller distance is good
        // weight coverage higher
        (coverage * 10) - min_enemy_dist
    }

    fn find_best_placement(&self, map: &Map, piece: &Piece) -> Coord {
        let mut best = Coord { x: -1, y: -1 };
        let mut max_score = i32::MIN;

        for y in 0..=(map.height as isize - piece.height as isize) {
            for x in 0..=(map.width as isize - piece.width as isize) {
                if self.is_valid_placement(map, piece, x, y) {
                    let score = self.calculate_score(map, piece, x, y);
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
        // Add some debug output to stderr
        eprintln!("AI starting...");
        
        // Read player info
        let line = match self.read_line() {
            Some(value) => value,
            None => {
                return;
            }
        };
        eprintln!("Received: {}", line);
        self.parse_player_info(&line);
        eprintln!("I am player {}", self.player);

        loop {
            eprintln!("Waiting for map...");
            let map = self.parse_map();
            
            if map.width == 0 {
                eprintln!("Empty map received, ending.");
                println!("0 0");
                io::stdout().flush().unwrap();
                break;
            }
            
            eprintln!("Map parsed: {}x{}", map.width, map.height);
            
            eprintln!("Waiting for piece...");
            let piece = self.parse_piece();

            if piece.width == 0 {
                eprintln!("Empty piece received, ending.");
                println!("0 0");
                io::stdout().flush().unwrap();
                break;
            }
            
            eprintln!("Piece parsed: {}x{}", piece.width, piece.height);

            let best = self.find_best_placement(&map, &piece);

            if best.x == -1 || best.y == -1 {
                eprintln!("No valid placement found, sending 0 0");
                println!("0 0");
            } else {
                eprintln!("Found placement at ({}, {})", best.x, best.y);
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