# Filler AI (Rust)

This directory contains a self-contained **Rust implementation** of a player (robot) for the 42 `filler` game.

## Build

```bash
# host build (native arch)
cargo build --release
```

**Docker / x86-64**
```bash
# at project root (docker_image)
docker build --platform linux/amd64 -t filler-amd64 .
docker run -it --platform linux/amd64 --rm \
           -v "$(pwd)/solution":/filler/solution \
           filler-amd64
# inside the container
cd /filler/solution && cargo build --release
```

The compiled bot is at `target/release/filler`.

## Run vs bundled opponents

```bash
cd /filler
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 solution/target/release/filler
```

For Apple Silicon (ARM) without Docker use the native engine & robots:

```bash
./m1_game_engine -f maps/map01 -p1 m1_robots/bender -p2 solution/target/release/filler
```

## Algorithm

* **Parsing** – reads the *Anfield* grid and the incoming *Piece* exactly as the game-engine sends them.
* **Placement validator** – ensures:
  * Piece stays in bounds.
  * Exactly **one** cell overlaps existing own territory.
  * No cell overlaps enemy territory.
* **Heuristic search** – brute-force all legal placements, score each, choose max.

### Scoring function
```text
score = (new_cells * 10) - distance_to_enemy_front
```
* `new_cells` – how many `O` cells the piece contributes (encourages large coverage).
* `distance_to_enemy_front` – Manhattan distance from any placed cell to the *closest* enemy cell. Favouring smaller distances pushes the bot toward the opponent.
* When no enemy cells exist yet, distance is measured to the board centre—helps early expansion.

### Why it beats the baseline robots
* **Aggressive expansion** keeps the bot from walling itself off (common with naïve centre-only strategies).
* **Coverage weight (×10)** ensures larger pieces are preferred when scores tie, maximising surface quickly.
* **Input robustness** – tolerates blank lines and optional headers so it never times-out.

## Limitations & possible improvements
* Still deterministic – a clever opponent can predict its path.
* No full look-ahead; adding one-ply minimax could improve decision quality.
* No edge penalty yet; introducing it might further reduce early self-blocking.

---
© 2026 Your Name – MIT License
