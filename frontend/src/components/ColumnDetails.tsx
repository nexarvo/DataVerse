/* eslint-disable @typescript-eslint/no-explicit-any */
import React from 'react';

interface ColumnDetailsProps {
  header: any;
}

const ColumnDetails: React.FC<ColumnDetailsProps> = ({ header }) => {
  return (
    <div className='border border-gray-400 rounded-sm p-4'>
      <div className='flex items-center'>
        <span className='text-lg text-white'>{header.title}</span>
        <span className='text-gray-400 text-xs border border-gray-600 rounded-sm ml-2 p-1'>
          {header.type}
        </span>
      </div>
      <span className='text-sm text-gray-400'>{header.description}</span>
    </div>
  );
};

export default ColumnDetails;
