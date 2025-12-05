# Lo-Fi / Natural Rolloff Detection - Research Notes

## Problem Statement

Losselot currently flags lo-fi sources (cassette tapes, vintage masters, noisy productions) as transcodes because they have high-frequency rolloff. But the *character* of the rolloff is different:

- **MP3/lossy**: Hard brick-wall cutoff at specific frequencies
- **Analog/tape**: Soft, gradual rolloff that varies with content

We need to distinguish between these two cases to reduce false positives.

---

## Key Observations

### From Sound Engineer Feedback (Dec 2024)

1. Charlie Miller Grateful Dead cassette transfer flagged as "Suspect, 108kbps"
2. Known lossless master flagged at "121kbps"
3. Cassette spectral analysis shows cutoff ~16kHz but "soft and wavey, not a hard cutoff"
4. Lo-fi/noisy production (Bomb the Music Industry - laptop punk) may have mixed-source assets

### The Core Distinction

| Characteristic | MP3 Transcode | Natural Lo-Fi |
|----------------|---------------|---------------|
| Cutoff sharpness | Very sharp (brick-wall) | Gradual slope |
| Cutoff frequency | Fixed across entire file | Varies with dynamics |
| Matches codec freq? | Yes (16k, 18k, 19k, 20k) | Usually not |
| Shape before cutoff | Flat shelf | Continuous slope |
| Pre-echo artifacts | Often present | Absent |
| Temporal consistency | Identical every frame | Breathes with music |

### Visual Representation

```
MP3 (brick-wall):
Magnitude
    │██████████████████████▄░░░░░░░░
    │██████████████████████▄░░░░░░░░
    │██████████████████████▄░░░░░░░░
    └─────────────────────────────── Frequency
                           ^ fixed cutoff

Tape (gradual rolloff):
Magnitude
    │████████████████▇▆▅▄▃▂▁░░░░░░░
    │██████████████▇▆▅▄▃▂▁░░░░░░░░░  <- varies!
    │████████████████████▇▆▅▄▃▂▁░░░
    └─────────────────────────────── Frequency
                      ^ cutoff moves with content
```

---

## Proposed Detection Metrics

### 1. Transition Width (Basic)
```
Find frequency where energy drops to -3dB from HF peak
Find frequency where energy drops to -40dB
transition_width = freq_at_-40dB - freq_at_-3dB

Brick-wall: 50-300 Hz
Gradual: 2000-5000+ Hz
```

**Limitations**: Too simple, doesn't catch edge cases.

### 2. Slope Fitting (Better)
```
In the 12-20kHz region, fit a linear regression to magnitude vs frequency
slope = dB per kHz

Brick-wall: slope approaches infinity at cutoff, flat before
Gradual: consistent negative slope (e.g., -3 to -10 dB/kHz)
```

### 3. Temporal Cutoff Variance (Key Insight!)
```
For each time window (e.g., 100ms chunks):
  - Find the -20dB point (relative to that window's peak)
  - Record the frequency

Calculate standard deviation of cutoff frequencies across all windows

MP3: std_dev ≈ 0-200 Hz (cutoff is fixed by encoder)
Tape: std_dev ≈ 500-2000+ Hz (cutoff varies with dynamics)
```

**This is the strongest signal** - MP3 encoders apply the same filter everywhere, but analog rolloff is content-dependent.

### 4. Shelf Detection
```
Measure energy variance in the 8-14kHz region (before typical cutoffs)

Flat shelf (MP3): low variance, consistent energy
Sloped (tape): higher variance, already declining
```

### 5. Known Frequency Matching
```
Common lossy cutoff frequencies:
- 128 kbps MP3: ~16 kHz
- 192 kbps MP3: ~18 kHz
- 256 kbps MP3: ~19 kHz
- 320 kbps MP3: ~20 kHz
- 128 kbps AAC: ~15 kHz
- 256 kbps AAC: ~18 kHz

If detected cutoff is within ±500Hz of these, increase suspicion
If cutoff is at an "odd" frequency (14.3kHz, 17.2kHz), less suspicious
```

### 6. Pre-Echo Detection (Future)
MP3 encoding causes temporal smearing - energy appears before transients.
This is independent of frequency cutoff and could catch transcodes that have been post-processed.

---

## Proposed Algorithm

```
function analyze_rolloff(audio):
    # Step 1: Basic cutoff detection
    cutoff_freq = find_primary_cutoff(audio)

    # Step 2: Measure transition sharpness
    transition_width = measure_transition_width(audio, cutoff_freq)
    sharpness_score = map_to_score(transition_width)  # 0-100

    # Step 3: Temporal variance analysis (THE KEY)
    cutoff_variance = measure_cutoff_variance_over_time(audio)
    variance_score = map_variance_to_score(cutoff_variance)  # 0-100

    # Step 4: Shelf detection
    shelf_flatness = measure_pre_cutoff_flatness(audio)
    shelf_score = map_flatness_to_score(shelf_flatness)  # 0-100

    # Step 5: Known frequency matching
    codec_match_score = check_known_frequencies(cutoff_freq)  # 0-100

    # Combine scores
    # Temporal variance is weighted highest - it's the strongest signal
    lossy_likelihood = (
        sharpness_score * 0.20 +
        variance_score * 0.40 +      # Highest weight!
        shelf_score * 0.15 +
        codec_match_score * 0.25
    )

    # Confidence based on signal strength
    if cutoff_variance > 1000:  # Very high variance
        return ("LIKELY_NATURAL", high_confidence)
    elif cutoff_variance < 100 and transition_width < 200:
        return ("LIKELY_LOSSY", high_confidence)
    else:
        return (verdict_from_score(lossy_likelihood), medium_confidence)
```

