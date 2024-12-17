import React from 'react';

import AIBlueIcon from '../assets/ai-blue-icon.svg';
import TableBlueIcon from '../assets/table-blue-icon.svg';
import ChartBlueIcon from '../assets/chart-blue-icon.svg';
import InputBlueIcon from '../assets/input-blue-icon.svg';
import PythonBlueIcon from '../assets/python-blue-icon.svg';
import MarkdownBlueIcon from '../assets/markdown-blue-icon.svg';
import MoreBlueIcon from '../assets/more-hollow-blue-icon.svg';
import PivotBlueIcon from '../assets/pivot-table-blue-icon.svg';
import DataBlueIcon from '../assets/data-blue-icon.svg';
import SQLBlueIcon from '../assets/sql-blue-icon.svg';

const CellOptionsComponent: React.FC = () => {
  return (
    <div className='flex bg-gray-800 text-white p-4 w-3/5 border border-blue-300 rounded-md drop-shadow-lg self-center items-center'>
      <button className='mx-2'>
        <img src={AIBlueIcon} alt='' className='h-10 w-10' />
      </button>
      {/* Vertical Line */}
      <div className='border-l border-blue-300 h-12 mx-2'></div>{' '}
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={SQLBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Query</span>
        </div>
      </button>
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={PythonBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Python</span>
        </div>
      </button>
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={MarkdownBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Markdown</span>
        </div>
      </button>
      {/* Vertical Line */}
      <div className='border-l border-blue-300 h-12 mx-2'></div>{' '}
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={TableBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Table</span>
        </div>
      </button>
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={ChartBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Chart</span>
        </div>
      </button>
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={PivotBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Pivot</span>
        </div>
      </button>
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={InputBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Input</span>
        </div>
      </button>
      {/* Vertical Line */}
      <div className='border-l border-blue-300 h-12 mx-2'></div>{' '}
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={DataBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>Data</span>
        </div>
      </button>
      <button className='px-4 py-1'>
        <div className='flex flex-col items-center'>
          <img src={MoreBlueIcon} alt='' className='h-6 w-6' />
          <span className='text-xs text-gray-300'>More</span>
        </div>
      </button>
    </div>
  );
};

export default CellOptionsComponent;
