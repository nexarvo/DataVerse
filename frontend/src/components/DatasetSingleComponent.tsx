/* eslint-disable @typescript-eslint/no-explicit-any */
import React from 'react';
import SpreadsheetIcon from '../assets/spreadsheet-icon.svg';
import StarIcon from '../assets/star-icon-fill-yellow.svg';
import TrustedIcon from '../assets/trusted-icon.svg';

interface DatasetSingleComponentProps {
  dataset: any;
}

const DatasetSingleComponent: React.FC<DatasetSingleComponentProps> = ({
  dataset,
}) => {
  return (
    <div className='py-2 px-1 hover:bg-third'>
      <div className='flex items-center'>
        <img src={SpreadsheetIcon} alt='$' className='h-4 w-4 mr-2' />
        <span className='text-sm text-white'>{dataset.title}</span>
        <img src={StarIcon} alt='$' className='h-3 w-3 ml-2' />
      </div>
      <p className='text-xs text-gray-500'>{dataset.description}</p>
    </div>
  );
};

export default DatasetSingleComponent;
