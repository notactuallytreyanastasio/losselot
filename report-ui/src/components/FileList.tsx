import type { AnalysisResult } from '../types/analysis';
import { getVerdictColor, getVerdictLabel } from '../types/analysis';
import './FileList.css';

interface FileListProps {
  files: AnalysisResult[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
  sortBy: 'score' | 'name' | 'bitrate';
  sortAsc: boolean;
  onSort: (column: 'score' | 'name' | 'bitrate') => void;
}

/**
 * Sortable file list table.
 *
 * Handles click events but doesn't manage selection state itself -
 * that's lifted up to the parent. Classic React pattern.
 */
export function FileList({
  files,
  selectedIndex,
  onSelect,
  sortBy,
  sortAsc,
  onSort
}: FileListProps) {
  const sortIndicator = (column: 'score' | 'name' | 'bitrate') => {
    if (sortBy !== column) return null;
    return <span className="sort-indicator">{sortAsc ? '↑' : '↓'}</span>;
  };

  return (
    <div className="file-list-container">
      <table className="file-list">
        <thead>
          <tr>
            <th>Verdict</th>
            <th onClick={() => onSort('score')} className="sortable">
              Score {sortIndicator('score')}
            </th>
            <th onClick={() => onSort('bitrate')} className="sortable">
              Bitrate {sortIndicator('bitrate')}
            </th>
            <th>Encoder</th>
            <th>Flags</th>
            <th onClick={() => onSort('name')} className="sortable">
              File {sortIndicator('name')}
            </th>
          </tr>
        </thead>
        <tbody>
          {files.map((file, index) => (
            <tr
              key={file.file_path}
              onClick={() => onSelect(index)}
              className={selectedIndex === index ? 'selected' : ''}
            >
              <td>
                <span
                  className="verdict-badge"
                  style={{ backgroundColor: `${getVerdictColor(file.verdict)}20`, color: getVerdictColor(file.verdict) }}
                >
                  {getVerdictLabel(file.verdict)}
                </span>
              </td>
              <td>
                <div className="score-cell">
                  <div className="score-bar">
                    <div
                      className="score-fill"
                      style={{
                        width: `${file.combined_score}%`,
                        backgroundColor: getVerdictColor(file.verdict)
                      }}
                    />
                  </div>
                  <span className="score-value">{file.combined_score}%</span>
                </div>
              </td>
              <td>{file.bitrate}k</td>
              <td className="encoder-cell">{file.encoder}</td>
              <td className="flags-cell">
                {file.flags.length > 0 ? (
                  file.flags.slice(0, 2).map(flag => (
                    <span key={flag} className="flag-tag">{flag}</span>
                  ))
                ) : (
                  <span className="dim">—</span>
                )}
                {file.flags.length > 2 && (
                  <span className="flag-more">+{file.flags.length - 2}</span>
                )}
              </td>
              <td className="filename-cell" title={file.file_path}>
                {file.file_name}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
