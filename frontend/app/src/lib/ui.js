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
                <span>${escapeHtml(labelFormatter(item))}</span>
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

export function renderLineChart(points = [], { yLabel = 'Average rank' } = {}) {
  if (!points.length) {
    return renderEmptyState(
      'No trend available',
      'This entity has no historical samples for the selected event window.',
    );
  }

  if (points.length === 1) {
    return `
      <div class="single-point">
        <strong>${escapeHtml(String(points[0].label))}</strong>
        <span>${escapeHtml(`${yLabel}: ${points[0].value.toFixed(2)}`)}</span>
      </div>
    `;
  }

  const width = 560;
  const height = 220;
  const padding = 28;
  const values = points.map((point) => Number(point.value));
  const min = Math.min(...values);
  const max = Math.max(...values);
  const span = max - min || 1;

  const path = points
    .map((point, index) => {
      const x = padding + (index * (width - padding * 2)) / (points.length - 1);
      const y = height - padding - ((point.value - min) / span) * (height - padding * 2);
      return `${index === 0 ? 'M' : 'L'} ${x} ${y}`;
    })
    .join(' ');

  const dots = points
    .map((point, index) => {
      const x = padding + (index * (width - padding * 2)) / (points.length - 1);
      const y = height - padding - ((point.value - min) / span) * (height - padding * 2);

      return `
        <g>
          <circle cx="${x}" cy="${y}" r="5"></circle>
          <text x="${x}" y="${height - 6}" text-anchor="middle">${escapeHtml(String(point.label))}</text>
        </g>
      `;
    })
    .join('');

  return `
    <div class="line-chart">
      <svg viewBox="0 0 ${width} ${height}" role="img" aria-label="${escapeHtml(yLabel)} line chart">
        <path d="${path}" class="line-chart__path"></path>
        ${dots}
      </svg>
    </div>
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
