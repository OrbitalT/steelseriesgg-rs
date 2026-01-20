#!/bin/bash
# Test if device responds to commands by checking for any HID responses

echo "Testing device responses..."
echo ""

# Try to read from the device after sending commands
echo "1. Sending RGB off command..."
./target/release/ssgg rgb off

echo ""
echo "2. Attempting to read device state from hidraw..."
# Try reading from the HID device
for dev in /dev/hidraw*; do
    if sudo timeout 0.1 cat "$dev" 2>/dev/null | xxd -l 32 2>/dev/null; then
        echo "Got response from $dev"
    fi
done

echo ""
echo "3. Sending RGB red command..."
./target/release/ssgg rgb color red

echo ""
echo "4. Checking device state again..."
for dev in /dev/hidraw*; do
    if sudo timeout 0.1 cat "$dev" 2>/dev/null | xxd -l 32 2>/dev/null; then
        echo "Got response from $dev"
    fi
done

echo ""
echo "Test complete. If device is responding, you should see hex data above."
