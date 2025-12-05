//! HTML report generation with D3.js visualizations

use crate::analyzer::{AnalysisResult, Verdict};
use crate::report::Summary;
use std::io::{self, Write};

pub fn write<W: Write>(writer: &mut W, results: &[AnalysisResult]) -> io::Result<()> {
    let summary = Summary::from_results(results);

    // Sort by score descending
    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by(|a, b| b.combined_score.cmp(&a.combined_score));

    // Build JSON data for D3.js
    let json_data = build_json_data(&sorted_results);

    // Write the full HTML document
    write!(writer, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Losselot Analysis Report</title>
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        :root {{
            --bg: #0d1117;
            --card: #161b22;
            --border: #30363d;
            --text: #e6edf3;
            --dim: #7d8590;
            --ok: #3fb950;
            --suspect: #d29922;
            --transcode: #f85149;
            --error: #6e7681;
            --accent: #58a6ff;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.5;
        }}
        .container {{ max-width: 1600px; margin: 0 auto; padding: 2rem; }}

        /* Header */
        .header {{
            display: flex;
            align-items: center;
            gap: 1rem;
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border);
        }}
        .logo {{
            font-size: 2.5rem;
            font-weight: 800;
            background: linear-gradient(135deg, var(--accent), #a371f7);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .subtitle {{ color: var(--dim); font-size: 1rem; }}

        /* Stats Row */
        .stats {{
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 1rem;
            margin-bottom: 2rem;
        }}
        .stat {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.5rem;
            text-align: center;
        }}
        .stat-value {{ font-size: 3rem; font-weight: 700; line-height: 1; }}
        .stat-label {{ color: var(--dim); font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em; margin-top: 0.5rem; }}
        .stat.ok .stat-value {{ color: var(--ok); }}
        .stat.suspect .stat-value {{ color: var(--suspect); }}
        .stat.transcode .stat-value {{ color: var(--transcode); }}

        /* Charts Grid */
        .charts {{
            display: grid;
            grid-template-columns: 350px 1fr;
            gap: 1.5rem;
            margin-bottom: 2rem;
        }}
        .chart-card {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.5rem;
        }}
        .chart-title {{
            font-size: 1rem;
            font-weight: 600;
            margin-bottom: 1rem;
            color: var(--dim);
        }}
        #donut-chart {{ display: flex; justify-content: center; }}
        #spectrum-chart {{ width: 100%; }}

        /* Donut legend */
        .donut-legend {{
            display: flex;
            justify-content: center;
            gap: 1.5rem;
            margin-top: 1rem;
            flex-wrap: wrap;
        }}
        .legend-item {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 0.875rem;
        }}
        .legend-dot {{
            width: 12px;
            height: 12px;
            border-radius: 50%;
        }}

        /* File Details Panel */
        .detail-panel {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.5rem;
            margin-bottom: 2rem;
            display: none;
        }}
        .detail-panel.active {{ display: block; }}
        .detail-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1rem;
        }}
        .detail-filename {{
            font-family: 'SF Mono', 'Fira Code', monospace;
            font-size: 1.1rem;
            color: var(--accent);
        }}
        .detail-close {{
            background: none;
            border: none;
            color: var(--dim);
            cursor: pointer;
            font-size: 1.5rem;
            padding: 0.5rem;
        }}
        .detail-close:hover {{ color: var(--text); }}
        .detail-grid {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 1.5rem;
        }}
        #file-spectrum {{ width: 100%; }}

        /* Table */
        .table-container {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 12px;
            overflow: hidden;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        th, td {{ padding: 0.875rem 1rem; text-align: left; }}
        th {{
            background: rgba(255,255,255,0.03);
            font-weight: 600;
            font-size: 0.75rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--dim);
            border-bottom: 1px solid var(--border);
        }}
        tr {{ cursor: pointer; transition: background 0.15s; }}
        tr:hover td {{ background: rgba(255,255,255,0.02); }}
        tr.selected td {{ background: rgba(88,166,255,0.1); }}
        td {{ border-bottom: 1px solid var(--border); }}
        tr:last-child td {{ border-bottom: none; }}

        .verdict {{
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.25rem 0.75rem;
            border-radius: 20px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
        }}
        .verdict.ok {{ background: rgba(63,185,80,0.15); color: var(--ok); }}
        .verdict.suspect {{ background: rgba(210,153,34,0.15); color: var(--suspect); }}
        .verdict.transcode {{ background: rgba(248,81,73,0.15); color: var(--transcode); }}
        .verdict.error {{ background: rgba(110,118,129,0.15); color: var(--error); }}

        .score-cell {{ display: flex; align-items: center; gap: 0.75rem; }}
        .score-bar {{
            width: 80px;
            height: 8px;
            background: rgba(255,255,255,0.1);
            border-radius: 4px;
            overflow: hidden;
        }}
        .score-fill {{ height: 100%; border-radius: 4px; }}
        .score-fill.low {{ background: var(--ok); }}
        .score-fill.medium {{ background: var(--suspect); }}
        .score-fill.high {{ background: var(--transcode); }}

        .flags {{ display: flex; flex-wrap: wrap; gap: 0.25rem; }}
        .flag {{
            background: rgba(255,255,255,0.05);
            padding: 0.2rem 0.5rem;
            border-radius: 4px;
            font-size: 0.7rem;
            font-family: 'SF Mono', monospace;
            color: var(--dim);
        }}
        .filepath {{
            max-width: 250px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            font-family: 'SF Mono', monospace;
            font-size: 0.8rem;
        }}
        .mono {{ font-family: 'SF Mono', monospace; font-size: 0.85rem; }}
        .dim {{ color: var(--dim); }}

        /* Spectrum bars */
        .bar-ok {{ fill: var(--ok); }}
        .bar-warning {{ fill: var(--suspect); }}
        .bar-danger {{ fill: var(--transcode); }}

        /* Tooltip */
        .tooltip {{
            position: absolute;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 0.75rem 1rem;
            font-size: 0.875rem;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.15s;
            z-index: 1000;
            box-shadow: 0 4px 12px rgba(0,0,0,0.3);
        }}
        .tooltip.visible {{ opacity: 1; }}

        /* Footer */
        .footer {{
            margin-top: 2rem;
            padding-top: 1rem;
            border-top: 1px solid var(--border);
            color: var(--dim);
            font-size: 0.875rem;
            text-align: center;
        }}
        .footer a {{ color: var(--accent); text-decoration: none; }}
        .footer a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div>
                <div class="logo">Losselot</div>
                <div class="subtitle">Audio Transcode Detection Report</div>
            </div>
        </div>

        <div class="stats">
            <div class="stat ok">
                <div class="stat-value">{ok}</div>
                <div class="stat-label">Clean</div>
            </div>
            <div class="stat suspect">
                <div class="stat-value">{suspect}</div>
                <div class="stat-label">Suspect</div>
            </div>
            <div class="stat transcode">
                <div class="stat-value">{transcode}</div>
                <div class="stat-label">Transcode</div>
            </div>
            <div class="stat">
                <div class="stat-value">{total}</div>
                <div class="stat-label">Total Files</div>
            </div>
        </div>

        <div class="charts">
            <div class="chart-card">
                <div class="chart-title">Verdict Distribution</div>
                <div id="donut-chart"></div>
                <div class="donut-legend">
                    <div class="legend-item"><div class="legend-dot" style="background: var(--ok)"></div>Clean</div>
                    <div class="legend-item"><div class="legend-dot" style="background: var(--suspect)"></div>Suspect</div>
                    <div class="legend-item"><div class="legend-dot" style="background: var(--transcode)"></div>Transcode</div>
                </div>
            </div>
            <div class="chart-card">
                <div class="chart-title">Score Distribution by File</div>
                <div id="spectrum-chart"></div>
            </div>
        </div>

        <div class="detail-panel" id="detail-panel">
            <div class="detail-header">
                <div class="detail-filename" id="detail-filename">filename.mp3</div>
                <button class="detail-close" onclick="closeDetail()">&times;</button>
            </div>
            <div class="detail-grid">
                <div>
                    <div class="chart-title">Frequency Band Energy</div>
                    <div id="file-spectrum"></div>
                </div>
                <div>
                    <div class="chart-title">Analysis Details</div>
                    <div id="file-details"></div>
                </div>
            </div>
        </div>

        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th>Verdict</th>
                        <th>Score</th>
                        <th>Bitrate</th>
                        <th>Spectral</th>
                        <th>Binary</th>
                        <th>Encoder</th>
                        <th>Flags</th>
                        <th>File</th>
                    </tr>
                </thead>
                <tbody id="results-table">
                </tbody>
            </table>
        </div>

        <div class="footer">
            Generated by <a href="https://github.com/notactuallytreyanastasio/losselot" target="_blank">Losselot</a>
        </div>
    </div>

    <div class="tooltip" id="tooltip"></div>

    <script>
    const data = {json_data};

    const colors = {{
        ok: '#3fb950',
        suspect: '#d29922',
        transcode: '#f85149',
        error: '#6e7681'
    }};

    // Donut Chart
    function drawDonutChart() {{
        const width = 280, height = 280;
        const radius = Math.min(width, height) / 2;

        const svg = d3.select('#donut-chart')
            .append('svg')
            .attr('width', width)
            .attr('height', height)
            .append('g')
            .attr('transform', `translate(${{width/2}},${{height/2}})`);

        const pieData = [
            {{ label: 'Clean', value: data.summary.ok, color: colors.ok }},
            {{ label: 'Suspect', value: data.summary.suspect, color: colors.suspect }},
            {{ label: 'Transcode', value: data.summary.transcode, color: colors.transcode }}
        ].filter(d => d.value > 0);

        const pie = d3.pie().value(d => d.value).sort(null);
        const arc = d3.arc().innerRadius(radius * 0.6).outerRadius(radius * 0.9);
        const arcHover = d3.arc().innerRadius(radius * 0.6).outerRadius(radius * 0.95);

        const arcs = svg.selectAll('path')
            .data(pie(pieData))
            .enter()
            .append('path')
            .attr('d', arc)
            .attr('fill', d => d.data.color)
            .attr('stroke', '#0d1117')
            .attr('stroke-width', 2)
            .style('cursor', 'pointer')
            .on('mouseover', function(event, d) {{
                d3.select(this).transition().duration(100).attr('d', arcHover);
                showTooltip(event, `${{d.data.label}}: ${{d.data.value}} files`);
            }})
            .on('mouseout', function() {{
                d3.select(this).transition().duration(100).attr('d', arc);
                hideTooltip();
            }});

        // Center text
        svg.append('text')
            .attr('text-anchor', 'middle')
            .attr('dy', '-0.2em')
            .style('font-size', '2.5rem')
            .style('font-weight', '700')
            .style('fill', '#e6edf3')
            .text(data.summary.total);

        svg.append('text')
            .attr('text-anchor', 'middle')
            .attr('dy', '1.5em')
            .style('font-size', '0.875rem')
            .style('fill', '#7d8590')
            .text('files');
    }}

    // Score Distribution Chart
    function drawScoreChart() {{
        const container = document.getElementById('spectrum-chart');
        const margin = {{ top: 20, right: 30, bottom: 60, left: 50 }};
        const width = container.clientWidth - margin.left - margin.right;
        const height = 300 - margin.top - margin.bottom;

        const svg = d3.select('#spectrum-chart')
            .append('svg')
            .attr('width', width + margin.left + margin.right)
            .attr('height', height + margin.top + margin.bottom)
            .append('g')
            .attr('transform', `translate(${{margin.left}},${{margin.top}})`);

        const x = d3.scaleBand()
            .domain(data.files.map((d, i) => i))
            .range([0, width])
            .padding(0.2);

        const y = d3.scaleLinear()
            .domain([0, 100])
            .range([height, 0]);

        // Grid lines
        svg.append('g')
            .attr('class', 'grid')
            .call(d3.axisLeft(y).tickSize(-width).tickFormat(''))
            .style('stroke-dasharray', '3,3')
            .style('stroke-opacity', 0.1);

        // Threshold lines
        [35, 65].forEach(thresh => {{
            svg.append('line')
                .attr('x1', 0)
                .attr('x2', width)
                .attr('y1', y(thresh))
                .attr('y2', y(thresh))
                .attr('stroke', thresh === 65 ? colors.transcode : colors.suspect)
                .attr('stroke-dasharray', '5,5')
                .attr('stroke-opacity', 0.5);
        }});

        // Bars
        svg.selectAll('.bar')
            .data(data.files)
            .enter()
            .append('rect')
            .attr('class', d => {{
                if (d.score >= 65) return 'bar bar-danger';
                if (d.score >= 35) return 'bar bar-warning';
                return 'bar bar-ok';
            }})
            .attr('x', (d, i) => x(i))
            .attr('width', x.bandwidth())
            .attr('y', d => y(d.score))
            .attr('height', d => height - y(d.score))
            .attr('rx', 3)
            .style('cursor', 'pointer')
            .on('mouseover', function(event, d) {{
                d3.select(this).style('opacity', 0.8);
                showTooltip(event, `${{d.filename}}: ${{d.score}}%`);
            }})
            .on('mouseout', function() {{
                d3.select(this).style('opacity', 1);
                hideTooltip();
            }})
            .on('click', (event, d) => showDetail(d));

        // Y axis
        svg.append('g')
            .call(d3.axisLeft(y).ticks(5).tickFormat(d => d + '%'))
            .style('color', '#7d8590');

        // X axis label
        svg.append('text')
            .attr('x', width / 2)
            .attr('y', height + 45)
            .attr('text-anchor', 'middle')
            .style('fill', '#7d8590')
            .style('font-size', '0.875rem')
            .text('Files (sorted by score)');
    }}

    // File Detail Spectrum
    function drawFileSpectrum(file) {{
        const container = document.getElementById('file-spectrum');
        container.innerHTML = '';

        if (!file.spectral) return;

        const margin = {{ top: 20, right: 30, bottom: 60, left: 60 }};
        const width = container.clientWidth - margin.left - margin.right;
        const height = 250 - margin.top - margin.bottom;

        const svg = d3.select('#file-spectrum')
            .append('svg')
            .attr('width', width + margin.left + margin.right)
            .attr('height', height + margin.top + margin.bottom)
            .append('g')
            .attr('transform', `translate(${{margin.left}},${{margin.top}})`);

        const bands = [
            {{ label: 'Full', value: file.spectral.rms_full, range: '20Hz-20kHz' }},
            {{ label: 'Mid-High', value: file.spectral.rms_mid_high, range: '10-15kHz' }},
            {{ label: 'High', value: file.spectral.rms_high, range: '15-20kHz' }},
            {{ label: 'Upper', value: file.spectral.rms_upper, range: '17-20kHz' }},
            {{ label: 'Ultrasonic', value: file.spectral.rms_ultrasonic, range: '20-22kHz' }}
        ];

        const x = d3.scaleBand()
            .domain(bands.map(d => d.label))
            .range([0, width])
            .padding(0.3);

        const minVal = Math.min(...bands.map(d => d.value), -80);
        const maxVal = Math.max(...bands.map(d => d.value), 0);

        const y = d3.scaleLinear()
            .domain([minVal - 10, maxVal + 10])
            .range([height, 0]);

        // Grid
        svg.append('g')
            .call(d3.axisLeft(y).tickSize(-width).tickFormat(''))
            .style('stroke-dasharray', '3,3')
            .style('stroke-opacity', 0.1);

        // Bars
        svg.selectAll('.bar')
            .data(bands)
            .enter()
            .append('rect')
            .attr('x', d => x(d.label))
            .attr('width', x.bandwidth())
            .attr('y', d => d.value >= 0 ? y(d.value) : y(0))
            .attr('height', d => Math.abs(y(d.value) - y(0)))
            .attr('rx', 4)
            .attr('fill', (d, i) => {{
                if (i === 4) return d.value < -60 ? colors.transcode : colors.ok;
                return colors.ok;
            }})
            .on('mouseover', function(event, d) {{
                showTooltip(event, `${{d.range}}: ${{d.value.toFixed(1)}} dB`);
            }})
            .on('mouseout', hideTooltip);

        // Zero line
        svg.append('line')
            .attr('x1', 0)
            .attr('x2', width)
            .attr('y1', y(0))
            .attr('y2', y(0))
            .attr('stroke', '#7d8590')
            .attr('stroke-dasharray', '3,3');

        // Axes
        svg.append('g')
            .attr('transform', `translate(0,${{height}})`)
            .call(d3.axisBottom(x))
            .style('color', '#7d8590');

        svg.append('g')
            .call(d3.axisLeft(y).ticks(6).tickFormat(d => d + ' dB'))
            .style('color', '#7d8590');
    }}

    // Show file details
    function showDetail(file) {{
        const panel = document.getElementById('detail-panel');
        panel.classList.add('active');
        document.getElementById('detail-filename').textContent = file.filename;

        drawFileSpectrum(file);

        const detailsHtml = `
            <div style="display: grid; gap: 1rem;">
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem;">
                    <div style="color: var(--dim);">Verdict:</div>
                    <div><span class="verdict ${{file.verdict.toLowerCase()}}">${{file.verdict}}</span></div>
                    <div style="color: var(--dim);">Score:</div>
                    <div>${{file.score}}%</div>
                    <div style="color: var(--dim);">Bitrate:</div>
                    <div>${{file.bitrate}} kbps</div>
                    <div style="color: var(--dim);">Encoder:</div>
                    <div style="font-family: monospace;">${{file.encoder || 'Unknown'}}</div>
                    ${{file.lowpass ? `<div style="color: var(--dim);">Lowpass:</div><div>${{file.lowpass}} Hz</div>` : ''}}
                </div>
                ${{file.spectral ? `
                <div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--border);">
                    <div style="font-weight: 600; margin-bottom: 0.5rem;">Spectral Analysis</div>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem; font-size: 0.875rem;">
                        <div style="color: var(--dim);">Upper Drop:</div>
                        <div style="color: ${{file.spectral.upper_drop > 15 ? 'var(--transcode)' : 'var(--ok)'}}">${{file.spectral.upper_drop.toFixed(1)}} dB</div>
                        <div style="color: var(--dim);">Ultrasonic Drop:</div>
                        <div style="color: ${{file.spectral.ultrasonic_drop > 25 ? 'var(--transcode)' : 'var(--ok)'}}">${{file.spectral.ultrasonic_drop.toFixed(1)}} dB</div>
                        <div style="color: var(--dim);">Flatness (19-21k):</div>
                        <div style="color: ${{file.spectral.ultrasonic_flatness < 0.3 ? 'var(--transcode)' : 'var(--ok)'}}">${{file.spectral.ultrasonic_flatness.toFixed(3)}}</div>
                    </div>
                </div>
                ` : ''}}
                ${{file.flags.length > 0 ? `
                <div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--border);">
                    <div style="font-weight: 600; margin-bottom: 0.5rem;">Flags</div>
                    <div class="flags">${{file.flags.map(f => `<span class="flag">${{f}}</span>`).join('')}}</div>
                </div>
                ` : ''}}
            </div>
        `;
        document.getElementById('file-details').innerHTML = detailsHtml;

        // Highlight table row
        document.querySelectorAll('#results-table tr').forEach(tr => tr.classList.remove('selected'));
        const row = document.querySelector(`#results-table tr[data-file="${{file.filename}}"]`);
        if (row) row.classList.add('selected');
    }}

    function closeDetail() {{
        document.getElementById('detail-panel').classList.remove('active');
        document.querySelectorAll('#results-table tr').forEach(tr => tr.classList.remove('selected'));
    }}

    // Tooltip
    function showTooltip(event, text) {{
        const tooltip = document.getElementById('tooltip');
        tooltip.textContent = text;
        tooltip.classList.add('visible');
        tooltip.style.left = (event.pageX + 10) + 'px';
        tooltip.style.top = (event.pageY - 10) + 'px';
    }}

    function hideTooltip() {{
        document.getElementById('tooltip').classList.remove('visible');
    }}

    // Build table
    function buildTable() {{
        const tbody = document.getElementById('results-table');
        data.files.forEach(file => {{
            const scoreClass = file.score >= 65 ? 'high' : file.score >= 35 ? 'medium' : 'low';
            const flagsHtml = file.flags.length > 0
                ? file.flags.map(f => `<span class="flag">${{f}}</span>`).join('')
                : '<span class="dim">—</span>';

            const tr = document.createElement('tr');
            tr.setAttribute('data-file', file.filename);
            tr.innerHTML = `
                <td><span class="verdict ${{file.verdict.toLowerCase()}}">${{file.verdict}}</span></td>
                <td>
                    <div class="score-cell">
                        <div class="score-bar"><div class="score-fill ${{scoreClass}}" style="width: ${{file.score}}%"></div></div>
                        ${{file.score}}%
                    </div>
                </td>
                <td class="mono">${{file.bitrate}}k</td>
                <td class="dim">${{file.spectral_score}}%</td>
                <td class="dim">${{file.binary_score}}%</td>
                <td class="mono">${{file.encoder || '—'}}</td>
                <td class="flags">${{flagsHtml}}</td>
                <td class="filepath" title="${{file.filepath}}">${{file.filename}}</td>
            `;
            tr.onclick = () => showDetail(file);
            tbody.appendChild(tr);
        }});
    }}

    // Initialize
    drawDonutChart();
    drawScoreChart();
    buildTable();
    </script>
