#!/bin/bash
# Test RGB functionality

echo "=== Testing RGB Functionality ==="
echo ""

echo "1. Setting color to RED"
cargo run --quiet -- rgb color red
sleep 2

echo ""
echo "2. Setting color to BLUE"
cargo run --quiet -- rgb color blue
sleep 2

echo ""
echo "3. Setting color to GREEN"
cargo run --quiet -- rgb color green
sleep 2

echo ""
echo "4. Setting brightness to 50%"
cargo run --quiet -- rgb brightness 50
sleep 2

echo ""
echo "5. Setting brightness to 100%"
cargo run --quiet -- rgb brightness 100
sleep 2

echo ""
echo "6. Turning OFF LEDs"
cargo run --quiet -- rgb off
sleep 2

echo ""
echo "7. Setting color to WHITE (verify it turns back on)"
cargo run --quiet -- rgb color white
sleep 2

echo ""
echo "=== RGB Test Complete ==="
