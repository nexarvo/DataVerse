/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';

import {
  isNumber,
  isDate,
  isBoolean,
  isNullOrUndefined,
} from '../utils/columnDataTypeHerlper';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faHashtag,
  faCalendar,
  faCheck,
  faQuestion,
  faA,
} from '@fortawesome/free-solid-svg-icons';

interface DropDownProps {
  dataList: any[];
  defaultText: string;
  isDropDownOpen: boolean;
  onChange?: (value: string) => void;
  data: any[];
  shouldRenderIcon: boolean;
}

const DropDown: React.FC<DropDownProps> = ({
  dataList,
  defaultText,
  isDropDownOpen = false,
  onChange,
  data,
  shouldRenderIcon = false,
}) => {
  const [selectedDataNode, setSelectedDataNode] = useState<string>(
    defaultText || '',
  );
  const [isOpen, setIsOpen] = useState<boolean>(isDropDownOpen);
  const [searchTerm, setSearchTerm] = useState<string>('');
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);

  const handleMouseEnter = (index: number) => {
    setHoveredIndex(index);
  };

  const handleMouseLeave = () => {
    setHoveredIndex(null);
  };

  const filteredDataList = dataList?.filter((dataNode: any) =>
    String(dataNode).toLowerCase().includes(searchTerm.toLowerCase()),
  );

  const handleSelect = (dataNode: string) => {
    setSelectedDataNode(dataNode);
    setIsOpen(false); // Close the dropdown after selection
    if (onChange) {
      onChange(dataNode); // Trigger callback
    }
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
    <div className='relative mx-4 w-64 h-8 my-1'>
      {/* Dropdown Button */}
      <div
        className='flex items-center justify-between px-4 py-1 bg-gray-300 text-white border border-gray-400 rounded cursor-pointer'
        onClick={() => setIsOpen(!isOpen)}
      >
        <span className='text-black'>{selectedDataNode}</span>
        <svg
          className={`ml-2 w-4 h-4 text-black transition-transform ${
            isOpen ? 'rotate-180' : ''
          }`}
          xmlns='http://www.w3.org/2000/svg'
          fill='none'
          viewBox='0 0 24 24'
          stroke='currentColor'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            strokeWidth={2}
            d='M19 9l-7 7-7-7'
          />
        </svg>
      </div>

      {/* Dropdown Options */}
      {isOpen && (
        <div className='absolute bg-gray-300 text-white rounded shadow-lg z-10 w-full max-h-56 overflow-y-auto custom-scrollbar'>
          {/* Search Bar */}
          <div className='p-2'>
            <input
              type='text'
              placeholder='Search...'
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className='w-full px-3 py-2 text-sm text-gray-900 rounded focus:outline-none'
            />
          </div>

          {/* Options List */}
          {filteredDataList?.length > 0 ? (
            filteredDataList.map((dataNode, index) => (
              <div
                key={dataNode}
                onClick={() => handleSelect(dataNode)}
                className='px-4 py-2 cursor-pointer text-black hover:bg-fourth hover:text-white'
                onMouseEnter={() => handleMouseEnter(index)}
                onMouseLeave={handleMouseLeave}
              >
                {shouldRenderIcon ? (
                  <span
                    className={`mr-2 ${hoveredIndex === index ? 'text-white' : 'text-gray-500'}`}
                  >
                    {getIconForDataType(data[0][index])}
                  </span>
                ) : null}
                {dataNode}
              </div>
            ))
          ) : (
            <div className='px-4 py-2 text-gray-300'>No results found</div>
          )}
        </div>
      )}
    </div>
  );
};

export default DropDown;
