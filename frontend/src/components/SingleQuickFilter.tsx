/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';
import QuickFilterDropDownsComponent from './QuickFilterDropDownsComponent';

interface SingleQuickFilterProps {
  colums: any[];
  data: any[];
  handelSetFilter: (filter: any) => void;
  defaultText: string;
  shouldShowFilter: boolean;
}

const SingleQuickFilter: React.FC<SingleQuickFilterProps> = ({
  colums,
  data,
  handelSetFilter,
  defaultText,
  shouldShowFilter,
}) => {
  const [filterSelection, setFilterSelection] = useState<any | null>(null);
  const [isFilterVisible, setIsFilterVisible] = useState(shouldShowFilter);

  const handleSelect = (selection: any) => {
    setFilterSelection(selection);
    handelSetFilter(selection);
  };

  const toggleFilterVisibility = () => {
    setIsFilterVisible((prev) => !prev);
  };

  return (
    <div className='flex flex-col mx-1 my-1'>
      <div
        className={`flex items-center cursor-pointer px-2 bg-gray-300 rounded-sm`}
      >
        <span className={`text-sm`} onClick={toggleFilterVisibility}>
          {filterSelection
            ? filterSelection.column +
              ' ' +
              filterSelection.operation +
              ' ' +
              filterSelection.value
            : `${defaultText}`}
        </span>
      </div>
      {/* Dropdown Content */}
      <div className='absolute top-8 z-50'>
        {!filterSelection && isFilterVisible && (
          <QuickFilterDropDownsComponent
            colums={colums}
            data={data}
            setFilter={handleSelect}
          />
        )}
      </div>
    </div>
  );
};

export default SingleQuickFilter;