</body>
</html>
"#,
        ok = summary.ok,
        suspect = summary.suspect,
        transcode = summary.transcode,
        total = summary.total,
        json_data = json_data
    )?;

    Ok(())
}

fn build_json_data(results: &[&AnalysisResult]) -> String {
    let files: Vec<String> = results.iter().map(|r| {
        let spectral = if let Some(ref s) = r.spectral_details {
            format!(r#"{{
                "rms_full": {:.2},
                "rms_mid_high": {:.2},
                "rms_high": {:.2},
                "rms_upper": {:.2},
                "rms_ultrasonic": {:.2},
                "upper_drop": {:.2},
                "ultrasonic_drop": {:.2},
                "ultrasonic_flatness": {:.4}
            }}"#,
                s.rms_full, s.rms_mid_high, s.rms_high, s.rms_upper,
                s.rms_ultrasonic, s.upper_drop, s.ultrasonic_drop, s.ultrasonic_flatness
            )
        } else {
            "null".to_string()
        };

        let flags: Vec<String> = r.flags.iter().map(|f| format!("\"{}\"", f)).collect();

        format!(r#"{{
            "filename": "{}",
            "filepath": "{}",
            "verdict": "{}",
            "score": {},
            "spectral_score": {},
            "binary_score": {},
            "bitrate": {},
            "encoder": "{}",
            "lowpass": {},
            "flags": [{}],
            "spectral": {}
        }}"#,
            json_escape(&r.file_name),
            json_escape(&r.file_path),
            r.verdict,
            r.combined_score,
            r.spectral_score,
            r.binary_score,
            r.bitrate,
            json_escape(&r.encoder),
            r.lowpass.map(|l| l.to_string()).unwrap_or_else(|| "null".to_string()),
            flags.join(","),
            spectral
        )
    }).collect();

    let ok_count = results.iter().filter(|r| r.verdict == Verdict::Ok).count();
    let suspect_count = results.iter().filter(|r| r.verdict == Verdict::Suspect).count();
    let transcode_count = results.iter().filter(|r| r.verdict == Verdict::Transcode).count();

    format!(r#"{{
        "summary": {{
            "total": {},
            "ok": {},
            "suspect": {},
            "transcode": {}
        }},
        "files": [{}]
    }}"#,
        results.len(),
        ok_count,
        suspect_count,
        transcode_count,
        files.join(",")
    )
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
