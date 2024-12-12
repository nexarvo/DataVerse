/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useEffect, useState } from 'react';
import EnterKeyIcon from '../assets/enter-key-icon.svg';
import CommandIcon from '../assets/command-icon.svg';
import DropDown from './DropDown';
import {
  SELECT_COLUMN_FILTER_PLACEHOLDER,
  SELECT_OPERATION_FILTER_PLACEHOLDER,
  SELECT_VALUE_FILTER_PLACEHOLDER,
  NUMBER_FILTER_OPERATIONS,
  ENTER_VALUE_FILTER_PLACEHOLDER,
} from '../utils/consts';

interface QuickFilterDropDownsComponentProps {
  colums: any[];
  data: any[];
  setFilter?: (value: any) => void;
}

const QuickFilterDropDownsComponent: React.FC<
  QuickFilterDropDownsComponentProps
> = ({ colums, data, setFilter }) => {
  const [selectedColumn, setSelectedColumn] = useState<string | null>(null);
  const [selectedOperation, setSelectedOperation] = useState<string | null>(
    null,
  );
  const [selectedValue, setSelectedValue] = useState<string | null>(null);
  const [value, setValue] = useState<string>('');

  const handleColumnChange = (value: string) => {
    setSelectedColumn(value);
  };

  const handleOperationChange = (value: string) => {
    setSelectedOperation(value);
  };

  const handleValueChange = (value: string) => {
    setSelectedValue(value);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    // Detect Command + Enter key press
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      if (selectedColumn && selectedOperation && value.length > 0) {
        setFilter?.({
          column: selectedColumn,
          operation: selectedOperation,
          value: value,
        });
      }
    }
  };

  // Add event listener for keydown to handle Command + Enter
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
        if (selectedColumn && selectedOperation && value.length > 0) {
          setFilter?.({
            column: selectedColumn,
            operation: selectedOperation,
            value: value,
          });
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [selectedColumn, selectedOperation, value, setFilter]);

  return (
    <div className='flex flex-col items-center justify-center bg-gray-300 border border-gray-400 rounded-md shadow-lg mt-2 py-4'>
      {/* Columns drop down */}
      <DropDown
        dataList={colums}
        defaultText={SELECT_COLUMN_FILTER_PLACEHOLDER}
        isDropDownOpen={true}
        onChange={handleColumnChange}
        data={data}
        shouldRenderIcon={true}
      />
      <DropDown
        dataList={NUMBER_FILTER_OPERATIONS}
        defaultText={SELECT_OPERATION_FILTER_PLACEHOLDER}
        isDropDownOpen={false}
        onChange={handleOperationChange}
        data={data}
        shouldRenderIcon={false}
      />
      {/* Search Bar */}
      <div className='p-2'>
        <input
          type='text'
          placeholder={ENTER_VALUE_FILTER_PLACEHOLDER}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          className='w-64 px-3 py-2 text-sm text-black border border-gray-400 bg-gray-300 rounded focus:outline-none'
        />
      </div>
      {/* Drop Down for types */}
      {/* <DropDown
        dataList={colums}
        defaultText={SELECT_VALUE_FILTER_PLACEHOLDER}
        isDropDownOpen={false}
        onChange={handleValueChange}
        data={data}
        shouldRenderIcon={false}
      /> */}
      <div className='flex justify-between items-center w-full'>
        <span className='self-start mt-2 ml-4 text-gray-500 text-sm'>
          Add condition
        </span>
        {/* Conditionally render Apply div when input has at least 1 character */}
        {selectedColumn && selectedOperation && value.length > 0 && (
          <div className='flex items-center bg-blue-200/[0.9] rounded-md px-4 py-1 mr-3'>
            <span className='text-md text-blue-400 mr-2'>Apply</span>
            <div className='flex border border-gray-400 rounded-md h-8 w-12 items-center justify-center'>
              <img src={EnterKeyIcon} alt='' className='h-5 w-5' />
              <img src={CommandIcon} alt='' className='h-5 w-5' />
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default QuickFilterDropDownsComponent;
