/* eslint-disable @typescript-eslint/no-explicit-any */
import React from 'react';

import {
  isNumber,
  isDate,
  isBoolean,
  isNullOrUndefined,
} from '../utils/columnDataTypeHerlper';

// Import Font Awesome icon library
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faA,
  faHashtag,
  faCalendar,
  faCheck,
  faQuestion,
} from '@fortawesome/free-solid-svg-icons';

interface TableComponentProps {
  data: any;
  headers: any;
}

const TableComponent: React.FC<TableComponentProps> = ({ headers, data }) => {
  // Combined function to handle alignment and width based on value type
  const getDynamicStyle = (value: any) => {
    let alignment = 'text-left'; // Default alignment
    let minWidth = 'min-w-[100px]'; // Default width

    if (isNumber(value)) {
      alignment = 'text-right';
      minWidth = 'min-w-[100px]';
    } else if (isDate(value)) {
      alignment = 'text-center';
      minWidth = 'min-w-[150px]';
    } else if (isBoolean(value)) {
      alignment = 'text-center';
      minWidth = '60px';
    } else if (isNullOrUndefined(value)) {
      alignment = 'text-left';
      minWidth = '80px';
    } else if (typeof value === 'string' && value.length > 20) {
      minWidth = '300px';
    }

    return { alignment, minWidth };
  };

  // Function to format the value if it is a complex type (like object or array)
  const formatValue = (value: any) => {
    if (Array.isArray(value) || typeof value === 'object') {
      return JSON.stringify(value); // For arrays and objects, return a stringified version
    }
    return value; // Otherwise, return the value as-is
  };

  const getIconForDataType = (value: any) => {
    if (isNumber(value)) {
      return <FontAwesomeIcon icon={faHashtag} />;
    } else if (isDate(value)) {
      return <FontAwesomeIcon icon={faCalendar} />;
    } else if (isBoolean(value)) {
      return <FontAwesomeIcon icon={faCheck} />;
    } else if (isNullOrUndefined(value)) {
      return <FontAwesomeIcon icon={faQuestion} />;
    } else {
      return <FontAwesomeIcon icon={faA} />;
    }
  };

  return (
    <div className='overflow-hidden max-w-fit rounded-sm border border-gray-600 bg-accent'>
      {/* Set a fixed height for the table container */}
      <div className='max-h-80 overflow-y-auto max-w-2xl overflow-x-auto custom-scrollbar'>
        <table className='min-w-full table-auto table-layout-auto'>
          {' '}
          {/* Add table-layout-fixed */}
          <thead className='bg-gray-800 sticky top-0'>
            <tr className='bg-dark'>
              <th className='px-2 text-left text-xs text-gray-400 border border-gray-600'></th>
              {headers?.map((obj: any, index: number) => {
                const { minWidth } = getDynamicStyle(data[0][index]);
                return (
                  <th
                    key={obj}
                    className={`px-2 text-left text-xs font-medium text-gray-400 border border-gray-600 text-left ${minWidth}`}
                  >
                    <span className='mr-2'>
                      {getIconForDataType(data[0][index])}
                    </span>
                    {obj}
                  </th>
                );
              })}
            </tr>
          </thead>
          <tbody>
            {data?.map((item: any, index: number) => (
              <tr key={index} className='bg-accent'>
                {/* Row Number */}
                <td className='px-4 text-sm text-center text-gray-400 border border-gray-600 bg-dark'>
                  {index + 1}
                </td>
                {item.map((obj: any, idx: number) => {
                  const { alignment } = getDynamicStyle(obj);
                  return (
                    <td
                      key={`${index}-${idx}`}
                      className={`px-2 text-xs text-gray-300 border border-gray-600 hover:bg-third ${alignment} whitespace-nowrap`}
                    >
                      {formatValue(obj)}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default TableComponent;
