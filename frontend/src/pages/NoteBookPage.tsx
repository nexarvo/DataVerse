import React, { useEffect, useState } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import CellComponent from '../components/CellComponent';
import { getDatasets } from '../services/datasets';
import {
  getCellById,
  getCells,
  mapFetchedCellsToCellState,
} from '../services/cellService';
import { RootState } from '../app/store'; // Update with your store path
import {
  addCell,
  setCells,
  setDataframeMetadataList,
  setDatasets,
} from '../app/slices/notebookSlice';
import { getDataframesMetadata } from '../services/dataframe';
import LeftNavBar from '../components/LeftNavBar';
import TopBar from '../components/TopBar';

const NotebookPage: React.FC = () => {
  const dispatch = useDispatch();

  // State to track if the LeftNavBar is collapsed
  const [isCollapsed, setIsCollapsed] = useState(false);

  // Toggle function to collapse/expand the LeftNavBar
  const toggleNavBar = () => {
    setIsCollapsed(!isCollapsed);
  };

  const { cells, datasets, title, dataframeMetadataList } = useSelector(
    (state: RootState) => state.notebook,
  );

  const handleDatasetChange = async (cellId: string) => {
    try {
      const fetchedCell = await getCellById(cellId);
      const fetchedDataframesMetadataList = await getDataframesMetadata();

      // Update the existing cell in the array
      const updatedCells = cells.map((cell) =>
        cell.id === fetchedCell.id ? fetchedCell : cell,
      );

      dispatch(setCells(updatedCells));
      dispatch(setDataframeMetadataList(fetchedDataframesMetadataList));
    } catch (error) {
      console.error('Error fetching cell:', error);
    }
  };

  useEffect(() => {
    const fetchCells = async () => {
      try {
        const fetchedCells = await getCells();
        dispatch(setCells(mapFetchedCellsToCellState(fetchedCells)));
      } catch (error) {
        console.error('Error fetching cells:', error);
      }
    };

    fetchCells();
  }, [dispatch]);

  useEffect(() => {
    const fetchDatasets = async () => {
      try {
        const fetchedDatasets = await getDatasets();
        dispatch(setDatasets(fetchedDatasets));
      } catch (error) {
        console.error('Error fetching datasets:', error);
      }
    };

    fetchDatasets();
  }, [dispatch]);

  useEffect(() => {
    const fetchDataframesMetadataList = async () => {
      try {
        const fetchedDataframesMetadataList = await getDataframesMetadata();
        dispatch(setDataframeMetadataList(fetchedDataframesMetadataList));
      } catch (error) {
        console.error('Error fetching dataframes metadata list:', error);
      }
    };

    fetchDataframesMetadataList();
  }, [dispatch]);

  return (
    <div className='flex'>
      <div className='fixed top-0 left-0 w-full z-10 bg-dark border-b border-gray-600'>
        <TopBar isProjectView={true} />
      </div>
      {/* Fixed LeftNavBar */}
      <div
        className={`fixed left-0 top-0 bottom-0 transition-all duration-200 ${
          isCollapsed ? 'w-12' : 'w-60' // Adjust width based on collapse state
        } bg-dark custom-scrollbar`}
      >
        {/* Add a button or icon to toggle collapse */}
        <button
          className='absolute bottom-4 right-0 text-gray-500 p-1 rounded-full text-sm'
          onClick={toggleNavBar}
        >
          {isCollapsed ? '>>' : '<<'}
        </button>
        <LeftNavBar isCollapsed={isCollapsed} projectView={true} />
      </div>

      {/* Vertical Divider with Gray Color */}
      <div
        className={`transition-all duration-200 w-0.5 bg-gray-600`}
        style={{
          height: '100vh', // Ensures the divider spans the full height of the viewport
          marginLeft: isCollapsed ? '3rem' : '15rem', // Adjust based on collapsed state
        }}
      ></div>

      {/* Content Section with margin on the left to avoid overlapping */}
      <div
        className={`flex-1 pt-20 px-4 max-w-full h-full mx-auto bg-dark custom-scrollbar transition-all duration-200 ${
          isCollapsed ? 'pl-16' : 'pl-20' // Adjust left padding based on collapse state
        }`}
      >
        <h1 className='text-4xl font-bold text-gray-400 mb-4'>{title}</h1>
        <div className='flex flex-col items-start'>
          <span className='text-gray-400 text-sm mb-4'>Add Description...</span>
          <label className='text-gray-400 text-xs mb-11'>
            + Add project filter
          </label>
        </div>
        <div className=''>
          {cells.map((cell) => (
            <CellComponent
              key={cell.id}
              datasetsList={datasets}
              cellMetadata={cell}
              notifyDatasetChange={handleDatasetChange}
              dataframeMetadataList={dataframeMetadataList}
            />
          ))}
        </div>
        {/* Add button to create new cell */}
        <button
          className='mt-4 p-2 bg-blue-500 text-white rounded-md'
          onClick={() => dispatch(addCell())}
        >
          Add Cell
        </button>
      </div>
    </div>
  );
};

export default NotebookPage;