---

## Implementation Plan

### Phase 1: Temporal Cutoff Variance (Start Here)
- [ ] Implement windowed analysis (100ms windows)
- [ ] Find -20dB point in each window
- [ ] Calculate std dev across windows
- [ ] Test on known cassette transfers vs known transcodes

### Phase 2: Transition Width Refinement
- [ ] Implement proper slope fitting
- [ ] Measure dB/kHz in the rolloff region
- [ ] Distinguish shelf+cliff from continuous slope

### Phase 3: Integration
- [ ] Add new metrics to SpectralDetails struct
- [ ] Update scoring algorithm
- [ ] Add "natural_rolloff_detected" flag
- [ ] Update UI to show rolloff analysis

### Phase 4: Validation
- [ ] Test against cassette transfers (should pass)
- [ ] Test against known transcodes (should fail)
- [ ] Test against vintage masters
- [ ] Test against lo-fi productions

---

## Test Cases Needed

1. **True positives** (should detect as transcode):
   - 128k MP3 → FLAC
   - 192k MP3 → FLAC
   - 320k MP3 → FLAC
   - AAC → FLAC

2. **True negatives** (should NOT flag):
   - Cassette transfers (Charlie Miller GD tapes)
   - Vinyl rips
   - Vintage masters (1960s-80s recordings)
   - Lo-fi productions
   - AM radio recordings

3. **Edge cases**:
   - MP3 that's been EQ'd (softened cutoff)
   - Re-encoded multiple times (blurred cutoff)
   - Lossless with aggressive lowpass filter
   - Mixed-source productions (some MP3 assets in lossless project)

---

## Open Questions

1. What window size is optimal for temporal analysis? (100ms? 500ms?)
2. How do we handle very short files where variance is hard to measure?
3. Should we weight recent audio differently (intro vs body of track)?
4. How do we handle intentional brick-wall filters in mastering?
5. Can we detect pre-echo reliably? Would it help?

---

## Current Implementation Analysis

After reviewing `src/analyzer/spectral.rs`:

### What We Already Have
- FFT_SIZE = 8192 samples (~186ms windows at 44.1kHz)
- 50% overlap (hop_size = 4096)
- Frequency resolution: 44100/8192 ≈ 5.38 Hz per bin
- Band energy measurements: 10-15kHz, 15-20kHz, 17-20kHz, 19-20kHz, 20-22kHz
- Spectrogram data already stored per-window
- `upper_drop` and `ultrasonic_drop` metrics

### What We Need to Add

1. **Per-window cutoff frequency detection**
   - In each FFT window, find the frequency where energy drops below threshold
   - Store this cutoff frequency for each window
   - Already iterating windows, just need to add detection

2. **Cutoff variance calculation**
   - After processing all windows, calculate std dev of cutoff frequencies
   - Add to `SpectralDetails` struct

3. **Slope measurement**
   - In each window, measure the gradient (dB/kHz) in the 12-20kHz region
   - Average across windows

4. **Shelf detection**
   - Measure energy variance in 8-14kHz band
   - Low variance = flat shelf (suspicious)
   - High variance = natural slope (less suspicious)

### Implementation Approach

```rust
// Add to SpectralDetails struct:
pub struct SpectralDetails {
    // ... existing fields ...

    /// Cutoff frequency variance across time windows (Hz)
    /// Low variance (<200) = fixed cutoff = likely lossy
    /// High variance (>500) = varying cutoff = likely natural
    pub cutoff_variance: f64,

    /// Average detected cutoff frequency (Hz)
    pub avg_cutoff_freq: f64,

    /// Rolloff slope in the 12-20kHz region (dB/kHz)
    /// Steep negative slope with cliff = lossy
    /// Gradual negative slope = natural
    pub rolloff_slope: f64,

    /// Pre-cutoff flatness (8-14kHz energy variance)
    /// Low = flat shelf before cutoff (suspicious)
    /// High = already rolling off (natural)
    pub shelf_flatness: f64,
}
```

### Cutoff Detection Algorithm

For each FFT window:
```
1. Find peak energy in 5-15kHz range (reference level)
2. Scan upward from 15kHz
3. Find first frequency where energy drops 20dB below reference
4. Record this as the cutoff frequency for this window
```

This captures where the "cliff" or "slope" begins for each moment in time.

---

## References

- MP3 psychoacoustic model and typical cutoff frequencies
- Tape frequency response characteristics
- Spectral analysis techniques for audio forensics

---

## Change Log

- 2024-12-05: Initial research notes from sound engineer feedback
- 2024-12-05: Proposed temporal variance as key metric
- 2024-12-05: Reviewed spectral.rs, documented implementation approach
