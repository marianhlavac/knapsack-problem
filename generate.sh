echo "Retrieve solutions..."
./solutions.sh > solutions.csv
echo "Generating, give it a while..."
cargo run --release data/knap_{4,10,15,20,22,25,27}.inst.dat > results.csv