/* eslint-disable @typescript-eslint/no-explicit-any */

// Function to detect if a string can be converted to a number
export const isNumber = (value: any): boolean =>
  typeof value === 'number' || (!isNaN(parseFloat(value)) && isFinite(value));

// Function to detect if a string is a valid date
export const isDate = (value: any): boolean => {
  if (value instanceof Date) {
    return !isNaN(value.getTime());
  }
  const parsedDate = new Date(value);
  return !isNaN(parsedDate.getTime());
};

// Function to check if the value is a boolean
export const isBoolean = (value: any): boolean =>
  typeof value === 'boolean' || value === 'true' || value === 'false';

// Function to check if the value is null or undefined
export const isNullOrUndefined = (value: any): boolean =>
  value === null || value === undefined;

// Function to check if the value is a string
export const isString = (value: any): boolean =>
  typeof value === 'string' || value instanceof String;
