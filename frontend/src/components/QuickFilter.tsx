/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';
import FilterIconBlue from '../assets/filter-icon-blue.svg';
import FilterIconGray from '../assets/filter-icon-gray.svg';
import SingleQuickFilter from './SingleQuickFilter';
import { applyTransformations } from '../services/transformations';
import { ApplyTransformationParam } from '../utils/apiTypes';

interface QuickFilterProps {
  dataset_id: string;
  colums: any[];
  data: any[];
}

const QuickFilter: React.FC<QuickFilterProps> = ({
  dataset_id,
  colums,
  data,
}) => {
  const [isHovered, setIsHovered] = useState(false);
  const [filters, setFilters] = useState<number[]>([0]); // Tracks filter indices, starts with one filter
  const [isFilterVisible, setIsFilterVisible] = useState<boolean>(false);

  const handleMouseEnter = () => setIsHovered(true);
  const handleMouseLeave = () => setIsHovered(false);

  const handleFilterAdded = async (filter: ApplyTransformationParam) => {
    // Add a new filter
    setFilters([...filters, filters.length]);
    const transformation = { type: 'filter', action: 'filter', params: filter };
    try {
      await applyTransformations(dataset_id, transformation);
    } catch (err) {
      console.log('Transformation Failed: ', err);
    }
  };

  const handleFilterDeleted = (index: number) => {
    // Remove the filter by index
    setFilters(filters.filter((_, i) => i !== index));
  };

  const hanleFilterClick = () => {
    setIsFilterVisible((prev) => !prev);
  };

  return (
    <div className='flex'>
      <div
        className={`flex flex-row cursor-pointer hover:bg-blue-950 ${filters.length > 1 ? 'bg-blue-950' : isHovered ? 'bg-blue-950' : null} px-2 rounded-sm mr-2`}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        onClick={hanleFilterClick}
        style={{ height: '1.6rem' }}
      >
        <img
          src={
            filters.length > 1
              ? FilterIconBlue
              : isHovered
                ? FilterIconBlue
                : FilterIconGray
          }
          alt='$'
          className='h-5 w-5 mr-1'
        />
        <span
          className={`text-sm ${
            filters.length > 1
              ? 'text-blue-400'
              : isHovered
                ? 'text-blue-400'
                : 'text-gray-400'
          } py-0`}
        >
          {/* minus 1 because there is a filter which will be used to add new filter */}
          {filters.length > 1 ? `${filters.length - 1}` : 'Filters'}
        </span>
      </div>

      {/* Render all filters in a row */}
      <div className='flex flex-wrap overflow-x-auto items-start'>
        {isFilterVisible &&
          filters.map((filterIndex) => (
            <SingleQuickFilter
              key={filterIndex}
              colums={colums}
              data={data}
              handelSetFilter={handleFilterAdded}
              defaultText={
                !(filters.length === 1) && filterIndex + 1 === filters.length
                  ? '+'
                  : 'Select a column...'
              }
              shouldShowFilter={filters.length === 1}
            />
          ))}
      </div>
    </div>
  );
};

export default QuickFilter;
