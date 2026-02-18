1. Build the bot:
cd docker_image/solution
cargo build --release
# binary: target/release/filler

2. Run a game on mac (M1) without Docker:
cd docker_image
./m1_game_engine -f maps/map01 \
                 -p1 m1_robots/bender \
                 -p2 solution/target/release/filler



3.Run inside the project Docker image (x86-64):
# build x86 image once
cd docker_image
docker build --platform linux/amd64 -t filler-amd64 .

# start a disposable container and mount the solution folder
docker run -it --platform linux/amd64 --rm \
           -v "$(pwd)/solution":/filler/solution \
           filler-amd64        


Inside the container:                    
cd /filler/solution
cargo build --release            # builds Linux binary
BOT=/filler/solution/target/release/filler
cd /filler

# play a match
./linux_game_engine -f maps/map01 \
                    -p1 linux_robots/bender \
                    -p2 "$BOT"


4. Run the 5-game audit loops (still inside the container):                    

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

root@3e364faeccc7:/filler# run_series map02 bender
Error: Timeout for player1
Error: Timeout for player2

./linux_game_engine -t 60 -f maps/map02 \
                    -p1 "$BOT" \
                    -p2 linux_robots/bender



docker run -v "$(pwd)/solution":/filler/solution -it filler 

./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 solution/target/release/filler 


enter the container and build the boot in linux

docker run --platform linux/amd64 -v "$(pwd)/solution":/filler/solution -it filler-amd64
# inside the container
cd /filler/solution
cargo build --release          # produces target/release/filler (ELF x86-64)


///////////////

go inside the cd docker_image

docker run --platform linux/amd64 -v "$(pwd)/solution":/filler/solution -it filler-amd64
# inside the container
cd /filler/solution
cargo build --release          # produces target/release/filler (ELF x86-64)

# then go back to the /filler

./linux_game_engine -f maps/map01 \
    -p1 linux_robots/bender \
    -p2 solution/target/release/filler


terminator


h2_d2 --> map02  
seed: 1770500183141407464
Error: Timeout for player2


terminator -->map02
player 1 won

wall_e -
Error: Timeout for player2
i lost 




