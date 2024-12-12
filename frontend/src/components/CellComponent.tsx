/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useEffect, useState } from 'react';
import { getDatasetById } from '../services/datasets';
import TableComponent from './TableComponent';
import Dataset from '../utils/types';
import DownRightIcon from '../assets/down-right-icon.svg';
import CommentsIcon from '../assets/comment-icon.svg';
import MenuIcon from '../assets/menu-icon.svg';
import QuickFilter from './QuickFilter';

interface CellComponentProps {
  datasetsList: any[];
}

const CellComponent: React.FC<CellComponentProps> = ({ datasetsList }) => {
  const [label, setLabel] = useState('Table 1'); // Default label
  const [isEditing, setIsEditing] = useState(false);
  const [dataset, setDataset] = useState<Dataset | null>(null);
  const [selectedDatasetId, setSelectedDatasetId] = useState<string>(
    datasetsList.length > 0 ? datasetsList[0].id : null,
  );

  const handleLabelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(e.target.value);
  };

  const handleBlur = () => {
    setIsEditing(false);
  };

  const handleDatasetChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedDatasetId(e.target.value);
  };

  useEffect(() => {
    const fetchDatasets = async () => {
      try {
        const dataset = await getDatasetById(selectedDatasetId);
        setDataset(dataset);
      } catch (error) {
        console.error('Error fetching dataset:', error);
      }
    };

    fetchDatasets();
  }, [selectedDatasetId]);

  return (
    <div className='flex flex-col mb-8'>
      <div className='flex flex-col justify-center items-center place-self-end'>
        <img src={CommentsIcon} alt='$' className='h-4 w-4 mt-9 ml-2' />
        <img src={MenuIcon} alt='$' className='h-4 w-4 mt-2 ml-2 self-end' />
      </div>

      <div className='relative cell p-4 bg-accent border border-blue-200 rounded-md mr-7'>
        {/* Editable Label */}
        <div className='absolute -top-2 left-4 bg-dark rounded-md text-gray-400 text-xs'>
          {isEditing ? (
            <input
              type='text'
              value={label}
              onChange={handleLabelChange}
              onBlur={handleBlur}
              className='bg-transparent border-none text-white focus:outline-none'
              autoFocus
            />
          ) : (
            <span
              onClick={() => setIsEditing(true)}
              className='cursor-pointer'
              title='Click to edit'
            >
              {label}
            </span>
          )}
        </div>
        <div className='flex'>
          {/* Dropdown to select dataset */}
          <select
            className='bg-gray-700 text-white rounded-sm mb-4 w-20 h-5 text-green-300 bg-green-500/[.2] mr-4'
            onChange={handleDatasetChange}
            value={selectedDatasetId || ''}
          >
            {datasetsList.map((dataset) => (
              <option
                key={dataset.id}
                value={dataset.id}
                className='text-green-200'
              >
                {dataset.file_name}
              </option>
            ))}
          </select>
          {/* Quick filter Option */}
          <div className='max-w-4xl'>
            <QuickFilter
              dataset_id={selectedDatasetId}
              colums={dataset?.latest_preview.headers}
              data={dataset?.latest_preview.preview}
            />
          </div>
        </div>
        <hr className='border-t border-blue-200 mb-8 w-full' />
        {/* Table */}
        <TableComponent
          headers={dataset?.latest_preview.headers}
          data={dataset?.latest_preview.preview}
        />
      </div>
      <div className='flex items-center'>
        <img src={DownRightIcon} alt='$' className='h-3 w-3 mt-2 ml-2' />
        <span className='text-blue-400 text-xs mt-3 ml-2 bg-blue-950 px-1 rounded-sm'>
          table_result
        </span>
      </div>
    </div>
  );
};

export default CellComponent;
