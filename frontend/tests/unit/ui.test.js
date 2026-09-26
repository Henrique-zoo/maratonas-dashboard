import { describe, expect, test } from 'bun:test';

import {
  chooseLocationType,
  escapeHtml,
  formatDate,
  formatNumber,
  formatYearSpan,
  renderBarList,
  renderChip,
  renderEmptyState,
  renderErrorState,
  renderLineChart,
  renderMetricStrip,
  renderPageIntro,
  renderParticipationChart,
  renderStatGrid,
  renderTable,
  serialiseQuery,
  sumBy,
} from '../../app/src/lib/ui.js';

describe('UI formatting and safe HTML rendering', () => {
  test('escapes untrusted text before inserting it into markup', () => {
    expect(escapeHtml(`<&>"'`)).toBe('&lt;&amp;&gt;&quot;&#39;');

    const error = renderErrorState('<img src=x onerror=alert(1)>');
    expect(error).toContain('&lt;img src=x onerror=alert(1)&gt;');
    expect(error).not.toContain('<img src=x');

    const empty = renderEmptyState('No <rows>', 'Try "another" filter');
    expect(empty).toContain('No &lt;rows&gt;');
    expect(empty).toContain('Try &quot;another&quot; filter');
  });

  test('serialises only meaningful query values and preserves zero and false', () => {
    expect(
      serialiseQuery({
        missing: undefined,
        nil: null,
        empty: '',
        zero: 0,
        enabled: false,
        term: 'São Paulo / BR',
      }),
    ).toBe('?zero=0&enabled=false&term=S%C3%A3o+Paulo+%2F+BR');
    expect(serialiseQuery()).toBe('');
  });

  test('formats empty, single-year and multi-year values consistently', () => {
    expect(formatNumber(null)).toBe('0');
    expect(formatNumber(12_345)).toBe('12,345');
    expect(formatDate(null)).toBe('—');
    expect(formatDate('2025-05-10')).toBe('10 May 2025');
    expect(formatYearSpan([])).toBe('—');
    expect(formatYearSpan([2025])).toBe('2025');
    expect(formatYearSpan([2025, 2023, 2024])).toBe('2023–2025');
  });

  test('chooses the most useful available location level', () => {
    expect(chooseLocationType([])).toBe('Country');
    expect(chooseLocationType(['City', 'State'])).toBe('City');
    expect(chooseLocationType(['Region', 'State'])).toBe('Region');
    expect(chooseLocationType(['City', 'Country'])).toBe('Country');
  });

  test('renders empty, zero and proportional bar-list values without invalid widths', () => {
    expect(renderBarList([])).toContain('No location data yet');

    const bars = renderBarList([
      { label: 'Zero', value: 0 },
      { label: 'Half', value: 5 },
      { label: 'Full', value: 10 },
    ]);
    expect(bars).toContain('width: 0.00%');
    expect(bars).toContain('width: 50.00%');
    expect(bars).toContain('width: 100.00%');
  });

  test('renders all line-chart cardinalities and escapes labels', () => {
    expect(renderLineChart([])).toContain('No trend available');
    expect(renderLineChart([{ label: 2025, value: null }])).toContain('no numeric samples');
    expect(renderLineChart([{ label: '2025<script>', value: 2.5 }])).toContain('2025&lt;script&gt;');

    const chart = renderLineChart([
      { label: 2024, value: 3 },
      { label: 2025, value: 1 },
    ]);
    expect(chart).toContain('role="img"');
    expect(chart).toContain('Value over time');
    expect(chart).toContain('line-chart__axis');
    expect(chart).toContain('line-chart__value');
    expect(chart).toContain('M 64 34');
    expect(chart).toContain('L 656 266');
  });

  test('renders multiple series with an inverted rank axis and accessible points', () => {
    const chart = renderLineChart(
      [
        { label: 2024, best: 8, average: 10.5 },
        { label: 2025, best: 1, average: 4.25 },
      ],
      {
        yLabel: 'Rank (lower is better)',
        reverseY: true,
        series: [
          { key: 'best', label: 'Best rank' },
          { key: 'average', label: 'Average rank' },
        ],
      },
    );

    expect(chart).toContain('Best rank');
    expect(chart).toContain('Average rank');
    expect(chart).toContain('line-chart__path--2');
    expect(chart).toContain('aria-label="2025, Best rank: 1"');
    expect(chart).toContain('<desc>Best rank: 2024 8, 2025 1. Average rank: 2024 10.5, 2025 4.25</desc>');
  });

  test('renders proportional participation bars and clamps highlighted totals', () => {
    expect(renderParticipationChart([])).toContain('No location data yet');

    const chart = renderParticipationChart([
      { label: 'Brazil', total: 12, highlighted: 4, context: '3 teams' },
      { label: '<Chile>', total: 6, highlighted: 9, context: '2 teams' },
    ]);

    expect(chart).toContain('Female participants');
    expect(chart).toContain('Remaining participants');
    expect(chart).toContain('width: 100.00%');
    expect(chart).toContain('width: 50.00%');
    expect(chart).toContain('&lt;Chile&gt;');
    expect(chart).toContain('Chile&gt;: 6 participants, 6 female participants');
  });

  test('renders reusable cards and tables with escaped labels and explicit alignment', () => {
    expect(renderChip('<Open>', 'amber')).toContain('&lt;Open&gt;');

    const intro = renderPageIntro({
      eyebrow: 'Summary',
      title: '<Competition>',
      blurb: 'Latest "season"',
      meta: [renderChip('2025')],
      actions: '<a href="/safe">Open</a>',
    });
    expect(intro).toContain('&lt;Competition&gt;');
    expect(intro).toContain('Latest &quot;season&quot;');
    expect(intro).toContain('<a href="/safe">Open</a>');

    const stats = renderStatGrid([{ label: '<Teams>', value: 2, hint: 'Distinct' }]);
    expect(stats).toContain('&lt;Teams&gt;');
    expect(stats).toContain('<small>Distinct</small>');

    const table = renderTable({
      compact: true,
      columns: [{ label: 'Name' }, { label: 'Count', align: 'right' }],
      rows: [[{ value: '<strong>Team</strong>' }, { value: '2', align: 'right' }]],
    });
    expect(table).toContain('table-wrap--compact');
    expect(table).toContain('<th class="is-right">Count</th>');
    expect(table).toContain('data-label="Count"');
    expect(table).toContain('<strong>Team</strong>');

    const metrics = renderMetricStrip([{ label: '<Entries>', value: '3' }]);
    expect(metrics).toContain('&lt;Entries&gt;');
    expect(metrics).toContain('<strong>3</strong>');
  });

  test('sums numeric, missing and string values without producing NaN', () => {
    expect(sumBy([{ value: 2 }, {}, { value: '3' }], (item) => item.value)).toBe(5);
  });
});
