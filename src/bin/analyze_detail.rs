//! Detailed spectral comparison tool for investigating 320k detection

use rustfft::{num_complex::Complex, FftPlanner};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::env;
use std::fs::File;
use std::io::Read;

const FFT_SIZE: usize = 8192;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: analyze_detail <file1> [file2]");
        std::process::exit(1);
    }

    for path in &args[1..] {
        println!("\n{}", "=".repeat(60));
        println!("FILE: {}", path);
        println!("{}", "=".repeat(60));
        analyze_file(path);
    }
}

fn analyze_file(path: &str) {
    let mut file = File::open(path).expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    let (samples, sample_rate) = match decode_audio(&data) {
        Some(s) => s,
        None => {
            eprintln!("Failed to decode audio");
            return;
        }
    };

    println!("Sample rate: {} Hz", sample_rate);
    println!("Samples: {} ({:.2}s)", samples.len(), samples.len() as f64 / sample_rate as f64);

    // Set up FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(FFT_SIZE);
    let window = hanning_window(FFT_SIZE);

    // Process windows and accumulate spectrum
    let hop_size = FFT_SIZE / 2;
    let num_windows = (samples.len() - FFT_SIZE) / hop_size + 1;
    let mut avg_spectrum = vec![0.0f64; FFT_SIZE / 2];

    for i in 0..num_windows.min(100) {
        let start = i * hop_size;
        let end = start + FFT_SIZE;
        if end > samples.len() {
            break;
        }

        let mut buffer: Vec<Complex<f64>> = samples[start..end]
            .iter()
            .zip(window.iter())
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();

        fft.process(&mut buffer);

        for (j, c) in buffer.iter().take(FFT_SIZE / 2).enumerate() {
            avg_spectrum[j] += c.norm();
        }
    }

    let num_windows = num_windows.min(100) as f64;
    for v in &mut avg_spectrum {
        *v /= num_windows;
    }

    // Convert to dB
    let spectrum_db: Vec<f64> = avg_spectrum.iter().map(|&v| to_db(v)).collect();

    // Frequency resolution
    let bin_hz = sample_rate as f64 / FFT_SIZE as f64;
    println!("Bin resolution: {:.2} Hz", bin_hz);

    // Print energy in 1kHz bands from 10kHz to 22kHz
    println!("\nEnergy by 1kHz bands:");
    println!("{:>8} {:>10} {:>10}", "Band", "Energy(dB)", "Δ from prev");

    let mut prev_energy = None;
    for start_khz in (10..=21).step_by(1) {
        let start_hz = start_khz * 1000;
        let end_hz = start_hz + 1000;
        let energy = band_energy_db(&spectrum_db, sample_rate, start_hz as u32, end_hz as u32);

        let delta = match prev_energy {
            Some(p) => format!("{:+.1}", energy - p),
            None => "-".to_string(),
        };

        println!("{:>5}-{:<2}k {:>10.1} {:>10}", start_khz, start_khz + 1, energy, delta);
        prev_energy = Some(energy);
    }

    // Spectral flatness in different regions
    println!("\nSpectral flatness (higher = more noise-like, lower = more tonal):");
    println!("  10-15kHz: {:.4}", spectral_flatness(&avg_spectrum, sample_rate, 10000, 15000));
    println!("  15-20kHz: {:.4}", spectral_flatness(&avg_spectrum, sample_rate, 15000, 20000));
    println!("  17-20kHz: {:.4}", spectral_flatness(&avg_spectrum, sample_rate, 17000, 20000));
    println!("  19-21kHz: {:.4}", spectral_flatness(&avg_spectrum, sample_rate, 19000, 21000));

    // Rolloff analysis - find -3dB, -10dB, -20dB points from peak
    let peak_idx = spectrum_db.iter()
        .enumerate()
        .skip(10) // skip DC
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);
    let peak_db = spectrum_db[peak_idx];

    println!("\nRolloff analysis (from peak at {:.0}Hz = {:.1}dB):", peak_idx as f64 * bin_hz, peak_db);

    for threshold in [-3.0, -10.0, -20.0, -30.0, -40.0] {
        let target = peak_db + threshold;
        // Find highest frequency bin above target
        let rolloff_bin = spectrum_db.iter()
            .enumerate()
            .rev()
            .find(|(_, &db)| db >= target)
            .map(|(i, _)| i);

        if let Some(bin) = rolloff_bin {
            println!("  {}dB point: {:.0} Hz", threshold, bin as f64 * bin_hz);
        }
    }

    // Derivative analysis - how sharply does it roll off?
    println!("\nRolloff sharpness (dB/kHz) in high frequency region:");
    for start_khz in [16, 17, 18, 19, 20] {
        let e1 = band_energy_db(&spectrum_db, sample_rate, start_khz * 1000, start_khz * 1000 + 500);
        let e2 = band_energy_db(&spectrum_db, sample_rate, start_khz * 1000 + 500, (start_khz + 1) * 1000);
        let slope = (e2 - e1) * 2.0; // dB per kHz
        println!("  {}-{}kHz: {:.1} dB/kHz", start_khz, start_khz + 1, slope);
    }
}

