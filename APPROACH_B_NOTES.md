# Approach B: Cross-Frequency Coherence Analysis

## Competition Goal
Distinguish natural lo-fi/tape rolloff from lossy transcoding brick-walls.

## Challenger's Approach (Approach A)
- Spectral gradient/derivative analysis
- Transition width measurement (Hz to drop from -10dB to -40dB)
- Temporal variance of cutoff frequency
- Shelf vs slope detection
- Pre-echo artifact detection

## My Approach: Different Mathematical Foundation

### Core Insight
The challenger measures the **shape of the rolloff curve**. I'll measure the **statistical relationships between frequency bands**.

In natural audio, adjacent frequency bands are **correlated** - energy at 18kHz predicts energy at 19kHz. This correlation gradually weakens at higher frequencies (natural decorrelation).

In MP3 brick-walls, this correlation **abruptly drops to zero** at the cutoff - there's energy at 19kHz but completely uncorrelated silence at 20kHz.

### Primary Metrics

#### 1. Cross-Frequency Correlation Coefficient (CFCC)
```
For each frequency band pair (f1, f2) where f2 = f1 + Δf:
  CFCC(f1, f2) = pearson_correlation(energy_over_time[f1], energy_over_time[f2])

Natural audio: CFCC gradually decreases from ~0.9 to ~0.5 as f increases
MP3 brick-wall: CFCC suddenly drops from ~0.8 to ~0.0 at cutoff
```

The "CFCC cliff" is a fingerprint of lossy encoding.

#### 2. Decorrelation Rate
```
d(CFCC)/d(frequency) across the HF region

Natural: constant, small negative slope (gradual decorrelation)
MP3: spike in negative slope at cutoff (sudden decorrelation)
```

#### 3. Spectral Kurtosis Profile
```
Kurtosis = measure of "tailedness" of distribution

In each frequency band, measure kurtosis of magnitude distribution over time:
- Natural audio: consistent kurtosis across bands (real signal characteristics)
- MP3 cutoff region: anomalous kurtosis (quasi-random noise floor vs signal)
```

#### 4. Energy Ratio Stability
```
For adjacent bands: ratio = energy[f+Δf] / energy[f]

In natural rolloff: ratio is relatively stable (e.g., 0.85, 0.82, 0.80, 0.78...)
In brick-wall: ratio suddenly drops (e.g., 0.85, 0.82, 0.03, 0.01...)

Measure: variance of ratio differences
- Low variance = natural rolloff
- High variance with outlier = brick-wall
```

### Why This Approach Might Win

1. **Information-theoretic foundation** - Measuring statistical relationships, not just amplitude curves
2. **Robust to post-processing** - EQ/limiting affects amplitude but not correlation structure
3. **Sensitive to MP3's block-based encoding** - Block artifacts disrupt natural correlation patterns
4. **Different failure modes** - Where gradient analysis might fail, coherence might succeed (and vice versa)

### Potential Weaknesses

1. **More computationally expensive** - Need to compute correlations across many band pairs
2. **Needs careful band selection** - What frequency resolution for bands?
3. **May need more data** - Correlation requires sufficient samples for stability
4. **Could be fooled by certain content** - Very sparse HF content might look uncorrelated naturally

### Implementation Plan

1. **Add band correlation analysis to spectral.rs**
   - Compute energy time-series for overlapping narrow bands (100Hz wide?) from 15kHz to 22kHz
   - Calculate pearson correlation between adjacent bands
   - Find the "correlation cliff" point if it exists

2. **Add spectral kurtosis measurement**
   - For each band, collect magnitude samples across all FFT windows
   - Compute kurtosis of the distribution
   - Flag anomalous kurtosis patterns

3. **Add energy ratio stability metric**
   - Compute band energy ratios
   - Measure variance/outliers in the ratio series

4. **Scoring integration**
   - CFCC cliff detected: strong indicator of brick-wall
   - High decorrelation rate spike: moderate indicator
   - Anomalous kurtosis profile: supporting evidence
   - Unstable energy ratios: supporting evidence

5. **Lo-fi safe passage**
   - If CFCC shows gradual decorrelation (no cliff) → likely natural
   - If kurtosis is consistent across bands → likely natural
   - If energy ratios are stable → likely natural

### Test Cases to Validate

1. **True 320kbps transcode** - Should show CFCC cliff at ~20kHz
2. **True 128kbps transcode** - Should show CFCC cliff at ~16kHz
3. **Cassette transfer** - Should show gradual CFCC decline, no cliff
4. **Vintage master** - Should show gradual CFCC decline
5. **Lo-fi production** - Should show gradual CFCC decline
6. **True lossless** - Should show gradual CFCC decline to 22kHz

---

## Session Log

### Session 1: Initial Research

Read the existing `spectral.rs` code. Current approach:
- Uses FFT with 8192 samples, Hanning window
- Measures energy in bands: 10-15kHz, 15-20kHz, 17-20kHz, 19-20kHz, 20-22kHz
- Computes "drops" between bands (dB difference)
- Flags based on thresholds (>40dB drop = severe damage, etc.)
- Also measures spectral flatness in 19-21kHz range

Key limitation: The current code measures **amplitude differences** but not **statistical relationships**. A tape recording with gradual rolloff from -20dB at 15kHz to -60dB at 20kHz would trigger the same flags as an MP3 with a brick-wall at 18kHz.

My CFCC approach would distinguish these:
- Tape: correlation between 15kHz and 20kHz might be ~0.4 (lower due to rolloff, but still present)
- MP3: correlation between 18kHz and 20kHz would be ~0.0 (nothing above cutoff)

### Next Steps
1. Prototype the CFCC calculation in Rust
2. Test on known good/bad samples
3. Tune parameters (band width, frequency range, etc.)
4. Integrate into scoring
