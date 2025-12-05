# MP3 Detective - Go Package

## Overview

This is a cross-platform MP3 transcode detection tool that combines spectral analysis and binary forensics to identify MP3 files that have been transcoded from lower-quality sources.

## Files Included

1. `mp3_detective.sh` - The reference bash implementation (requires ffmpeg + sox)
2. `CLAUDE_CODE_PROMPT.md` - Instructions for Claude Code to build the Go version

---

## CLAUDE_CODE_PROMPT.md

Use this prompt with Claude Code to generate the Go implementation:

---

# Task: Build a Cross-Platform MP3 Transcode Detector in Go

## What This Tool Does

Detects if MP3 files have been transcoded from lower bitrate sources using two complementary approaches:

### 1. Binary/Structural Analysis (no external deps)
- Parse MP3 frame headers directly (sync words 0xFFE/0xFFF)
- Extract and validate LAME/Xing/Info headers
- **Key check**: LAME header contains lowpass filter frequency - if a 320kbps file has lowpass=16000Hz, it's a transcode from 128kbps
- Detect multiple encoder signatures in the file
- Analyze frame size consistency (high variance in CBR = suspicious)
- Check ID3 tags for encoder mismatches

### 2. Spectral Analysis (requires ffmpeg)
- Decode MP3 to raw PCM using ffmpeg (shelled out)
- Perform FFT on the audio data
- Measure energy in frequency bands:
  - 10-15 kHz (mid-high)
  - 15-20 kHz (high)  
  - 17-20 kHz (upper - the dead giveaway zone)
- Compare energy dropoff between bands
- High bitrate files should have gradual rolloff; transcodes have a cliff

### Scoring Logic
- Binary analysis flags: `lowpass_mismatch`, `multi_encoder_sigs`, `irregular_frames`, `id3_encoder_mismatch`
- Spectral analysis flags: `steep_hf_rolloff`, `dead_upper_band`, `silent_17k+`, `early_cutoff`
- Combined score 0-100%
- Verdicts: OK (0-34%), SUSPECT (35-64%), TRANSCODE (65-100%)
- Bonus points when both analyses agree

## Project Structure

```
mp3detective/
├── cmd/
│   └── mp3detective/
│       └── main.go           # CLI entry point
├── pkg/
│   ├── analyzer/
│   │   ├── analyzer.go       # Main analysis orchestrator
│   │   ├── spectral.go       # FFT-based frequency analysis
│   │   └── binary.go         # MP3 structure parsing
│   ├── mp3parser/
│   │   ├── frame.go          # MP3 frame header parsing
│   │   ├── lame.go           # LAME/Xing header extraction
│   │   └── id3.go            # ID3 tag parsing
│   └── report/
│       ├── report.go         # Report generation interface
│       ├── html.go           # HTML report (embed the CSS)
│       ├── csv.go            # CSV output
│       └── json.go           # JSON output
├── internal/
│   └── fft/
│       └── fft.go            # Pure Go FFT implementation (or use go-dsp)
├── go.mod
├── go.sum
├── Makefile                  # Cross-compilation targets
└── README.md
```

## Key Implementation Details

### MP3 Frame Parsing (pkg/mp3parser/frame.go)

MP3 frames start with sync word. Parse the 4-byte header:

```go
// Frame header structure (4 bytes)
// AAAAAAAA AAABBCCD EEEEFFGH IIJJKLMM
// A = sync (11 bits, all 1s)
// B = MPEG version (2 bits): 00=2.5, 01=reserved, 10=2, 11=1
// C = Layer (2 bits): 00=reserved, 01=III, 10=II, 11=I
// D = Protection bit
// E = Bitrate index (4 bits) - lookup table
// F = Sample rate index (2 bits) - lookup table
// G = Padding bit
// H = Private bit
// I = Channel mode
// J = Mode extension
// K = Copyright
// L = Original
// M = Emphasis

type FrameHeader struct {
    Version      int  // 1, 2, or 2.5
    Layer        int  // 1, 2, or 3
    Bitrate      int  // kbps
    SampleRate   int  // Hz
    Padding      bool
    ChannelMode  int
    FrameSize    int  // calculated
}

func ParseFrameHeader(data []byte) (*FrameHeader, error)
```

### LAME Header Extraction (pkg/mp3parser/lame.go)

LAME writes a VBR header in the first frame. Key fields:

```go
type LAMEHeader struct {
    Encoder     string  // "LAME3.100" etc
    VBRMethod   int     // 0=CBR, 1-5=VBR methods
    Lowpass     int     // Lowpass filter frequency (Hz) - THIS IS THE KEY
    EncoderDelay int
    Padding     int
    // ... more fields
}

// LAME header is at offset 0x24 (36 bytes) into Xing frame for stereo
// or 0x15 (21 bytes) for mono
// Look for "LAME" string, then parse subsequent bytes
```