fn decode_audio(data: &[u8]) -> Option<(Vec<f64>, u32)> {
    let cursor = std::io::Cursor::new(data.to_vec());
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .ok()?;

    let mut format = probed.format;
    let track = format.default_track()?;
    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .ok()?;

    let mut samples = Vec::new();
    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    let max_samples = (sample_rate as usize) * 15;

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(_) => break,
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(_) => continue,
        };

        if sample_buf.is_none() {
            let spec = *decoded.spec();
            let duration = decoded.capacity() as u64;
            sample_buf = Some(SampleBuffer::new(duration, spec));
        }

        if let Some(ref mut buf) = sample_buf {
            let channel_count = decoded.spec().channels.count();
            buf.copy_interleaved_ref(decoded);

            for chunk in buf.samples().chunks(channel_count) {
                let mono: f64 = chunk.iter().map(|&s| s as f64).sum::<f64>() / channel_count as f64;
                samples.push(mono);
            }

            if samples.len() >= max_samples {
                break;
            }
        }
    }

    if samples.is_empty() {
        return None;
    }

    Some((samples, sample_rate))
}

fn hanning_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (size - 1) as f64).cos()))
        .collect()
}

fn to_db(value: f64) -> f64 {
    if value <= 0.0 {
        -96.0
    } else {
        20.0 * value.log10()
    }
}

fn band_energy_db(spectrum_db: &[f64], sample_rate: u32, low_hz: u32, high_hz: u32) -> f64 {
    let bin_resolution = sample_rate as f64 / FFT_SIZE as f64;
    let low_bin = (low_hz as f64 / bin_resolution) as usize;
    let high_bin = (high_hz as f64 / bin_resolution).min((FFT_SIZE / 2) as f64) as usize;

    if low_bin >= high_bin || high_bin >= spectrum_db.len() {
        return -96.0;
    }

    // Average dB in band
    let sum: f64 = spectrum_db[low_bin..=high_bin.min(spectrum_db.len() - 1)].iter().sum();
    sum / (high_bin - low_bin + 1) as f64
}

fn spectral_flatness(spectrum: &[f64], sample_rate: u32, low_hz: u32, high_hz: u32) -> f64 {
    let bin_resolution = sample_rate as f64 / FFT_SIZE as f64;
    let low_bin = (low_hz as f64 / bin_resolution) as usize;
    let high_bin = (high_hz as f64 / bin_resolution).min((FFT_SIZE / 2) as f64) as usize;

    if low_bin >= high_bin || high_bin >= spectrum.len() {
        return 0.0;
    }

    let band = &spectrum[low_bin..=high_bin.min(spectrum.len() - 1)];
    let n = band.len() as f64;

    // Geometric mean
    let log_sum: f64 = band.iter().map(|&x| (x + 1e-10).ln()).sum();
    let geo_mean = (log_sum / n).exp();

    // Arithmetic mean
    let arith_mean: f64 = band.iter().sum::<f64>() / n;

    if arith_mean <= 0.0 {
        return 0.0;
    }

    geo_mean / arith_mean
}
