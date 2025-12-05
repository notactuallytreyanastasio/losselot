import type { AnalysisResult } from '../types/analysis';
import { getVerdictColor, getVerdictLabel } from '../types/analysis';
import { Spectrogram } from './Spectrogram';
import { BitrateTimeline } from './BitrateTimeline';
import { StereoCorrelation } from './StereoCorrelation';
import './FileDetail.css';

interface FileDetailProps {
  file: AnalysisResult;
  onClose: () => void;
}

/**
 * Detailed view of a single file's analysis.
 *
 * Composition: combines multiple visualization components.
 * Each viz component is independent and reusable.
 */
export function FileDetail({ file, onClose }: FileDetailProps) {
  return (
    <div className="file-detail">
      <div className="file-detail-header">
        <div className="file-detail-title">
          <h2>{file.file_name}</h2>
          <span className="file-path">{file.file_path}</span>
        </div>
        <button className="close-btn" onClick={onClose} aria-label="Close">
          ×
        </button>
      </div>

      <div className="file-detail-summary">
        <div
          className="verdict-large"
          style={{
            backgroundColor: `${getVerdictColor(file.verdict)}15`,
            color: getVerdictColor(file.verdict)
          }}
        >
          {getVerdictLabel(file.verdict)}
        </div>

        <div className="detail-stats">
          <div className="detail-stat">
            <span className="stat-value">{file.combined_score}%</span>
            <span className="stat-label">Score</span>
          </div>
          <div className="detail-stat">
            <span className="stat-value">{file.bitrate}k</span>
            <span className="stat-label">Bitrate</span>
          </div>
          <div className="detail-stat">
            <span className="stat-value">{file.encoder || '—'}</span>
            <span className="stat-label">Encoder</span>
          </div>
          <div className="detail-stat">
            <span className="stat-value">{file.lowpass ? `${file.lowpass}Hz` : '—'}</span>
            <span className="stat-label">Lowpass</span>
          </div>
        </div>
      </div>

      {file.flags.length > 0 && (
        <div className="file-detail-flags">
          <h4>Detection Flags</h4>
          <div className="flags-list">
            {file.flags.map(flag => (
              <span key={flag} className="flag-tag">{flag}</span>
            ))}
          </div>
        </div>
      )}

      <div className="file-detail-visualizations">
        {file.spectrogram && (
          <Spectrogram data={file.spectrogram} />
        )}

        {file.bitrate_timeline && (
          <BitrateTimeline data={file.bitrate_timeline} />
        )}

        {file.stereo_correlation && (
          <StereoCorrelation data={file.stereo_correlation} />
        )}

        {file.spectral && (
          <div className="spectral-details">
            <h4>Spectral Analysis</h4>
            <div className="spectral-grid">
              <div className="spectral-item">
                <span className="value">{file.spectral.upper_drop.toFixed(1)} dB</span>
                <span className="label">Upper Drop</span>
              </div>
              <div className="spectral-item">
                <span className="value">{file.spectral.ultrasonic_drop.toFixed(1)} dB</span>
                <span className="label">Ultrasonic Drop</span>
              </div>
              <div className="spectral-item">
                <span className="value">{file.spectral.rms_full.toFixed(1)} dB</span>
                <span className="label">RMS Full</span>
              </div>
              <div className="spectral-item">
                <span className="value">{file.spectral.rms_ultrasonic.toFixed(1)} dB</span>
                <span className="label">RMS Ultrasonic</span>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
