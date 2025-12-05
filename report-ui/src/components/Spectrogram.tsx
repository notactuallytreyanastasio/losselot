import { useRef, useEffect } from 'react';
import type { Spectrogram as SpectrogramData } from '../types/analysis';
import './Spectrogram.css';

interface SpectrogramProps {
  data: SpectrogramData;
  width?: number;
  height?: number;
}

/**
 * Canvas-based spectrogram visualization.
 *
 * Using Canvas here because we're rendering potentially thousands of
 * frequency bins - SVG would be too slow. This is one case where
 * imperative DOM manipulation (via useEffect) makes sense in React.
 */
export function Spectrogram({ data, width = 600, height = 200 }: SpectrogramProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const { magnitudes, num_freq_bins, num_time_slices } = data;

    // Clear canvas
    ctx.fillStyle = '#1a1a2e';
    ctx.fillRect(0, 0, width, height);

    // Calculate cell dimensions
    const cellWidth = width / num_time_slices;
    const cellHeight = height / num_freq_bins;

    // Find magnitude range for normalization
    let minMag = Infinity, maxMag = -Infinity;
    for (const m of magnitudes) {
      if (m < minMag) minMag = m;
      if (m > maxMag) maxMag = m;
    }

    // Draw spectrogram cells
    for (let t = 0; t < num_time_slices; t++) {
      for (let f = 0; f < num_freq_bins; f++) {
        const index = t * num_freq_bins + f;
        const magnitude = magnitudes[index];

        // Normalize to 0-1
        const normalized = (magnitude - minMag) / (maxMag - minMag || 1);

        // Magma-like colormap
        const color = magmaColormap(normalized);
        ctx.fillStyle = color;

        // Y is inverted (low freq at bottom)
        const y = height - (f + 1) * cellHeight;
        ctx.fillRect(t * cellWidth, y, cellWidth + 0.5, cellHeight + 0.5);
      }
    }
  }, [data, width, height]);

  return (
    <div className="spectrogram-container">
      <div className="spectrogram-header">
        <h3>Spectrogram</h3>
        <span className="subtitle">Time vs Frequency</span>
      </div>
      <div className="spectrogram-canvas-wrapper">
        <canvas
          ref={canvasRef}
          width={width}
          height={height}
          className="spectrogram-canvas"
        />
        <div className="spectrogram-y-axis">
          <span>22kHz</span>
          <span>11kHz</span>
          <span>0Hz</span>
        </div>
        <div className="spectrogram-x-axis">
          <span>0s</span>
          <span>{(data.times[data.times.length - 1] || 15).toFixed(0)}s</span>
        </div>
      </div>
    </div>
  );
}

/**
 * Magma-like colormap function.
 * Input: 0-1 normalized value
 * Output: CSS color string
 */
function magmaColormap(t: number): string {
  // Simplified magma colormap
  const r = Math.round(255 * Math.min(1, t * 2.5));
  const g = Math.round(255 * Math.max(0, Math.min(1, (t - 0.3) * 1.5)));
  const b = Math.round(255 * Math.max(0, Math.min(1, 0.3 + t * 0.5)));

  // Darker at low values
  if (t < 0.1) {
    return `rgb(${Math.round(t * 100)}, ${Math.round(t * 50)}, ${Math.round(t * 150 + 30)})`;
  }

  return `rgb(${r}, ${g}, ${b})`;
}
