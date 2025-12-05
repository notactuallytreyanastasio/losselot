# Losselot Detection Improvement Plan

## Overview

This document outlines a systematic, scientific approach to improving lo-fi/transcode detection using schema versioning, SQLite-backed analysis tracking, and web-based analysis comparison tools.

---

## Background

### Current State (Schema v1.1.0 "lofi-detection")

The current implementation uses Approach A (Gradient/Temporal Variance):
- `cutoff_variance`: Temporal variance of detected cutoff frequency across windows
- `rolloff_slope`: dB/kHz gradient in the 12-20kHz region
- `transition_width`: Hz from -3dB to -40dB points
- `natural_rolloff`: Boolean flag for gradual rolloff detection

### Problem Statement

Both approaches aim to distinguish:
| Source | Characteristic |
|--------|---------------|
| MP3 transcode | Brick-wall cutoff at codec-specific frequency |
| Cassette/tape | Soft, varying rolloff with dynamics |
| Vintage masters | Limited bandwidth but natural decay |

Sound engineer feedback indicates false positives on legitimate lo-fi sources.

---

## Two Competing Approaches

### Approach A: Gradient/Temporal Variance Analysis
**Currently implemented in v1.1.0**

| Metric | What It Measures | Signal |
|--------|------------------|--------|
| `cutoff_variance` | Std dev of cutoff frequency across time | Low = fixed (lossy), High = varying (natural) |
| `rolloff_slope` | dB/kHz in rolloff region | Steep = cliff, Gradual = natural |
| `transition_width` | Hz span of rolloff transition | Narrow = brick-wall, Wide = gradual |

**Strengths:**
- Conceptually simple
- Computationally efficient
- Temporal variance is a strong signal

**Weaknesses:**
- Can be fooled by post-processing that smooths cutoff
- Requires tuning thresholds

### Approach B: Cross-Frequency Correlation Coefficient (CFCC)
**Proposed for v1.2.0 or v2.0.0**

| Metric | What It Measures | Signal |
|--------|------------------|--------|
| CFCC profile | Correlation between adjacent frequency bands over time | Sudden decorrelation = brick-wall |
| `cliff_frequency` | Location of correlation drop | Match with known codec frequencies |
| `cliff_magnitude` | Severity of correlation drop | >0.5 drop = strong evidence |
| Decorrelation rate | d(CFCC)/d(freq) | Spike = artificial cutoff |

