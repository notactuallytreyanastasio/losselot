import { describe, it, expect } from 'vitest';
import {
  getVerdict,
  getVerdictClass,
  formatDuration,
  formatFrequency,
  hasBinaryAnalysis,
  getPrimaryEncoder,
  THRESHOLDS,
  type AnalysisResult,
} from '../analysis';

describe('analysis type helpers', () => {
  describe('THRESHOLDS', () => {
    it('has correct threshold values matching Rust backend', () => {
      expect(THRESHOLDS.SUSPECT).toBe(35);
      expect(THRESHOLDS.TRANSCODE).toBe(65);
    });
  });

  describe('getVerdict', () => {
    it('returns OK for low scores', () => {
      expect(getVerdict(0)).toBe('OK');
      expect(getVerdict(34)).toBe('OK');
    });

    it('returns SUSPECT for mid scores', () => {
      expect(getVerdict(35)).toBe('SUSPECT');
      expect(getVerdict(50)).toBe('SUSPECT');
      expect(getVerdict(64)).toBe('SUSPECT');
    });

    it('returns TRANSCODE for high scores', () => {
      expect(getVerdict(65)).toBe('TRANSCODE');
      expect(getVerdict(80)).toBe('TRANSCODE');
      expect(getVerdict(100)).toBe('TRANSCODE');
    });
  });

  describe('getVerdictClass', () => {
    it('returns correct CSS class for each verdict', () => {
      expect(getVerdictClass('OK')).toBe('verdict-ok');
      expect(getVerdictClass('SUSPECT')).toBe('verdict-suspect');
      expect(getVerdictClass('TRANSCODE')).toBe('verdict-transcode');
    });
  });

  describe('formatDuration', () => {
    it('formats seconds as mm:ss', () => {
      expect(formatDuration(0)).toBe('0:00');
      expect(formatDuration(30)).toBe('0:30');
      expect(formatDuration(60)).toBe('1:00');
      expect(formatDuration(90)).toBe('1:30');
      expect(formatDuration(125)).toBe('2:05');
      expect(formatDuration(3661)).toBe('61:01');
    });

    it('pads seconds with leading zero', () => {
      expect(formatDuration(5)).toBe('0:05');
      expect(formatDuration(65)).toBe('1:05');
    });
  });

  describe('formatFrequency', () => {
    it('formats Hz for low frequencies', () => {
      expect(formatFrequency(440)).toBe('440Hz');
      expect(formatFrequency(999)).toBe('999Hz');
    });

    it('formats kHz for high frequencies', () => {
      expect(formatFrequency(1000)).toBe('1.0kHz');
      expect(formatFrequency(16000)).toBe('16.0kHz');
      expect(formatFrequency(19500)).toBe('19.5kHz');
    });
  });

  describe('hasBinaryAnalysis', () => {
    const makeResult = (binary: AnalysisResult['binary']): AnalysisResult => ({
      filename: 'test.mp3',
      format: 'mp3',
      sampleRate: 44100,
      channels: 2,
      duration: 180,
      verdict: 'OK',
      score: 10,
      reason: 'Clean',
      flags: [],
      binary,
      spectral: {
        flags: [],
        bandEnergies: [],
        hfCutoff: null,
        cutoffVariance: null,
        cfccCliff: false,
        naturalRolloff: false,
        score: 0,
      },
      spectrogramData: null,
      spectrogramTimes: null,
      spectrogramFreqs: null,
      frequencyResponse: null,
    });

    it('returns false for null binary', () => {
      expect(hasBinaryAnalysis(makeResult(null))).toBe(false);
    });

    it('returns false for empty encoders', () => {
      expect(hasBinaryAnalysis(makeResult({
        encoders: [],
        lameHeader: null,
        lowpass: null,
        expectedBitrate: null,
        frames: [],
        flags: [],
        score: 0,
      }))).toBe(false);
    });

    it('returns true when encoders present', () => {
      expect(hasBinaryAnalysis(makeResult({
        encoders: ['LAME3.100'],
        lameHeader: null,
        lowpass: 16000,
        expectedBitrate: '128kbps',
        frames: [],
        flags: [],
        score: 5,
      }))).toBe(true);
    });
  });

  describe('getPrimaryEncoder', () => {
    const makeResult = (encoders: string[]): AnalysisResult => ({
      filename: 'test.mp3',
      format: 'mp3',
      sampleRate: 44100,
      channels: 2,
      duration: 180,
      verdict: 'OK',
      score: 10,
      reason: 'Clean',
      flags: [],
      binary: {
        encoders,
        lameHeader: null,
        lowpass: null,
        expectedBitrate: null,
        frames: [],
        flags: [],
        score: 0,
      },
      spectral: {
        flags: [],
        bandEnergies: [],
        hfCutoff: null,
        cutoffVariance: null,
        cfccCliff: false,
        naturalRolloff: false,
        score: 0,
      },
      spectrogramData: null,
      spectrogramTimes: null,
      spectrogramFreqs: null,
      frequencyResponse: null,
    });

    it('returns null for null binary', () => {
      const result = makeResult([]);
      result.binary = null;
      expect(getPrimaryEncoder(result)).toBeNull();
    });

    it('returns null for empty encoders', () => {
      expect(getPrimaryEncoder(makeResult([]))).toBeNull();
    });

    it('returns first encoder', () => {
      expect(getPrimaryEncoder(makeResult(['LAME3.100', 'FFmpeg']))).toBe('LAME3.100');
    });
  });
});
