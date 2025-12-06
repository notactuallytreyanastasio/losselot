/**
 * WASM Analysis Result Types
 *
 * These types mirror the Rust structs returned by the WASM analyzer:
 * - wasm-analyzer/src/lib.rs
 * - src/analyzer/mod.rs
 * - src/analyzer/binary.rs
 * - src/analyzer/spectral.rs
 */

/** Possible verdicts from analysis */
export type Verdict = 'OK' | 'SUSPECT' | 'TRANSCODE';

/** Band energy measurement from spectral analysis */
export interface BandEnergy {
  band: string;      // e.g., "0-5kHz", "5-10kHz"
  energy: number;    // Energy level in dB
  percentage: number; // Percentage of total energy
}

/** Frequency response point for spectrum visualization */
export interface FrequencyPoint {
  frequency: number; // Frequency in Hz
  magnitude: number; // Magnitude in dB
}

/** MP3 frame info from binary parsing */
export interface FrameInfo {
  bitrate: number;     // kbps
  sample_rate: number; // Hz
  layer: number;       // 1, 2, or 3
  padding: boolean;
}

/** LAME encoder header info */
export interface LameHeader {
  encoder: string;       // e.g., "LAME3.100"
  quality: string | null; // e.g., "V0", "CBR320"
  lowpass: number | null; // Lowpass frequency in Hz
  method: string | null;  // Encoding method
}

/** Binary analysis results (MP3 metadata) */
export interface BinaryAnalysis {
  /** Detected encoder signatures */
  encoders: string[];
  /** LAME header if present */
  lameHeader: LameHeader | null;
  /** Detected lowpass frequency from encoder */
  lowpass: number | null;
  /** Expected bitrate based on lowpass */
  expectedBitrate: string | null;
  /** Frame info samples */
  frames: FrameInfo[];
  /** Detection flags from binary analysis */
  flags: string[];
  /** Binary analysis score contribution */
  score: number;
}

/** Spectral analysis results */
export interface SpectralAnalysis {
  /** Detection flags from spectral analysis */
  flags: string[];
  /** Energy distribution across frequency bands */
  bandEnergies: BandEnergy[];
  /** Detected high-frequency cutoff (Hz) */
  hfCutoff: number | null;
  /** Cutoff variance (for CFCC detection) */
  cutoffVariance: number | null;
  /** Cross-frequency coherence results */
  cfccCliff: boolean;
  /** Natural rolloff detected (lo-fi safe) */
  naturalRolloff: boolean;
  /** Spectral analysis score contribution */
  score: number;
}

/** Complete analysis result from WASM analyzer */
export interface AnalysisResult {
  /** Original filename */
  filename: string;
  /** File format (mp3, flac, wav, etc.) */
  format: string;
  /** Sample rate in Hz */
  sampleRate: number;
  /** Number of channels */
  channels: number;
  /** Duration in seconds */
  duration: number;
  /** Final verdict */
  verdict: Verdict;
  /** Final combined score (0-100) */
  score: number;
  /** Human-readable reason for verdict */
  reason: string;
  /** All detection flags */
  flags: string[];
  /** Binary analysis (MP3 only) */
  binary: BinaryAnalysis | null;
  /** Spectral analysis */
  spectral: SpectralAnalysis;
  /** Spectrogram data for visualization (flattened 2D array) */
  spectrogramData: number[] | null;
  /** Time axis for spectrogram */
  spectrogramTimes: number[] | null;
  /** Frequency axis for spectrogram */
  spectrogramFreqs: number[] | null;
  /** Frequency response curve */
  frequencyResponse: FrequencyPoint[] | null;
}

// ============================================================================
// Helper functions
// ============================================================================

/**
 * Score thresholds matching Rust backend
 */
export const THRESHOLDS = {
  SUSPECT: 35,
  TRANSCODE: 65,
} as const;

/**
 * Determine verdict from score
 */
export function getVerdict(score: number): Verdict {
  if (score >= THRESHOLDS.TRANSCODE) return 'TRANSCODE';
  if (score >= THRESHOLDS.SUSPECT) return 'SUSPECT';
  return 'OK';
}

/**
 * Get CSS class for verdict styling
 */
export function getVerdictClass(verdict: Verdict): string {
  switch (verdict) {
    case 'TRANSCODE': return 'verdict-transcode';
    case 'SUSPECT': return 'verdict-suspect';
    case 'OK': return 'verdict-ok';
  }
}

/**
 * Format duration as mm:ss
 */
export function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

/**
 * Format frequency for display (kHz)
 */
export function formatFrequency(hz: number): string {
  if (hz >= 1000) {
    return `${(hz / 1000).toFixed(1)}kHz`;
  }
  return `${hz}Hz`;
}

/**
 * Check if result has binary analysis data
 */
export function hasBinaryAnalysis(result: AnalysisResult): boolean {
  return result.binary !== null && result.binary.encoders.length > 0;
}

/**
 * Get primary encoder from binary analysis
 */
export function getPrimaryEncoder(result: AnalysisResult): string | null {
  if (!result.binary || result.binary.encoders.length === 0) {
    return null;
  }
  return result.binary.encoders[0];
}
