import { useEffect, useState } from 'react';
import { useAnalysisData, loadReportData } from './hooks/useAnalysisData';
import { SummaryCards } from './components/SummaryCards';
import { FileList } from './components/FileList';
import { FileDetail } from './components/FileDetail';
import type { ReportData } from './types/analysis';
import './App.css';

// Demo data for development - in production this comes from Rust
const DEMO_DATA: ReportData = {
  summary: { total: 5, ok: 2, suspect: 1, transcode: 2, error: 0 },
  generated_at: new Date().toISOString(),
  files: [
    {
      file_path: '/music/Album/01_clean.mp3',
      file_name: '01_clean.mp3',
      bitrate: 320,
      sample_rate: 44100,
      duration_secs: 180,
      verdict: 'OK',
      combined_score: 12,
      spectral_score: 8,
      binary_score: 4,
      flags: [],
      encoder: 'LAME3.100',
    },
    {
      file_path: '/music/Album/02_suspect.mp3',
      file_name: '02_suspect.mp3',
      bitrate: 256,
      sample_rate: 44100,
      duration_secs: 210,
      verdict: 'SUSPECT',
      combined_score: 45,
      spectral_score: 30,
      binary_score: 15,
      flags: ['possible_320k_origin'],
      encoder: 'LAME3.99',
      stereo_correlation: {
        times: [0, 0.5, 1, 1.5, 2, 2.5, 3],
        correlations: [0.92, 0.89, 0.91, 0.88, 0.90, 0.93, 0.91],
        avg_correlation: 0.91,
        min_correlation: 0.88,
        max_correlation: 0.93
      }
    },
    {
      file_path: '/music/Album/03_transcode.mp3',
      file_name: '03_transcode.mp3',
      bitrate: 320,
      sample_rate: 44100,
      duration_secs: 195,
      verdict: 'TRANSCODE',
      combined_score: 78,
      spectral_score: 45,
      binary_score: 33,
      flags: ['hf_cutoff_detected', 'lowpass_bitrate_mismatch'],
      encoder: 'LAME3.100',
      lowpass: 16000,
      bitrate_timeline: {
        times: [0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4],
        bitrates: [320, 320, 320, 320, 320, 320, 320, 320, 320],
        is_vbr: false,
        min_bitrate: 320,
        max_bitrate: 320,
        avg_bitrate: 320
      }
    },
    {
      file_path: '/music/Album/04_fake_flac.flac',
      file_name: '04_fake_flac.flac',
      bitrate: 1411,
      sample_rate: 44100,
      duration_secs: 240,
      verdict: 'TRANSCODE',
      combined_score: 85,
      spectral_score: 55,
      binary_score: 30,
      flags: ['severe_hf_damage', 'dead_ultrasonic_band'],
      encoder: 'FLAC',
      spectral: {
        rms_full: -18.5,
        rms_mid_high: -24.2,
        rms_high: -45.6,
        rms_upper: -62.3,
        rms_ultrasonic: -78.1,
        upper_drop: 38.1,
        ultrasonic_drop: 53.9,
        ultrasonic_flatness: 0.12
      }
    },
    {
      file_path: '/music/Album/05_clean_vbr.mp3',
      file_name: '05_clean_vbr.mp3',
      bitrate: 245,
      sample_rate: 44100,
      duration_secs: 200,
      verdict: 'OK',
      combined_score: 8,
      spectral_score: 5,
      binary_score: 3,
      flags: [],
      encoder: 'LAME3.100',
      bitrate_timeline: {
        times: [0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4],
        bitrates: [320, 256, 224, 192, 256, 320, 288, 224, 256],
        is_vbr: true,
        min_bitrate: 192,
        max_bitrate: 320,
        avg_bitrate: 253
      }
    }
  ]
};

/**
 * Main App component.
 *
 * Dan Abramov philosophy:
 * - State lives at the top, flows down as props
 * - Components are functions of their props
 * - Hooks encapsulate related state logic
 */
function App() {
  const [loading, setLoading] = useState(true);
  const [error, _setError] = useState<string | null>(null);

  const {
    summary,
    sortedFiles,
    selectedFile,
    selectedFileIndex,
    sortBy,
    sortAsc,
    selectFile,
    toggleSort,
    loadData
  } = useAnalysisData();

  useEffect(() => {
    // Try to load real data, fall back to demo
    loadReportData()
      .then(data => {
        loadData(data);
        setLoading(false);
      })
      .catch(() => {
        // Use demo data for development
        loadData(DEMO_DATA);
        setLoading(false);
      });
  }, [loadData]);

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner" />
        <p>Loading analysis...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error">
        <p>Error: {error}</p>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="header">
        <h1 className="logo">Losselot</h1>
        <span className="subtitle">Audio Authenticity Report</span>
      </header>

      <main className="container">
        <SummaryCards summary={summary} />

        {selectedFile && (
          <FileDetail
            file={selectedFile}
            onClose={() => selectFile(null)}
          />
        )}

        <FileList
          files={sortedFiles}
          selectedIndex={selectedFileIndex}
          onSelect={selectFile}
          sortBy={sortBy}
          sortAsc={sortAsc}
          onSort={toggleSort}
        />
      </main>
    </div>
  );
}

export default App;
