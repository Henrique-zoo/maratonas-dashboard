const numberFormatter = new Intl.NumberFormat('en-US');
const dateFormatter = new Intl.DateTimeFormat('en-GB', {
  day: '2-digit',
  month: 'short',
  year: 'numeric',
});

export function escapeHtml(value) {
  return String(value ?? '')
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

export function formatNumber(value) {
  return numberFormatter.format(Number(value || 0));
}

export function formatDate(value) {
  if (!value) {
    return '—';
  }

  return dateFormatter.format(new Date(`${value}T00:00:00`));
}

export function formatYearSpan(years = []) {
  if (!years.length) {
    return '—';
  }

  const sorted = [...years].sort((left, right) => left - right);
  return sorted[0] === sorted.at(-1) ? `${sorted[0]}` : `${sorted[0]}–${sorted.at(-1)}`;
}

export function serialiseQuery(params = {}) {
  const query = new URLSearchParams();

  Object.entries(params).forEach(([key, value]) => {
    if (value === undefined || value === null || value === '') {
      return;
    }

    query.set(key, String(value));
  });

  const raw = query.toString();
  return raw ? `?${raw}` : '';
}

export function sumBy(items = [], selector = (value) => value) {
  return items.reduce((accumulator, item) => accumulator + Number(selector(item) || 0), 0);
}

export function chooseLocationType(types = []) {
  if (!types.length) {
    return 'Country';
  }

  return types.includes('Country') ? 'Country' : types.includes('City') ? 'City' : types[0];
}

export function renderChip(label, tone = 'slate') {
  return `<span class="chip chip--${tone}">${escapeHtml(label)}</span>`;
}

export function renderPageIntro({ eyebrow, title, blurb, meta = [], actions = '' }) {
  return `
    <section class="page-intro">
      <div>
        ${eyebrow ? `<span class="eyebrow">${escapeHtml(eyebrow)}</span>` : ''}
        <h1>${escapeHtml(title)}</h1>
        <p>${escapeHtml(blurb)}</p>
        ${meta.length ? `<div class="meta-row">${meta.join('')}</div>` : ''}
      </div>
      ${actions ? `<div class="page-intro__actions">${actions}</div>` : ''}
    </section>
  `;
}

export function renderStatGrid(items = []) {
  return `
    <section class="stat-grid">
      ${items
        .map(
          (item) => `
            <article class="stat-card">
              <span>${escapeHtml(item.label)}</span>
              <strong>${escapeHtml(item.value)}</strong>
              ${item.hint ? `<small>${escapeHtml(item.hint)}</small>` : ''}
            </article>
          `,
        )
        .join('')}
    </section>
  `;
}

export function renderTable({ columns, rows, compact = false }) {
  return `
    <div class="table-wrap ${compact ? 'table-wrap--compact' : ''}">
      <table class="data-table">
        <thead>
          <tr>
            ${columns
              .map(
                (column) =>
                  `<th class="${column.align === 'right' ? 'is-right' : ''}">${escapeHtml(column.label)}</th>`,
              )
              .join('')}
          </tr>
        </thead>
        <tbody>
          ${rows
            .map(
              (row) => `
                <tr>
                  ${row
                    .map((cell, index) => {
                      const column = columns[index] || {};
                      return `<td class="${cell.align === 'right' ? 'is-right' : ''}" data-label="${escapeHtml(column.label || '')}">${cell.value}</td>`;
                    })
                    .join('')}
                </tr>
              `,
            )
            .join('')}
        </tbody>
      </table>
    </div>
  `;
}

export function renderBarList(
  items = [],
  { valueFormatter = formatNumber, labelFormatter = (item) => item.label } = {},
) {
  if (!items.length) {
    return renderEmptyState(
      'No location data yet',
      'Pick another season or location level to explore this slice.',
    );
  }

  const maxValue = Math.max(...items.map((item) => Number(item.value || 0)), 1);

  return `
    <div class="bar-list">
      ${items
        .map((item) => {
          const width = (Number(item.value || 0) / maxValue) * 100;
          return `
            <article class="bar-row">
              <div class="bar-row__header">
                <span>
                  ${escapeHtml(labelFormatter(item))}
                  ${item.subtitle ? `<small>${escapeHtml(item.subtitle)}</small>` : ''}
                </span>
                <strong>${escapeHtml(valueFormatter(item.value))}</strong>
              </div>
              <div class="bar-track">
                <span class="bar-track__fill" style="width: ${width.toFixed(2)}%"></span>
              </div>
            </article>
          `;
        })
        .join('')}
    </div>
  `;
}

function chartNumber(value) {
  return new Intl.NumberFormat('en-US', {
    maximumFractionDigits: 2,
  }).format(Number(value));
}

function finiteNumber(value) {
  if (value === null || value === undefined || value === '') {
    return null;
  }

  const numericValue = Number(value);
  return Number.isFinite(numericValue) ? numericValue : null;
}

function linePath(points, xForIndex, yForValue, key) {
  let started = false;

  return points
    .map((point, index) => {
      const value = finiteNumber(point[key]);

      if (value === null) {
        started = false;
        return null;
      }

      const command = started ? 'L' : 'M';
      started = true;
      return `${command} ${xForIndex(index)} ${yForValue(value)}`;
    })
    .filter(Boolean)
    .join(' ');
}

export function renderParticipationChart(
  items = [],
  {
    totalLabel = 'Participants',
    highlightedLabel = 'Female participants',
    remainderLabel = 'Remaining participants',
  } = {},
) {
  if (!items.length) {
    return renderEmptyState(
      'No location data yet',
      'Pick another season or location level to explore this slice.',
    );
  }

  const normalizedItems = items.map((item) => {
    const total = Math.max(0, finiteNumber(item.total) || 0);
    const highlighted = Math.min(total, Math.max(0, finiteNumber(item.highlighted) || 0));

    return { ...item, total, highlighted };
  });
  const maxTotal = Math.max(...normalizedItems.map((item) => item.total), 1);

  return `
    <figure class="participation-chart" aria-label="${escapeHtml(`${totalLabel} by location`)}">
      <figcaption class="chart-legend">
        <span class="chart-legend__item">
          <span class="chart-legend__swatch chart-legend__swatch--highlighted"></span>
          ${escapeHtml(highlightedLabel)}
        </span>
        <span class="chart-legend__item">
          <span class="chart-legend__swatch chart-legend__swatch--remainder"></span>
          ${escapeHtml(remainderLabel)}
        </span>
      </figcaption>
      <div class="participation-chart__rows">
        ${normalizedItems
          .map((item) => {
            const totalWidth = (item.total / maxTotal) * 100;
            const highlightedWidth = item.total ? (item.highlighted / item.total) * 100 : 0;
            const remainderWidth = 100 - highlightedWidth;
            const context = item.context ? ` · ${item.context}` : '';
            const accessibleLabel = `${item.label}: ${chartNumber(item.total)} ${totalLabel.toLowerCase()}, ${chartNumber(item.highlighted)} ${highlightedLabel.toLowerCase()}${context}`;

            return `
              <div class="participation-chart__row">
                <div class="participation-chart__header">
                  <span>
                    <strong>${escapeHtml(item.label)}</strong>
                    ${item.context ? `<small>${escapeHtml(item.context)}</small>` : ''}
                  </span>
                  <span>${escapeHtml(chartNumber(item.total))}</span>
                </div>
                <div class="participation-chart__track" role="img" aria-label="${escapeHtml(accessibleLabel)}">
                  <span class="participation-chart__total" style="width: ${totalWidth.toFixed(2)}%">
                    <span class="participation-chart__highlighted" style="width: ${highlightedWidth.toFixed(2)}%"></span>
                    <span class="participation-chart__remainder" style="width: ${remainderWidth.toFixed(2)}%"></span>
                  </span>
                </div>
              </div>
            `;
          })
          .join('')}
      </div>
    </figure>
  `;
}

export function renderLineChart(
  points = [],
  {
    yLabel = 'Value',
    series = [{ key: 'value', label: yLabel }],
    reverseY = false,
    includeZero = false,
  } = {},
) {
  if (!points.length) {
    return renderEmptyState(
      'No trend available',
      'This entity has no historical samples for the selected event window.',
    );
  }

  const normalizedSeries = series
    .map((item, index) => ({
      key: item.key,
      label: item.label || item.key,
      index,
    }))
    .filter((item) => points.some((point) => finiteNumber(point[item.key]) !== null));

  if (!normalizedSeries.length) {
    return renderEmptyState(
      'No trend available',
      'This entity has no numeric samples for the selected event window.',
    );
  }

  if (points.length === 1) {
    return `
      <div class="single-point">
        <strong>${escapeHtml(String(points[0].label))}</strong>
        ${normalizedSeries
          .map((item) => `<span>${escapeHtml(`${item.label}: ${chartNumber(points[0][item.key])}`)}</span>`)
          .join('')}
      </div>
    `;
  }

  const width = 680;
  const height = 320;
  const margin = { top: 34, right: 24, bottom: 54, left: 64 };
  const plotWidth = width - margin.left - margin.right;
  const plotHeight = height - margin.top - margin.bottom;
  const values = normalizedSeries.flatMap((item) =>
    points.map((point) => finiteNumber(point[item.key])).filter((value) => value !== null),
  );
  let min = Math.min(...values);
  let max = Math.max(...values);

  if (includeZero) {
    min = Math.min(0, min);
    max = Math.max(0, max);
  }

  if (reverseY && min > 1) {
    min = 1;
  }

  if (min === max) {
    const padding = Math.max(Math.abs(min) * 0.1, 1);
    min = Math.max(reverseY ? 1 : 0, min - padding);
    max += padding;
  }

  const span = max - min;
  const xForIndex = (index) => margin.left + (index * plotWidth) / (points.length - 1);
  const yForValue = (value) => {
    const ratio = (value - min) / span;
    return reverseY ? margin.top + ratio * plotHeight : margin.top + (1 - ratio) * plotHeight;
  };
  const tickCount = 5;
  const ticks = Array.from({ length: tickCount }, (_, index) => min + (span * index) / (tickCount - 1));
  const grid = ticks
    .map((value) => {
      const y = yForValue(value);
      return `
        <line class="line-chart__grid" x1="${margin.left}" x2="${width - margin.right}" y1="${y}" y2="${y}"></line>
        <text class="line-chart__tick" x="${margin.left - 12}" y="${y + 4}" text-anchor="end">${escapeHtml(chartNumber(value))}</text>
      `;
    })
    .join('');
  const xLabels = points
    .map(
      (point, index) =>
        `<text class="line-chart__tick" x="${xForIndex(index)}" y="${height - 20}" text-anchor="middle">${escapeHtml(String(point.label))}</text>`,
    )
    .join('');
  const plottedSeries = normalizedSeries
    .map((item) => {
      const path = linePath(points, xForIndex, yForValue, item.key);
      const dots = points
        .map((point, pointIndex) => {
          const value = finiteNumber(point[item.key]);

          if (value === null) {
            return '';
          }

          const x = xForIndex(pointIndex);
          const y = yForValue(value);
          const valueLabelY = y + (item.index % 2 === 0 ? -11 : 18);
          const pointLabel = `${point.label}, ${item.label}: ${chartNumber(value)}`;

          return `
            <g class="line-chart__point line-chart__point--${item.index + 1}">
              <circle cx="${x}" cy="${y}" r="5" tabindex="0" aria-label="${escapeHtml(pointLabel)}">
                <title>${escapeHtml(pointLabel)}</title>
              </circle>
              <text class="line-chart__value" x="${x}" y="${valueLabelY}" text-anchor="middle">${escapeHtml(chartNumber(value))}</text>
            </g>
          `;
        })
        .join('');

      return `
        <path d="${path}" class="line-chart__path line-chart__path--${item.index + 1}"></path>
        ${dots}
      `;
    })
    .join('');
  const description = normalizedSeries
    .map(
      (item) =>
        `${item.label}: ${points
          .map((point) => `${point.label} ${chartNumber(point[item.key])}`)
          .join(', ')}`,
    )
    .join('. ');

  return `
    <figure class="line-chart">
      <figcaption class="chart-legend">
        ${normalizedSeries
          .map(
            (item) => `
              <span class="chart-legend__item">
                <span class="chart-legend__swatch chart-legend__swatch--series-${item.index + 1}"></span>
                ${escapeHtml(item.label)}
              </span>
            `,
          )
          .join('')}
      </figcaption>
      <svg viewBox="0 0 ${width} ${height}" role="img" aria-label="${escapeHtml(`${yLabel} over time`)}">
        <title>${escapeHtml(`${yLabel} over time`)}</title>
        <desc>${escapeHtml(description)}</desc>
        ${grid}
        <line class="line-chart__axis" x1="${margin.left}" x2="${margin.left}" y1="${margin.top}" y2="${height - margin.bottom}"></line>
        <line class="line-chart__axis" x1="${margin.left}" x2="${width - margin.right}" y1="${height - margin.bottom}" y2="${height - margin.bottom}"></line>
        <text class="line-chart__axis-title" transform="translate(18 ${margin.top + plotHeight / 2}) rotate(-90)" text-anchor="middle">${escapeHtml(yLabel)}</text>
        ${xLabels}
        ${plottedSeries}
      </svg>
    </figure>
  `;
}

export function renderEmptyState(title, body, action = '') {
  return `
    <section class="empty-state">
      <strong>${escapeHtml(title)}</strong>
      <p>${escapeHtml(body)}</p>
      ${action}
    </section>
  `;
}

export function renderErrorState(message) {
  return `
    <section class="empty-state empty-state--error">
      <strong>Something went off-script</strong>
      <p>${escapeHtml(message)}</p>
      <a class="button button--ghost" href="/" data-link>Return home</a>
    </section>
  `;
}

export function renderMetricStrip(metrics = []) {
  return `
    <div class="metric-strip">
      ${metrics
        .map(
          (metric) => `
            <div class="metric-pill">
              <span>${escapeHtml(metric.label)}</span>
              <strong>${escapeHtml(metric.value)}</strong>
            </div>
          `,
        )
        .join('')}
    </div>
  `;
}
