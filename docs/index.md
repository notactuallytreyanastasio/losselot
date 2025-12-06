---
layout: default
title: Home
---

# Losselot

Detects fake lossless audio. Point it at a FLAC/WAV and it tells you if it's actually from a lossy source.

![Losselot in action](demo.gif)

## Install

```bash
git clone https://github.com/notactuallytreyanastasio/losselot.git
cd losselot
cargo build --release
```

## Use

```bash
# Analyze files
./target/release/losselot ~/Music/

# Web UI
./target/release/losselot serve ~/Music/ --port 3000
```

## How it works

MP3s cut off high frequencies. Converting an MP3 to FLAC doesn't bring them back. Losselot runs FFT analysis to find where the cutoff is.

It also checks for lo-fi/tape recordings that naturally roll off high frequencies (not transcodes).

## Demo

**[Try the interactive demo →](demo/)**

Real decision graph data from development included.

## Also in this repo

- [Decision Graph](decision-graph) - DAG tracking why decisions were made
- [Claude Tooling](claude-tooling) - Slash commands and context recovery
- [Development Story](story) - How this was built
