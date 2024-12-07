import React from 'react';

interface DatasetListProps {
  tab: string;
}

const DatasetList: React.FC<DatasetListProps> = ({ tab }) => {
  // Sample dataset list (you can replace this with actual data fetching logic)
  const datasets = [
    { id: 1, name: 'Dataset 1', description: 'Description for dataset 1' },
    { id: 2, name: 'Dataset 2', description: 'Description for dataset 2' },
    { id: 3, name: 'Dataset 3', description: 'Description for dataset 3' },
  ];

  return (
    <div>
      {datasets.map((dataset) => (
        <div
          key={dataset.id}
          className='p-4 border-b border-gray-700 hover:bg-gray-800 rounded'
        >
          <h4 className='text-white'>{dataset.name}</h4>
          <p className='text-gray-400'>{dataset.description}</p>
        </div>
      ))}
    </div>
  );
};

export default DatasetList;
