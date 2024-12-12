import React, { useEffect, useState } from 'react';
import CellComponent from '../components/CellComponent';
import { getDatasets } from '../services/datasets';
import { v4 as uuidv4 } from 'uuid';

const NotebookPage: React.FC = () => {
  // State to hold datasets and cells
  const [datasets, setDatasets] = useState([]);
  const [cells, setCells] = useState<{ id: string }[]>([
    { id: uuidv4() }, // Initial cell
  ]);

  useEffect(() => {
    const fetchDatasets = async () => {
      try {
        const fetchedDatasets = await getDatasets();
        setDatasets(fetchedDatasets);
      } catch (error) {
        console.error('Error fetching datasets:', error);
      }
    };

    fetchDatasets();
  }, []);

  const addCell = () => {
    setCells([...cells, { id: uuidv4() }]);
  };

  return (
    <div className='p-4 max-w-full h-full mx-auto bg-dark pl-11 custom-scrollbar'>
      <h1 className='text-4xl font-bold text-gray-400 mb-4'>
        Untitled Project
      </h1>
      <div className='flex flex-col items-start'>
        <span className='text-gray-400 text-sm mb-4'>Add Description...</span>
        <label className='text-gray-400 text-xs mb-11'>
          + Add project filter
        </label>
      </div>
      <div className=''>
        {cells.map((cell) => (
          <CellComponent key={cell.id} datasetsList={datasets} />
        ))}
      </div>
      {/* Add button to create new cell */}
      <button
        className='mt-4 p-2 bg-blue-500 text-white rounded-md'
        onClick={addCell}
      >
        Add Cell
      </button>
    </div>
  );
};

export default NotebookPage;
