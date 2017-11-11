echo "Retrieve solutions..."
./solutions.sh data/knap*sol.dat > solutions.csv
echo "Generating, give it a while..."
cargo run --release data/knap_{4,10,15,20,22,25,27,30,32}.inst.dat > results.csv