The lowpass field is at byte offset 9 from "LAME" string, stored as value/100.
So 0xA0 (160) = 16000 Hz lowpass.

**Critical detection**: If file claims 320kbps but LAME lowpass is 16000Hz, it's 99% a transcode from 128kbps.

### Spectral Analysis (pkg/analyzer/spectral.go)

```go
// Shell out to ffmpeg for decoding (most reliable cross-platform)
func decodeToRaw(mp3Path string) ([]float64, int, error) {
    // ffmpeg -i input.mp3 -f f32le -acodec pcm_f32le -ac 1 -ar 44100 -
    // Returns mono float32 samples at 44100Hz
}

// Perform FFT and measure energy in bands
func analyzeSpectrum(samples []float64, sampleRate int) *SpectralResult {
    // Use overlapping windows (Hanning) for better frequency resolution
    // FFT size: 8192 or 16384 for good low-frequency resolution
    // 
    // Frequency bin = (bin_index * sample_rate) / fft_size
    // 
    // Sum energy (magnitude squared) in each band:
    // - midHigh: 10000-15000 Hz
    // - high: 15000-20000 Hz  
    // - upper: 17000-20000 Hz
    //
    // Convert to dB: 10 * log10(energy)
    // Compare dropoffs between bands
}
```

### CLI Interface (cmd/mp3detective/main.go)

```
mp3detective [flags] <path>

Flags:
  -o, --output FILE    Output report file (.html, .csv, .json)
  -j, --jobs INT       Parallel workers (default: NumCPU)
  -v, --verbose        Show detailed analysis
  -q, --quiet          Only show summary
  --no-spectral        Skip spectral analysis (faster, binary-only)
  --threshold INT      Transcode threshold percentage (default: 65)
  -h, --help           Show help

Examples:
  mp3detective ~/Music/
  mp3detective -o report.html -j 8 ~/Music/
  mp3detective --no-spectral suspicious_file.mp3
```

### Cross-Compilation (Makefile)

```makefile
BINARY=mp3detective
VERSION=$(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
LDFLAGS=-ldflags "-X main.Version=$(VERSION) -s -w"

.PHONY: all clean darwin-amd64 darwin-arm64 linux-amd64 windows-amd64

all: darwin-amd64 darwin-arm64 linux-amd64 windows-amd64

darwin-amd64:
	GOOS=darwin GOARCH=amd64 go build $(LDFLAGS) -o dist/$(BINARY)-darwin-amd64 ./cmd/mp3detective

darwin-arm64:
	GOOS=darwin GOARCH=arm64 go build $(LDFLAGS) -o dist/$(BINARY)-darwin-arm64 ./cmd/mp3detective

linux-amd64:
	GOOS=linux GOARCH=amd64 go build $(LDFLAGS) -o dist/$(BINARY)-linux-amd64 ./cmd/mp3detective

windows-amd64:
	GOOS=windows GOARCH=amd64 go build $(LDFLAGS) -o dist/$(BINARY)-windows-amd64.exe ./cmd/mp3detective

clean:
	rm -rf dist/
```

## Dependencies

Minimize external deps. Suggested:

```go
require (
    github.com/spf13/cobra v1.8.0        // CLI framework (optional, can use flag)
    github.com/mjibson/go-dsp v0.0.0     // FFT implementation (or write pure Go)
    github.com/schollz/progressbar/v3    // Progress bar (optional)
)
```

Or go fully stdlib - the FFT can be implemented in ~100 lines.

## HTML Report Template

Embed this CSS/HTML template in the binary using `//go:embed`. The reference bash script has the full HTML template with:
- Dark mode UI
- Color-coded verdict badges
- Score progress bars
- Sortable table
- Flag reference legend

## Testing Strategy

1. Create test fixtures:
   - Legitimate high-bitrate MP3 (encode from WAV at 320kbps)
   - Obvious transcode (decode that 320, re-encode at 320)
   - Upconvert (128kbps source → 320kbps)
   
2. Unit tests for frame parser, LAME header extraction
3. Integration tests comparing output to reference bash implementation

## Notes for Implementation

1. **Binary analysis works without ffmpeg** - useful for quick scans or systems without ffmpeg
2. **Spectral analysis is more accurate** but needs ffmpeg installed
3. **The `--no-spectral` flag** lets users choose speed vs accuracy
4. **Lowpass mismatch is the smoking gun** - if you find this, it's definitely a transcode
5. **Multiple encoder signatures** are suspicious but not definitive (could be re-tagged)
6. **Frame size variance** is a weak signal, weight it lower

## Deliverables

1. Working Go binary that matches the bash script's detection logic
2. Cross-platform builds (darwin-amd64, darwin-arm64, linux-amd64, windows-amd64)
3. HTML/CSV/JSON report generation
4. README with installation and usage instructions
5. MIT license

---

That's the full spec. Start with the MP3 parser since that's the foundation, then build up to the analyzer, then CLI, then reports.
