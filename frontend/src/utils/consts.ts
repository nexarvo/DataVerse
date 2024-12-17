export const SELECT_COLUMN_FILTER_PLACEHOLDER = 'Select a column';
export const SELECT_OPERATION_FILTER_PLACEHOLDER = 'Select an operation';
export const SELECT_VALUE_FILTER_PLACEHOLDER = 'Select a value';
export const ENTER_VALUE_FILTER_PLACEHOLDER = 'Enter a value...';

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
