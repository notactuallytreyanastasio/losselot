# Quick Start: Building MP3 Detective with Claude Code

## Step 1: Set Up

```bash
# Create a new directory and cd into it
mkdir mp3detective && cd mp3detective

# Initialize Go module
go mod init github.com/yourname/mp3detective
```

## Step 2: Give Claude Code This Prompt

Copy-paste this into Claude Code:

---

**Prompt:**

```
I need you to build a cross-platform MP3 transcode detector in Go. Read the full spec in README.md, but here's the TL;DR:

The tool detects MP3s that have been re-encoded from lower quality sources using:

1. **Binary analysis** (pure Go, no deps):
   - Parse MP3 frame headers to get declared bitrate
   - Extract LAME/Xing headers - the LAME header contains a "lowpass" field that reveals the ORIGINAL encoding quality
   - If a "320kbps" file has lowpass=16000Hz, it was transcoded from 128kbps
   - Detect multiple encoder signatures
   - Check frame size consistency

2. **Spectral analysis** (needs ffmpeg):
   - Decode to raw PCM via ffmpeg
   - FFT to measure energy in frequency bands (15-20kHz, 17-20kHz)
   - Transcodes have a "cliff" where high frequencies die; legit files have gradual rolloff

Start by implementing the MP3 frame parser and LAME header extractor - that's the core.
Then add the spectral analyzer.
Then wire up the CLI with cobra.
Finally, add HTML/CSV/JSON report generation.

The reference bash implementation is in reference_implementation.sh - match its detection logic.

Build targets needed: darwin-amd64, darwin-arm64, linux-amd64, windows-amd64
```

---

## Step 3: Key Things Claude Code Needs to Know

### MP3 Frame Header Format

The 4-byte header after sync word (0xFF 0xFB for MPEG1 Layer3):

```
Byte 0: 0xFF (sync)
Byte 1: 0xFB = 11111011
        ^^^^^ = sync bits (5 more)
             ^^ = MPEG version (11 = MPEG1)
               ^ = Layer (01 = Layer 3... wait, it's inverted: 01=L3, 10=L2, 11=L1)
Byte 2: EEEEFFGH
        EEEE = bitrate index (lookup table)
        FF = sample rate index
        G = padding
        H = private
Byte 3: Channel mode, etc
```

Bitrate lookup table for MPEG1 Layer 3:
```
Index:  0    1    2    3    4    5    6    7    8    9   10   11   12   13   14
kbps:  free  32   40   48   56   64   80   96  112  128  160  192  224  256  320
```

### LAME Header Location

1. Find "Xing" or "Info" tag in first frame (offset ~36 bytes into first frame for stereo)
2. After Xing header data, look for "LAME" string
3. LAME header structure (simplified):
   - Bytes 0-8: "LAMEx.xxx" version string
   - Byte 9: VBR method + lowpass (upper nibble = method, or it's more complex)
   - Actually: byte at offset 11 from "LAME" = lowpass/100

The exact offset varies by LAME version. Safest approach:
```go
lamePos := bytes.Index(data, []byte("LAME"))
if lamePos != -1 {
    // lowpass is typically at lamePos + 11, stored as Hz/100
    // So value 160 = 16000 Hz
    lowpassByte := data[lamePos+11]
    lowpassHz := int(lowpassByte) * 100
}
```

### FFT Band Energy Calculation

```go
// After FFT, you have complex bins
// Frequency of bin i = i * sampleRate / fftSize

// For 44100Hz sample rate, 8192 FFT size:
// Bin resolution = 44100/8192 ≈ 5.38 Hz per bin
// 15000 Hz = bin ~2790
// 20000 Hz = bin ~3720
// 17000 Hz = bin ~3162

// Energy = sum of |bin|² for all bins in range
// Convert to dB = 10 * log10(energy)
```

### Expected Lowpass Values by Bitrate

| Bitrate | Expected Lowpass | If Lower → Transcode |
|---------|------------------|----------------------|
| 320 kbps | 20500+ Hz | < 19000 Hz suspicious |
| 256 kbps | 20000 Hz | < 18500 Hz suspicious |
| 192 kbps | 18500 Hz | < 17500 Hz suspicious |
| 160 kbps | 17500 Hz | < 16500 Hz suspicious |
| 128 kbps | 16000 Hz | (this IS the source) |

## Step 4: Testing

Create test files:

```bash
# Make a "clean" 320kbps file
ffmpeg -i source.wav -b:a 320k -q:a 0 clean_320.mp3

# Make a transcode (128 → 320)
ffmpeg -i source.wav -b:a 128k intermediate.mp3
ffmpeg -i intermediate.mp3 -b:a 320k transcoded_320.mp3

# The tool should flag transcoded_320.mp3 but not clean_320.mp3
```

## Step 5: Building Releases

```bash
make all
# Creates:
# dist/mp3detective-darwin-amd64
# dist/mp3detective-darwin-arm64  
# dist/mp3detective-linux-amd64
# dist/mp3detective-windows-amd64.exe
```

---

## Troubleshooting

**"Can't find LAME header"**
- Not all MP3s have LAME headers (iTunes, Fraunhofer don't write them)
- Fall back to spectral analysis only

**"FFT results look wrong"**
- Make sure you're using a window function (Hanning)
- Check that ffmpeg is outputting float32 little-endian mono
- Verify sample rate matches what you're calculating bins for

**"False positives on legitimate files"**
- Some genres (electronic, heavily compressed pop) legitimately have less high-frequency content
- Consider adding a `--strict` mode vs default more lenient mode
