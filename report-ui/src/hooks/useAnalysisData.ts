import { useState, useCallback, useMemo } from 'react';
import type { ReportData, AnalysisResult, Summary } from '../types/analysis';

/**
 * Custom hook for managing analysis data state.
 *
 * Dan Abramov style: Keep state minimal, derive everything else.
 * The only state we need is the raw data and the selected file.
 */
export function useAnalysisData(initialData: ReportData | null = null) {
  const [data, setData] = useState<ReportData | null>(initialData);
  const [selectedFileIndex, setSelectedFileIndex] = useState<number | null>(null);
  const [sortBy, setSortBy] = useState<'score' | 'name' | 'bitrate'>('score');
  const [sortAsc, setSortAsc] = useState(false);

  // Derived: sorted files
  const sortedFiles = useMemo(() => {
    if (!data) return [];

    const files = [...data.files];
    files.sort((a, b) => {
      let cmp = 0;
      switch (sortBy) {
        case 'score':
          cmp = a.combined_score - b.combined_score;
          break;
        case 'name':
          cmp = a.file_name.localeCompare(b.file_name);
          break;
        case 'bitrate':
          cmp = a.bitrate - b.bitrate;
          break;
      }
      return sortAsc ? cmp : -cmp;
    });
    return files;
  }, [data, sortBy, sortAsc]);

  // Derived: selected file
  const selectedFile = useMemo(() => {
    if (selectedFileIndex === null || !sortedFiles.length) return null;
    return sortedFiles[selectedFileIndex] ?? null;
  }, [sortedFiles, selectedFileIndex]);

  // Derived: summary (computed fresh if data changes)
  const summary: Summary = useMemo(() => {
    if (!data) return { total: 0, ok: 0, suspect: 0, transcode: 0, error: 0 };
    return data.summary;
  }, [data]);

  // Derived: files grouped by folder (for collection map)
  const filesByFolder = useMemo(() => {
    if (!data) return new Map<string, AnalysisResult[]>();

    const map = new Map<string, AnalysisResult[]>();
    for (const file of data.files) {
      const lastSlash = file.file_path.lastIndexOf('/');
      const folder = lastSlash > 0 ? file.file_path.slice(0, lastSlash) : '(root)';

      if (!map.has(folder)) {
        map.set(folder, []);
      }
      map.get(folder)!.push(file);
    }
    return map;
  }, [data]);

  // Actions
  const selectFile = useCallback((index: number | null) => {
    setSelectedFileIndex(index);
  }, []);

  const selectFileByPath = useCallback((path: string) => {
    if (!data) return;
    const index = sortedFiles.findIndex(f => f.file_path === path);
    if (index >= 0) setSelectedFileIndex(index);
  }, [data, sortedFiles]);

  const toggleSort = useCallback((column: 'score' | 'name' | 'bitrate') => {
    if (sortBy === column) {
      setSortAsc(prev => !prev);
    } else {
      setSortBy(column);
      setSortAsc(false);
    }
  }, [sortBy]);

  const loadData = useCallback((newData: ReportData) => {
    setData(newData);
    setSelectedFileIndex(null);
  }, []);

  return {
    // State
    data,
    summary,
    sortedFiles,
    selectedFile,
    selectedFileIndex,
    filesByFolder,
    sortBy,
    sortAsc,

    // Actions
    selectFile,
    selectFileByPath,
    toggleSort,
    loadData,
  };
}

// For loading data from a JSON file or embedded script tag
export async function loadReportData(): Promise<ReportData> {
  // Option 1: Check for embedded data (like current HTML approach)
  if (typeof window !== 'undefined' && (window as any).__LOSSELOT_DATA__) {
    return (window as any).__LOSSELOT_DATA__;
  }

  // Option 2: Load from a JSON file
  const response = await fetch('./report-data.json');
  if (!response.ok) {
    throw new Error(`Failed to load report data: ${response.statusText}`);
  }
  return response.json();
}
