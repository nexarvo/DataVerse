/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';
import SpreadsheetIcon from '../assets/spreadsheet-icon.svg';
import StarIcon from '../assets/star-icon-fill-yellow.svg';
import PreviewIcon from '../assets/preview-icon.svg';
import ColumnsIcon from '../assets/columns-icon.svg';
import RefreshIcon from '../assets/refresh-icon.svg';
import TableComponent from './TableComponent';
import { Tab } from '@headlessui/react';
import ColumnDetails from './ColumnDetails';

interface DatasetPreviewProps {
  dataset: any;
}

const DatasetPreview: React.FC<DatasetPreviewProps> = ({ dataset }) => {
  const [selectedTab, setSelectedTab] = useState('Preview');
  // Sample data for the preview table
  const headers = [
    { title: 'Name', type: 'VARCHAR', description: 'Name of the dataset.' },
    {
      title: 'Size',
      type: 'NUMBER',
      description: 'The size of dataset (e.g. 60MB).',
    },
    {
      title: 'Type',
      type: 'VARCHAR',
      description: 'Type of the dataset (e.g. CSV).',
    },
    {
      title: 'Last Updated',
      type: 'DATETIME',
      description: 'The last date on which this dataset was updated',
    },
  ];
  const data = [
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
    { name: 'Dataset 1', size: '5MB', type: 'CSV', updated: '2024-12-05' },
    { name: 'Dataset 2', size: '12MB', type: 'JSON', updated: '2024-12-03' },
    { name: 'Dataset 3', size: '8MB', type: 'XML', updated: '2024-12-01' },
    { name: 'Dataset 4', size: '3MB', type: 'CSV', updated: '2024-12-07' },
    { name: 'Dataset 5', size: '7MB', type: 'JSON', updated: '2024-11-28' },
  ];

  return (
    <div className='bg-accent p-6'>
      <div className='flex items-center'>
        <img src={SpreadsheetIcon} alt='$' className='h-5 w-5 mr-2' />
        <span className='text-lg font-bold text-white'>{dataset.title}</span>
        {dataset.isFavorite ? (
          <img src={StarIcon} alt='$' className='h-4 w-4 ml-2 mb-1' />
        ) : null}
      </div>
      <p className='text-sm text-gray-400 mb-6'>{dataset.description}</p>

      <div className='flex items-center'>
        {/* Tabs */}
        <Tab.Group
          selectedIndex={selectedTab === 'Preview' ? 0 : 1}
          onChange={(index) => {
            const tabNames = ['Preview', 'Columns'];
            setSelectedTab(tabNames[index]);
          }}
          className='mb-2'
        >
          <Tab.List className='flex'>
            <Tab
              className={({ selected }) =>
                selected
                  ? 'text-sm text-white border rounded-sm border-fourth p-1'
                  : 'text-sm text-fourth border rounded-sm p-1 bg-dark border-transparent'
              }
              as='button'
              onFocus={(e) => e.target.blur()}
            >
              <div className='flex items-center'>
                <img src={PreviewIcon} alt='' className='h-5 w-5' />
                <div className='flex items-center ml-1'>Preview</div>
              </div>
            </Tab>
            <Tab
              className={({ selected }) =>
                selected
                  ? 'text-sm text-white border rounded-sm border-fourth p-1'
                  : 'text-sm text-fourth border rounded-sm p-1 bg-dark border-transparent'
              }
              as='button'
              onFocus={(e) => e.target.blur()}
            >
              <div className='flex items-center'>
                <img src={ColumnsIcon} alt='' className='h-5 w-5' />
                <div className='flex items-center ml-1'>
                  Columns ({headers.length})
                </div>
              </div>
            </Tab>
          </Tab.List>
        </Tab.Group>

        <div className='flex items-center mb-2 ml-4'>
          <img src={RefreshIcon} alt='' className='h-4 w-4' />
          <span className='text-gray-400 text-xs'>Refreshed 4 hrs ago</span>
        </div>
      </div>

      {selectedTab === 'Preview' ? (
        <TableComponent headers={headers} data={data} />
      ) : (
        headers.map((obj) => <ColumnDetails header={obj} />)
      )}
    </div>
  );
};

export default DatasetPreview;
