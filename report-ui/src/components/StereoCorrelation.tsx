import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ReferenceArea,
  ResponsiveContainer
} from 'recharts';
import type { StereoCorrelation as StereoCorrelationData } from '../types/analysis';
import './StereoCorrelation.css';

interface StereoCorrelationProps {
  data: StereoCorrelationData;
}

function getCorrelationStatus(avg: number): { label: string; color: string } {
  if (avg >= 0.99) return { label: 'MONO', color: '#ff3b30' };
  if (avg >= 0.95) return { label: 'Near-Mono', color: '#ff9f0a' };
  if (avg >= 0.7) return { label: 'Normal Stereo', color: '#34c759' };
  if (avg >= 0.3) return { label: 'Wide Stereo', color: '#007aff' };
  return { label: 'Phase Issues', color: '#ff3b30' };
}

/**
 * Stereo correlation visualization showing L/R channel similarity.
 */
export function StereoCorrelation({ data }: StereoCorrelationProps) {
  const chartData = data.times.map((time, i) => ({
    time: parseFloat(time.toFixed(2)),
    correlation: data.correlations[i]
  }));

  const status = getCorrelationStatus(data.avg_correlation);

  return (
    <div className="stereo-correlation-container">
      <div className="stereo-correlation-header">
        <div>
          <h3>Stereo Correlation</h3>
          <span className="subtitle">L/R channel similarity</span>
        </div>
        <div className="stereo-stats">
          <span className="status-badge" style={{ backgroundColor: `${status.color}20`, color: status.color }}>
            {status.label}
          </span>
          <span className="correlation-value">
            avg: {data.avg_correlation.toFixed(2)}
          </span>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={150}>
        <AreaChart data={chartData} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
          <defs>
            <linearGradient id="correlationGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stopColor="#34c759" stopOpacity={0.3} />
              <stop offset="95%" stopColor="#34c759" stopOpacity={0.05} />
            </linearGradient>
          </defs>

          {/* Normal stereo range highlight */}
          <ReferenceArea
            y1={0.7}
            y2={0.95}
            fill="#34c759"
            fillOpacity={0.1}
          />

          <XAxis
            dataKey="time"
            tickFormatter={(v) => `${v}s`}
            tick={{ fontSize: 10, fill: '#86868b' }}
            axisLine={{ stroke: '#d2d2d7' }}
            tickLine={false}
          />
          <YAxis
            domain={[-0.2, 1.1]}
            ticks={[-0.0, 0.5, 1.0]}
            tick={{ fontSize: 10, fill: '#86868b' }}
            axisLine={false}
            tickLine={false}
            width={30}
          />

          <Tooltip
            contentStyle={{
              backgroundColor: '#fff',
              border: '1px solid #d2d2d7',
              borderRadius: 8,
              fontSize: 12
            }}
            formatter={(value: number) => [value.toFixed(3), 'Correlation']}
            labelFormatter={(label) => `Time: ${label}s`}
          />

          <Area
            type="monotone"
            dataKey="correlation"
            stroke="#34c759"
            strokeWidth={1.5}
            fill="url(#correlationGradient)"
          />
        </AreaChart>
      </ResponsiveContainer>

      <div className="correlation-range">
        <span>min: {data.min_correlation.toFixed(2)}</span>
        <span>max: {data.max_correlation.toFixed(2)}</span>
      </div>
    </div>
  );
}
