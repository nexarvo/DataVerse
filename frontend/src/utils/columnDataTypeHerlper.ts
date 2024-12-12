/* eslint-disable @typescript-eslint/no-explicit-any */
// Function to detect if a string can be converted to a number
export const isNumber = (value: string) =>
  !isNaN(parseFloat(value)) && isFinite(value);

// Function to detect if a string is a valid date
export const isDate = (value: string) => {
  const parsedDate = new Date(value);
  return !isNaN(parsedDate.getTime());
};

// Function to check if the value is a boolean
export const isBoolean = (value: any) => typeof value === 'boolean';

// Function to check if the value is null or undefined
export const isNullOrUndefined = (value: any) =>
  value === null || value === undefined;
