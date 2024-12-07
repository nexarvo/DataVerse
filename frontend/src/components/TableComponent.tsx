/* eslint-disable @typescript-eslint/no-explicit-any */
import React from 'react';

interface TableComponentProps {
  data: any;
  headers: any;
}

const TableComponent: React.FC<TableComponentProps> = ({ headers, data }) => {
  return (
    <div className='overflow-hidden rounded-sm border border-gray-600 bg-accent'>
      {/* Set a fixed height for the table container */}
      <div className='max-h-80 overflow-y-auto'>
        <table className='min-w-full table-auto'>
          <thead className='bg-gray-800 sticky top-0'>
            <tr className='bg-dark'>
              <th className='px-2 text-left text-sm text-gray-400 border border-gray-600'></th>
              {headers.map((obj) => (
                <th
                  key={obj.title}
                  className='px-2 text-left text-sm text-gray-400 border border-gray-600'
                >
                  {obj.title}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {data.map((item, index) => (
              <tr key={index} className='bg-accent'>
                {/* Row Number */}
                <td className='text-sm text-center text-gray-400 border border-gray-600 bg-dark'>
                  {index + 1}
                </td>
                <td className='px-2 text-xs text-white border border-gray-600 hover:bg-third'>
                  {item.name}
                </td>
                <td className='px-2 text-xs text-white border border-gray-600 hover:bg-third'>
                  {item.size}
                </td>
                <td className='px-2 text-xs text-white border border-gray-600 hover:bg-third'>
                  {item.type}
                </td>
                <td className='px-2 text-xs text-white border border-gray-600 hover:bg-third'>
                  {item.updated}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default TableComponent;
