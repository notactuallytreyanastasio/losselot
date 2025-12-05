/**
 * TypeScript interfaces matching Rust's AnalysisResult and related types.
 * These form the data contract between the Rust analyzer and React UI.
 */

export type Verdict = 'OK' | 'SUSPECT' | 'TRANSCODE' | 'ERROR';

export interface Spectrogram {
  times: number[];
  frequencies: number[];
  magnitudes: number[];
  num_freq_bins: number;
  num_time_slices: number;
}

export interface StereoCorrelation {
  times: number[];
  correlations: number[];
  avg_correlation: number;
  min_correlation: number;
  max_correlation: number;
}

export interface SpectralDetails {
  rms_full: number;
  rms_mid_high: number;
  rms_high: number;
  rms_upper: number;
  rms_ultrasonic: number;
  upper_drop: number;
  ultrasonic_drop: number;
  ultrasonic_flatness: number;
  spectrogram?: Spectrogram;
  stereo_correlation?: StereoCorrelation;
}

export interface BitrateTimeline {
  times: number[];
  bitrates: number[];
  is_vbr: boolean;
  min_bitrate: number;
  max_bitrate: number;
  avg_bitrate: number;
}

export interface BinaryDetails {
  lowpass?: number;
  encoder_signatures: string[];
  bitrate_timeline?: BitrateTimeline;
}

export interface AnalysisResult {
  file_path: string;
  file_name: string;
  bitrate: number;
  sample_rate: number;
  duration_secs: number;
  verdict: Verdict;
  combined_score: number;
  spectral_score: number;
  binary_score: number;
  flags: string[];
  encoder: string;
  lowpass?: number;
  spectral?: SpectralDetails;
  binary?: BinaryDetails;
  spectrogram?: Spectrogram;
  bitrate_timeline?: BitrateTimeline;
  stereo_correlation?: StereoCorrelation;
  error?: string;
}

export interface Summary {
  total: number;
  ok: number;
  suspect: number;
  transcode: number;
  error: number;
}

export interface ReportData {
  summary: Summary;
  files: AnalysisResult[];
  generated_at: string;
}

// Utility functions
export function getVerdictColor(verdict: Verdict): string {
  switch (verdict) {
    case 'OK': return '#34c759';
    case 'SUSPECT': return '#ff9f0a';
    case 'TRANSCODE': return '#ff3b30';
    case 'ERROR': return '#8e8e93';
  }
}

export function getVerdictLabel(verdict: Verdict): string {
  switch (verdict) {
    case 'OK': return 'Clean';
    case 'SUSPECT': return 'Suspect';
    case 'TRANSCODE': return 'Transcode';
    case 'ERROR': return 'Error';
  }
}
