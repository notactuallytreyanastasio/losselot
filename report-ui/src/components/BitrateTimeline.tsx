import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ReferenceLine,
  ResponsiveContainer
} from 'recharts';
import type { BitrateTimeline as BitrateTimelineData } from '../types/analysis';
import './BitrateTimeline.css';

interface BitrateTimelineProps {
  data: BitrateTimelineData;
}

/**
 * Bitrate over time visualization using Recharts.
 *
 * Recharts is React-friendly - no direct DOM manipulation.
 * Just pass data as props and it renders.
 */
export function BitrateTimeline({ data }: BitrateTimelineProps) {
  // Transform data for Recharts
  const chartData = data.times.map((time, i) => ({
    time: parseFloat(time.toFixed(2)),
    bitrate: data.bitrates[i]
  }));

  return (
    <div className="bitrate-timeline-container">
      <div className="bitrate-timeline-header">
        <h3>Bitrate Timeline</h3>
        <div className="bitrate-badges">
          <span className={`vbr-badge ${data.is_vbr ? 'vbr' : 'cbr'}`}>
            {data.is_vbr ? 'VBR' : 'CBR'}
          </span>
          <span className="bitrate-range">
            {data.min_bitrate}–{data.max_bitrate} kbps
          </span>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={150}>
        <AreaChart data={chartData} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
          <defs>
            <linearGradient id="bitrateGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stopColor="#007aff" stopOpacity={0.3} />
              <stop offset="95%" stopColor="#007aff" stopOpacity={0.05} />
            </linearGradient>
          </defs>

          <XAxis
            dataKey="time"
            tickFormatter={(v) => `${v}s`}
            tick={{ fontSize: 10, fill: '#86868b' }}
            axisLine={{ stroke: '#d2d2d7' }}
            tickLine={false}
          />
          <YAxis
            tickFormatter={(v) => `${v}k`}
            tick={{ fontSize: 10, fill: '#86868b' }}
            axisLine={false}
            tickLine={false}
            width={40}
          />

          <Tooltip
            contentStyle={{
              backgroundColor: '#fff',
              border: '1px solid #d2d2d7',
              borderRadius: 8,
              fontSize: 12
            }}
            formatter={(value: number) => [`${value} kbps`, 'Bitrate']}
            labelFormatter={(label) => `Time: ${label}s`}
          />

          <ReferenceLine
            y={data.avg_bitrate}
            stroke="#ff9f0a"
            strokeDasharray="4 4"
            label={{
              value: `avg: ${data.avg_bitrate}k`,
              position: 'right',
              fontSize: 10,
              fill: '#ff9f0a'
            }}
          />

          <Area
            type="stepAfter"
            dataKey="bitrate"
            stroke="#007aff"
            strokeWidth={1.5}
            fill="url(#bitrateGradient)"
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}
