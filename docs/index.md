---
layout: default
title: Home
---

<style>
.hero {
  text-align: center;
  padding: 40px 20px 60px;
}
.hero h1 {
  font-size: 3rem;
  margin-bottom: 10px;
}
.hero .tagline {
  font-size: 1.25rem;
  color: #888;
  margin-bottom: 40px;
}
.hero-gif {
  width: 100%;
  max-width: 1000px;
  border-radius: 12px;
  margin: 0 auto 40px;
  display: block;
  box-shadow: 0 4px 20px rgba(0,0,0,0.3);
}
.hero-buttons {
  display: flex;
  justify-content: center;
  gap: 15px;
  flex-wrap: wrap;
}
.btn {
  display: inline-block;
  padding: 14px 28px;
  border-radius: 8px;
  text-decoration: none;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.2s;
}
.btn-primary {
  background: #3b82f6;
  color: white !important;
}
.btn-primary:hover { background: #2563eb; }
.btn-outline {
  border: 2px solid #444;
  color: #ccc !important;
}
.btn-outline:hover { border-color: #3b82f6; color: #3b82f6 !important; }

.section {
  max-width: 700px;
  margin: 0 auto;
  padding: 60px 20px;
}
.section h2 {
  font-size: 1.75rem;
  margin-bottom: 30px;
  padding-bottom: 10px;
  border-bottom: 1px solid #333;
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
}
.nav-item {
  margin-bottom: 25px;
}
.nav-item a {
  display: block;
  padding: 20px 25px;
  border: 1px solid #333;
  border-radius: 10px;
  text-decoration: none;
  transition: all 0.2s;
}
.nav-item a:hover {
  border-color: #3b82f6;
  transform: translateX(5px);
}
.nav-title {
  font-size: 1.2rem;
  font-weight: 600;
  color: #fff;
  margin-bottom: 6px;
}
.nav-desc {
  font-size: 0.95rem;
  color: #888;
  line-height: 1.5;
}

.how-it-works {
  max-width: 700px;
  margin: 0 auto;
  padding: 40px 20px 60px;
}
.how-it-works h2 {
  font-size: 1.75rem;
  margin-bottom: 20px;
}
.how-it-works p {
  color: #aaa;
  margin-bottom: 20px;
  line-height: 1.7;
}
.verdict-table {
  width: 100%;
  margin: 20px 0;
  border-collapse: collapse;
}
.verdict-table th, .verdict-table td {
  padding: 12px 15px;
  text-align: left;
  border-bottom: 1px solid #333;
}
.verdict-table th {
  color: #888;
  font-weight: 500;
}
.code-block {
  background: #111;
  padding: 20px;
  border-radius: 8px;
  font-family: monospace;
  font-size: 14px;
  overflow-x: auto;
  margin: 25px 0;
}

footer.site-footer {
  text-align: center;
  padding: 40px 20px;
  border-top: 1px solid #333;
  margin-top: 40px;
}
footer a {
  color: #60a5fa;
  text-decoration: none;
}
footer a:hover { text-decoration: underline; }
</style>

<div class="hero">
  <h1>Losselot</h1>
  <p class="tagline">Audio forensics meets AI-assisted development</p>

  <img src="demo.gif" alt="Losselot Demo" class="hero-gif">

  <div class="hero-buttons">
    <a href="analyzer.html" class="btn btn-primary">Try in Browser</a>
    <a href="https://github.com/notactuallytreyanastasio/losselot" class="btn btn-outline">View on GitHub</a>
  </div>
</div>

<div class="section">
  <h2>Explore</h2>

  <ul class="nav-list">
    <li class="nav-item">
      <a href="analyzer.html">
        <div class="nav-title">Browser Analyzer</div>
        <div class="nav-desc">Drop an audio file and get instant analysis. Runs entirely in your browser via WebAssembly.</div>
      </a>
    </li>

    <li class="nav-item">
      <a href="demo/">
        <div class="nav-title">Decision Graph</div>
        <div class="nav-desc">Interactive visualization of every development decision. See how the project evolved through 87+ tracked nodes.</div>
      </a>
    </li>

    <li class="nav-item">
      <a href="spelunk-timeline.html">
        <div class="nav-title">Timeline View</div>
        <div class="nav-desc">Chronological walkthrough merging git commits with decision nodes. Filter by type, search, and trace history.</div>
      </a>
    </li>

    <li class="nav-item">
      <a href="spelunk-graph.html">
        <div class="nav-title">Graph Explorer</div>
        <div class="nav-desc">Force-directed graph visualization. Zoom, pan, and trace paths between connected decisions.</div>
      </a>
    </li>

    <li class="nav-item">
      <a href="spelunk-story.html">
        <div class="nav-title">The Story</div>
        <div class="nav-desc">Narrative walkthrough of how this project came to be. Chapters covering detection algorithms, the memory problem, and more.</div>
      </a>
    </li>
  </ul>
</div>

<div class="how-it-works">
  <h2>How It Works</h2>

  <p>When someone converts MP3 to FLAC, the removed frequencies don't come back. Losselot detects these scars using dual analysis:</p>

  <p><strong>Spectral Analysis</strong> uses FFT to detect frequency cutoffs. Lossy codecs remove high frequencies - 128kbps MP3 cuts at ~16kHz, 320kbps at ~20kHz.</p>

  <p><strong>Binary Analysis</strong> finds encoder signatures embedded in files. LAME headers, FFmpeg markers, re-encoding chains.</p>

  <table class="verdict-table">
    <tr><th>Score</th><th>Verdict</th><th>Meaning</th></tr>
    <tr><td>0-34</td><td>OK</td><td>Likely genuine lossless</td></tr>
    <tr><td>35-64</td><td>SUSPECT</td><td>Possibly transcoded</td></tr>
    <tr><td>65+</td><td>TRANSCODE</td><td>Definitely lossy origin</td></tr>
  </table>

  <h2>Quick Start</h2>

  <div class="code-block">
git clone https://github.com/notactuallytreyanastasio/losselot<br>
cd losselot && cargo build --release<br>
./target/release/losselot analyze ~/Music/album.flac<br>
./target/release/losselot serve ~/Music/  # Web UI
  </div>
</div>

<footer class="site-footer">
  <a href="https://github.com/notactuallytreyanastasio/losselot">GitHub</a>
</footer>
