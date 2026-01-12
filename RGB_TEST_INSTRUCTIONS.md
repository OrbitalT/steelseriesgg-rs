# RGB Physical Testing Instructions

## Quick Start

To verify that RGB commands actually control your keyboard LEDs, run:

```bash
./verify_rgb_physical.sh
```

This interactive script will:
1. Send RGB commands to your keyboard
2. Ask you to visually confirm LED changes
3. Test all colors, brightness levels, and the "off" command
4. Report whether physical verification passed

## What This Tests

- ✅ **RGB Off**: Confirms LEDs actually turn off (dark)
- ✅ **Colors**: Tests red, green, blue, purple, white
- ✅ **Brightness**: Tests dimming (25%) and full brightness (100%)
- ✅ **On/Off Cycle**: Ensures LEDs turn back on after being off

## Expected Results

If everything works correctly, you should see:

1. **Red Color Test**: All keyboard LEDs turn red
2. **Green Color Test**: All keyboard LEDs turn green
3. **Blue Color Test**: All keyboard LEDs turn blue
4. **Purple Color Test**: All keyboard LEDs turn purple/magenta
5. **White Color Test**: All keyboard LEDs turn white
6. **Dim Test**: LEDs become noticeably dimmer (about 25% brightness)
7. **Bright Test**: LEDs return to full brightness
8. **Off Test**: LEDs turn completely off (black/dark)
9. **On Test**: LEDs turn back on (white)

## During the Test

For each test, the script will:
1. Execute an RGB command
2. Ask: "Do you see [expected result]? (y/n)"
3. You press 'y' if you see the change, 'n' if you don't
4. Script continues to next test

## If a Test Fails

If you answer 'n' to any question, the script will report which test failed. This helps identify specific issues like:
- Colors not changing
- "Off" not working
- Brightness not adjusting

## Troubleshooting

### LEDs Don't Respond At All

Check permissions:
```bash
ls -l /dev/hidraw*
```

Add udev rule if needed:
```bash
echo 'SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1038", MODE="0666"' | sudo tee /etc/udev/rules.d/99-steelseries.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### LEDs Change But Wrong Colors

This might indicate:
- Color encoding issue (report this as a bug)
- Device-specific protocol variation

### "Off" Doesn't Turn LEDs Off

This might indicate:
- Device needs special "off" command
- Brightness is very low but not zero

### Colors Work But Brightness Doesn't

This might indicate:
- Brightness command not supported by device
- Separate brightness interface needed

## Manual Testing Alternative

If you prefer manual testing:

```bash
# Test RGB off
./target/release/ssgg rgb off
# Observe: Are LEDs off?

# Test red
./target/release/ssgg rgb color red
# Observe: Are LEDs red?

# Test brightness
./target/release/ssgg rgb brightness 25
# Observe: Are LEDs dim?

./target/release/ssgg rgb brightness 100
# Observe: Are LEDs bright again?
```

## Reporting Results

After testing, please report:

### If All Tests Pass ✅
The RGB functionality is fully working! No further action needed.

### If Any Tests Fail ❌
Create an issue with:
- Which tests failed
- Device model (e.g., "Apex Pro TKL 2023")
- What you observed vs. what was expected
- Output of `./target/release/ssgg devices`

## Technical Background

See these documents for details:
- `RGB_VERIFICATION.md` - Protocol verification details
- `PHYSICAL_VERIFICATION_NEEDED.md` - Complete testing guide
- `RALPH_ITERATION_1_SUMMARY.md` - Protocol verification results
- `RALPH_ITERATION_2_SUMMARY.md` - Physical verification setup

---

**Ready to test? Run:** `./verify_rgb_physical.sh`
