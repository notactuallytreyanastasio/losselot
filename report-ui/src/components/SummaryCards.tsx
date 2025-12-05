import type { Summary } from '../types/analysis';
import './SummaryCards.css';

interface SummaryCardsProps {
  summary: Summary;
}

/**
 * Summary statistics cards showing verdict distribution.
 *
 * Simple, pure component - just renders props. No state.
 */
export function SummaryCards({ summary }: SummaryCardsProps) {
  return (
    <div className="summary-cards">
      <div className="stat-card ok">
        <div className="stat-value">{summary.ok}</div>
        <div className="stat-label">Clean</div>
      </div>
      <div className="stat-card suspect">
        <div className="stat-value">{summary.suspect}</div>
        <div className="stat-label">Suspect</div>
      </div>
      <div className="stat-card transcode">
        <div className="stat-value">{summary.transcode}</div>
        <div className="stat-label">Transcode</div>
      </div>
      <div className="stat-card total">
        <div className="stat-value">{summary.total}</div>
        <div className="stat-label">Total Files</div>
      </div>
    </div>
  );
}
