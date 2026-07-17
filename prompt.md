# Target Device: SteelSeries Arena 7 Speakers (0x1038)
# Feature: Static and Prism Light Control
# Action: Implement color formatting to drive all 4 zones to Cyan (00FFFF)

## 1. Static Command Implementation (Subtype a1)
Configure the driver to build and send a 64-byte output transfer report to the speaker's control interface when setting static colors. 
For Cyan (00FFFF) at maximum brightness, compile the following exact 64-byte buffer payload:
- Header: 06 a1
- Zone Bytes (6 bytes each): 
  * Zone 1 (LT): 00 ff ff 01 1e 0a
  * Zone 2 (LB): 00 ff ff 01 1e 0a
  * Zone 3 (RT): 00 ff ff 01 1e 0a
  * Zone 4 (RB): 00 ff ff 01 1e 0a
- Terminal Byte: 0f (at byte index 26 / 0x1a)
- Trailing Zero Padding: Fill bytes 27 to 63 with 0x00.

Raw Hex Payload representation to reference/test:
06a100ffff011e0a00ffff011e0a00ffff011e0a00ffff011e0a0f00000000000000000000000000000000000000000000000000000000000000000000000000

## 2. Prism Sync Command Implementation (Subtype a7)
For synchronized real-time animation frames setting all 4 zones to Cyan (00FFFF), compile the following 64-byte payload structure:
- Header: 06 a7 0f
- Linear RGB Zone mapping (3 bytes each):
  * Zone 1 (LT): 00 ff ff
  * Zone 2 (LB): 00 ff ff
  * Zone 3 (RT): 00 ff ff
  * Zone 4 (RB): 00 ff ff
- Trailing Zero Padding: Fill bytes 15 to 63 with 0x00.

Raw Hex Payload representation to reference/test:
06a70f00ffff00ffff00ffff00ffff0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
