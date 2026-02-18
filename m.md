./linux_game_engine -f maps/map00 -p1 solution/target/release/filler -p2 linux_robots/bender

cd /filler/solution
rm -rf target
cargo build --release

cd /filler
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler -p2 linux_robots/bender


✅ Rebuild & run inside container
docker run -it --rm -v "$(pwd)/solution":/filler/solution filler


Inside:
cd /filler/solution
cargo build --release
cd /filler
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler -p2 linux_robots/bender



need to confirm who is crashing (your bot or bender).
docker run -it --rm -v "$(pwd)/solution":/filler/solution filler


cd /filler/solution
cargo build --release
cd /filler
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 solution/target/release/filler

Run native M1 binaries on mac (no docker):
cd /Users/malakaljamri/Filler/docker_image
./m1_game_engine -f maps/map01 -p1 m1_robots/bender -p2 solution/target/release/filler



# from Filler/docker_image on mac host
docker build --platform linux/amd64 -t filler-amd64 .
docker run -it --platform linux/amd64 --rm \
           -v "$(pwd)/solution":/filler/solution \
           filler-amd64


cd /filler/solution
cargo build --release
BOT=/filler/solution/target/release/filler
cd /filler

# sanity check – two bundled robots
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator

# audit loops (5 games each, swapping sides)
run_series() {
  local map=$1 opp=$2 wins=0
  for i in {1..5}; do
    if (( i % 2 )); then
      ./linux_game_engine -q -f maps/$map -p1 "$BOT" -p2 linux_robots/$opp >out
    else
      ./linux_game_engine -q -f maps/$map -p1 linux_robots/$opp -p2 "$BOT" >out
    fi
    grep -q "Player.*surface.*filler" out && ((wins++))
  done
  echo "$opp on $map  →  $wins / 5"
}

run_series map00 wall_e
run_series map01 h2_d2
run_series map02 bender

# you’re already at the container prompt:  root@…:/filler

# 1. rebuild the bot (only once)
cd /filler/solution
cargo build --release
BOT=/filler/solution/target/release/filler
cd /filler          # <- stay here for every game

# 2. define a Bash function that runs 5 games
run_series () {
  local map=$1 opp=$2 wins=0
  for i in {1..5}; do
    if (( i % 2 )); then
      ./linux_game_engine -q -f maps/$map -p1 "$BOT"         -p2 linux_robots/$opp >out
    else
      ./linux_game_engine -q -f maps/$map -p1 linux_robots/$opp -p2 "$BOT"         >out
    fi
    grep -q "Player.*surface.*filler" out && ((wins++))
  done
  echo "$opp on $map  →  $wins / 5"
}

# 3. run the three required series
run_series map00 wall_e
run_series map01 h2_d2
run_series map02 bender