**Strengths:**
- Information-theoretic measure
- Cannot be fooled by EQ/smoothing (can't create correlation where none exists)
- More robust to post-processing

**Weaknesses:**
- Higher computational cost
- Requires storing all FFT windows (memory)
- More complex implementation

---

## Scientific Comparison Methodology

### Phase 1: Establish Baseline (v1.1.0)

1. **Run current algorithm on test corpus**
   - All files in `examples/demo_files/`
   - Store results in SQLite with schema version

2. **Document expected outcomes**
   ```
   TRUE_POSITIVE:  Files known to be transcodes → flagged as TRANSCODE
   TRUE_NEGATIVE:  Files known to be genuine → flagged as OK
   FALSE_POSITIVE: Genuine files flagged as TRANSCODE (Type I error)
   FALSE_NEGATIVE: Transcodes flagged as OK (Type II error)
   ```

3. **Generate confusion matrix**
   - Query SQLite for v1.1.0 results
   - Compare verdicts against ground truth labels
   - Calculate precision, recall, F1 score

### Phase 2: Implement CFCC (v1.2.0 or v2.0.0)

1. **Add CFCC metrics to SpectralDetails**
   ```rust
   // Schema v1.2.0 new fields
   pub cfcc_cliff_frequency: Option<u32>,
   pub cfcc_cliff_magnitude: Option<f64>,
   pub cfcc_pattern_type: Option<String>, // "lossy", "natural", "unknown"
   pub cfcc_decorrelation_rate: Option<f64>,
   ```

2. **Update schema version**
   ```rust
   pub const CURRENT_SCHEMA: AnalysisSchema = AnalysisSchema {
       major: 1,
       minor: 2,
       patch: 0,
       name: "cfcc-detection",
       features: &[
           // existing...
           "cfcc_analysis",
           "cfcc_cliff_detection",
           "cfcc_decorrelation_rate",
       ],
   };
   ```

3. **Run CFCC algorithm on same corpus**
   - Same files, new schema version
   - Results stored alongside v1.1.0 results

### Phase 3: Statistical Comparison

1. **Query comparison data**
   ```sql
   SELECT
     a.file_path,
     a.verdict AS v1_verdict,
     b.verdict AS v2_verdict,
     a.combined_score AS v1_score,
     b.combined_score AS v2_score,
     a.natural_rolloff AS v1_natural,
     b.cfcc_pattern_type AS v2_pattern
   FROM analysis_results a
   JOIN analysis_results b ON a.file_path = b.file_path
   WHERE a.schema_version = '1.1.0'
     AND b.schema_version = '1.2.0';
   ```

2. **Calculate metrics per approach**
   - Accuracy = (TP + TN) / Total
   - Precision = TP / (TP + FP)
   - Recall = TP / (TP + FN)
   - F1 = 2 * (Precision * Recall) / (Precision + Recall)

3. **Identify disagreements**
   ```sql
   SELECT file_path, v1_verdict, v2_verdict
   FROM comparison_view
   WHERE v1_verdict != v2_verdict;
   ```

   These files need manual review - which algorithm is correct?

### Phase 4: Hybrid Approach (v2.0.0)

Based on comparison results, potentially combine both approaches:

```rust
// Possible hybrid scoring
let hybrid_score = match (approach_a.natural_rolloff, approach_b.lossy_pattern) {
    (true, false) => 0,   // Both agree: natural
    (false, true) => 100, // Both agree: lossy
    (true, true) => 50,   // Conflict: needs review
    (false, false) => 50, // Inconclusive
};
```

---

## SQLite Integration

### Schema for Algorithm Comparison

The current `analysis_results` table already captures:
- `schema_version`: Which algorithm version produced the result
- All metrics from both approaches (nullable for backward compatibility)

### New API Endpoints Needed

```
GET /api/db/results?schema=1.1.0     → Results from specific version
GET /api/db/compare?v1=1.1.0&v2=1.2.0 → Side-by-side comparison
GET /api/db/summary                   → Statistics per schema version
GET /api/db/history/:file_path        → All analyses of one file over time
```

### Database Queries for Comparison

```sql
-- Count verdicts per schema version
SELECT schema_version, verdict, COUNT(*)
FROM analysis_results
GROUP BY schema_version, verdict;

-- Find files where verdict changed between versions
SELECT
  a.file_path,
  a.verdict AS old_verdict,
  b.verdict AS new_verdict,
  a.combined_score AS old_score,
  b.combined_score AS new_score
FROM analysis_results a
JOIN analysis_results b
  ON a.file_path = b.file_path
  AND a.schema_version < b.schema_version
WHERE a.verdict != b.verdict;

-- Track algorithm improvements over time
SELECT
  schema_version,
  SUM(CASE WHEN verdict = 'OK' THEN 1 ELSE 0 END) as ok,
  SUM(CASE WHEN verdict = 'TRANSCODE' THEN 1 ELSE 0 END) as transcode,
  AVG(combined_score) as avg_score
FROM analysis_results
GROUP BY schema_version
ORDER BY schema_version;
```

---

## Web UI Enhancements

### Tab 1: Analysis View (Current)
- File list with verdicts
- Detail view with charts
- No changes needed

### Tab 2: Raw Numbers Viewer (NEW)
Purpose: See all metrics for any file, across all schema versions.

**Features:**
- Table view of all SpectralDetails fields
- Side-by-side comparison of different schema versions
- Highlight metrics that changed between versions
- Export to CSV/JSON

**UI Components:**
```jsx
function RawNumbersTab({ file, analysisHistory }) {
  // Show all numeric fields from SpectralDetails
  // Compare across schema versions if multiple exist
}
```

### Tab 3: Algorithm Comparison (NEW)
Purpose: Compare effectiveness of different detection approaches.

**Features:**
- Confusion matrix visualization
- Precision/Recall/F1 per schema version
- List of files where algorithms disagree
- Manual verdict override capability

**UI Components:**
```jsx
function ComparisonTab({ schemaVersions }) {
  // Fetch comparison data from /api/db/compare
  // Show confusion matrix
  // Highlight disagreements for manual review
}
```

### Tab 4: History View (NEW)
Purpose: Track how verdicts change as algorithms improve.

**Features:**
- Timeline of analyses for selected file
- See how score/verdict changed
- Identify which algorithm version first caught a transcode

---

## Implementation Steps

### Step 1: Complete SQLite Integration (v1.1.0)
- [x] Create `db.rs` with schema versioning
- [x] Add `schema_version` to analysis records
- [x] Register current schema on startup
- [ ] Wire up CLI to store results in DB
- [ ] Add API endpoints for database queries
- [ ] Update README with SQLite usage

### Step 2: Raw Numbers Viewer Tab
- [ ] Add CSS for tab navigation
- [ ] Create `RawNumbersViewer` component
- [ ] Display all SpectralDetails fields
- [ ] Add JSON/CSV export buttons

### Step 3: Baseline Testing (v1.1.0)
- [ ] Run analysis on all demo files
- [ ] Store results in SQLite
- [ ] Create ground truth labels for test files
- [ ] Document baseline metrics (accuracy, precision, recall)

### Step 4: Implement CFCC (v1.2.0)
- [ ] Add CFCC calculation functions to `spectral.rs`
- [ ] Store per-window FFT data (or compute correlations on-the-fly)
- [ ] Add new fields to `SpectralDetails`
- [ ] Bump schema to v1.2.0

### Step 5: Comparison Testing
- [ ] Re-run analysis on demo files with v1.2.0
- [ ] Query comparison results from SQLite
- [ ] Generate comparison report
- [ ] Identify and investigate disagreements

### Step 6: Algorithm Comparison Tab
- [ ] Add `/api/db/compare` endpoint
- [ ] Create `AlgorithmComparison` component
- [ ] Visualize confusion matrix
- [ ] Add manual verdict override

### Step 7: History View Tab
- [ ] Add `/api/db/history/:path` endpoint
- [ ] Create `HistoryView` component
- [ ] Show timeline of analyses
- [ ] Highlight verdict changes

### Step 8: Hybrid Algorithm (v2.0.0)
- [ ] Combine best aspects of both approaches
- [ ] Tune weights based on comparison data
- [ ] Bump schema to v2.0.0
- [ ] Final validation testing

---

## Test File Ground Truth

Document the expected verdict for each demo file:

| File | Expected | Reason |
|------|----------|--------|
| `01_TRUE_320.mp3` | OK | Genuine 320kbps |
| `02_TRUE_256.mp3` | OK | Genuine 256kbps |
| `03_FAKE_128_to_320.mp3` | TRANSCODE | Upscaled from 128k |
| `04_TRUE_192.mp3` | OK | Genuine 192kbps |
| `11_FAKE_flac_from_128k.flac` | TRANSCODE | 128k MP3 → FLAC |
| ... | ... | ... |

This ground truth allows automated calculation of accuracy metrics.

---

## Success Criteria

### For v1.2.0 CFCC to be considered an improvement:
1. Equal or better recall on known transcodes (don't miss more fakes)
2. Better precision on lo-fi sources (fewer false positives)
3. F1 score improvement of at least 5%
4. No regression on any category of test files

### For v2.0.0 Hybrid:
1. Best-in-class metrics from both approaches
2. Robust to edge cases that fooled individual approaches
3. Clear documentation of when each sub-approach is weighted higher

---

## Timeline Estimate

| Step | Complexity | Estimated Effort |
|------|------------|------------------|
| 1. SQLite CLI integration | Low | Done |
| 2. Raw Numbers Tab | Medium | 2-3 hours |
| 3. Baseline Testing | Low | 1 hour |
| 4. CFCC Implementation | High | 4-6 hours |
| 5. Comparison Testing | Medium | 2 hours |
| 6. Comparison Tab | Medium | 3 hours |
| 7. History Tab | Medium | 2 hours |
| 8. Hybrid Algorithm | Medium | 3 hours |

---

## Appendix: Schema Version History

| Version | Name | Features Added |
|---------|------|----------------|
| 1.0.0 | initial | binary_analysis, spectral_analysis |
| 1.1.0 | lofi-detection | cutoff_variance, rolloff_slope, transition_width, natural_rolloff |
| 1.2.0 | cfcc-detection | cfcc_analysis, cfcc_cliff_detection (planned) |
| 2.0.0 | hybrid | Combined best of both approaches (planned) |
