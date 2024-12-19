export const SELECT_COLUMN_FILTER_PLACEHOLDER = 'Select a column';
export const SELECT_OPERATION_FILTER_PLACEHOLDER = 'Select an operation';
export const SELECT_VALUE_FILTER_PLACEHOLDER = 'Select a value';
export const ENTER_VALUE_FILTER_PLACEHOLDER = 'Enter a value...';

export const enum CellTypes {
  table = 'table',
  sql = 'sql',
  python = 'python',
  chart = 'chart',
  markdown = 'markdown',
  pivot = 'pivot',
}

export const FILTER_OPERATIONS = {
  number: [
    'Is one of',
    'Is not one of',
    'Is equal to',
    'Is not equal to',
    'Is not null',
    'Is null',
    'Greater than',
    'Greater than or equal to',
    'Less than',
    'Less than or equal to',
  ],
  text: [
    'Contains',
    'Does not contain',
    'Is equal to',
    'Is not equal to',
    'Starts with',
    'Does not start with',
    'Ends with',
    'Does not end with',
    'Is null',
    'Is not null',
  ],
  date: [
    'Is on',
    'Is not on',
    'Is before',
    'Is after',
    'Is on or before',
    'Is on or after',
    'Is between',
    'Is not between',
    'Is null',
    'Is not null',
  ],
  boolean: ['Is true', 'Is false', 'Is null', 'Is not null'],
  list: [
    'Contains any of',
    'Contains all of',
    'Does not contain any of',
    'Is equal to',
    'Is not equal to',
    'Is null',
    'Is not null',
  ],
  general: [
    'Is equal to',
    'Is not equal to',
    'Is null',
    'Is not null',
    'Is one of',
    'Is not one of',
  ],
};

export const CHART_TYPES = [
  {
    type: 'bar',
    name: 'Bar Chart',
    description: 'Display categorical data with rectangular bars.',
    customizations: [
      'Orientation (Vertical / Horizontal)',
      'Bar Style (Stacked or Grouped)',
      'Axis Labels (Customizable)',
      'Bar Colors',
      'Tooltips',
      'Data Labels',
    ],
  },
  {
    type: 'line',
    name: 'Line Chart',
    description: 'Show trends over time or continuous data.',
    customizations: [
      'Line Style (Solid, Dashed, Dotted)',
      'Markers on Data Points',
      'Multiple Lines',
      'Axis Scaling',
      'Tooltips',
      'Data Labels',
    ],
  },
  {
    type: 'scatter',
    name: 'Scatter Plot',
    description: 'Show the relationship between two continuous variables.',
    customizations: [
      'Point Shape',
      'Color Encoding',
      'Tooltips',
      'Gridlines',
      'Regression Line',
    ],
  },
  {
    type: 'histogram',
    name: 'Histogram',
    description: 'Display the distribution of a dataset.',
    customizations: ['Bin Size', 'Bar Style', 'Color Scheme'],
  },
  {
    type: 'heatmap',
    name: 'Heatmap',
    description: 'Display matrix data with color gradients.',
    customizations: ['Color Scale', 'Axis Labels', 'Cell Size', 'Legend'],
  },
  {
    type: 'boxPlot',
    name: 'Box Plot',
    description: 'Display distribution statistics (min, max, quartiles).',
    customizations: ['Outlier Handling', 'Box Style', 'Axis Labels'],
  },
  {
    type: 'pie',
    name: 'Pie Chart',
    description: 'Display parts of a whole (percentages).',
    customizations: [
      'Donut Style',
      'Label Positioning',
      'Slice Colors',
      'Exploded Slices',
    ],
  },
  {
    type: 'area',
    name: 'Area Chart',
    description: 'Display quantitative data with filled areas beneath lines.',
    customizations: ['Stacked Area', 'Opacity', 'Line Style'],
  },
] as const;

export const CHART_INPUTS = [
  {
    type: 'bar',
    name: 'Bar Chart',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'yAxis', label: 'Y-Axis', type: 'column', required: true },
      {
        id: 'sort',
        label: 'Sort',
        type: 'dropdown',
        options: ['Ascending', 'Descending'],
        required: false,
      },
    ],
  },
  {
    type: 'line',
    name: 'Line Chart',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'yAxis', label: 'Y-Axis', type: 'column', required: true },
      { id: 'group', label: 'Group By', type: 'column', required: false },
    ],
  },
  {
    type: 'scatter',
    name: 'Scatter Plot',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'yAxis', label: 'Y-Axis', type: 'column', required: true },
      { id: 'size', label: 'Point Size', type: 'column', required: false },
    ],
  },
  {
    type: 'histogram',
    name: 'Histogram',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'bins', label: 'Number of Bins', type: 'number', required: false },
    ],
  },
  {
    type: 'heatmap',
    name: 'Heatmap',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'yAxis', label: 'Y-Axis', type: 'column', required: true },
      { id: 'value', label: 'Value', type: 'column', required: true },
    ],
  },
  {
    type: 'boxplot',
    name: 'Box Plot',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'yAxis', label: 'Y-Axis', type: 'column', required: true },
      { id: 'group', label: 'Group By', type: 'column', required: false },
    ],
  },
  {
    type: 'pie',
    name: 'Pie Chart',
    inputs: [
      { id: 'labels', label: 'Labels', type: 'column', required: true },
      { id: 'values', label: 'Values', type: 'column', required: true },
    ],
  },
  {
    type: 'area',
    name: 'Area Chart',
    inputs: [
      { id: 'xAxis', label: 'X-Axis', type: 'column', required: true },
      { id: 'yAxis', label: 'Y-Axis', type: 'column', required: true },
      { id: 'group', label: 'Group By', type: 'column', required: false },
    ],
  },
];
