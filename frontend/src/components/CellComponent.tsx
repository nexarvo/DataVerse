/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useEffect } from 'react';
import { getDatasetById } from '../services/datasets';
import TableComponent from './TableComponent';
import DownRightIcon from '../assets/down-right-icon.svg';
import CommentsIcon from '../assets/comment-icon.svg';
import MenuIcon from '../assets/menu-icon.svg';
import QuickFilter from './QuickFilter';
import { getDataframeById } from '../services/dataframe';
import { useDispatch, useSelector } from 'react-redux';
import {
  updateCellLabel,
  setCellSelectedDatasetId,
  setCellViewDataset,
  toggleCellEditing,
} from '../app/slices/notebookSlice';
import { RootState } from '../app/store';
import CellOptionsComponent from './CellOptionsComponent';
import CellTableComponent from './CellTableComponent';

interface CellComponentProps {
  datasetsList: any[];
  cellMetadata: any;
  notifyDatasetChange: (cellId: string) => void;
  dataframeMetadataList: any[];
}

const CellComponent: React.FC<CellComponentProps> = ({
  datasetsList,
  cellMetadata,
  notifyDatasetChange,
  dataframeMetadataList,
}) => {
  const dispatch = useDispatch();

  // Dynamically fetch the state for this specific cell using cellId
  const cellState = useSelector((state: RootState) =>
    state.notebook.cells.find((cell) => cell.id === cellMetadata.id),
  );

  const {
    label = '',
    isEditing = false,
    viewDataset = null,
    selectedDatasetId = '',
  } = cellState || {}; // Default to an empty object if cellState is undefined

  const handleLabelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(
      updateCellLabel({ cellId: cellMetadata.id, label: e.target.value }),
    );
  };

  const handleBlur = () => {
    dispatch(toggleCellEditing({ cellId: cellMetadata }));
  };

  const handleDatasetChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    dispatch(
      setCellSelectedDatasetId({
        cellId: cellMetadata.id,
        selectedDatasetId: e.target.value,
      }),
    );
  };

  const handleDataChange = (dataset: any) => {
    dispatch(
      setCellViewDataset({ cellId: cellMetadata.id, viewDataset: dataset }),
    );
    notifyDatasetChange(cellMetadata.id);
  };

  const fetchDataset = async (datasetId: string) => {
    try {
      const id = datasetId.replace(/^dataframe-|^dataset-/, '');

      if (datasetId.startsWith('dataframe-')) {
        return await getDataframeById({
          dataframe_id: id,
          page: 1,
          page_size: 20,
        });
      } else {
        return await getDatasetById(id);
      }
    } catch (error) {
      console.error('Error fetching dataset:', error);
      return null;
    }
  };

  const fetchViewData = async () => {
    try {
      let dataset;

      if (selectedDatasetId) {
        // Fetch dataset based on selectedDatasetId
        dataset = await fetchDataset(selectedDatasetId);
      } else if (cellMetadata.result_dataframe) {
        // Fallback: Load using result_dataframe
        dataset = await getDataframeById({
          dataframe_id: cellMetadata.result_dataframe.id,
          page: 1,
          page_size: 20,
        });
      } else if (cellMetadata.input_dataset?.id) {
        // Fallback: Load using input_dataset
        dataset = await getDatasetById(cellMetadata.input_dataset.id);
      } else if (cellMetadata.input_dataframe) {
        // Fallback: Load using input_dataframe
        dataset = await getDataframeById({
          dataframe_id: cellMetadata.input_dataframe.id,
          page: 1,
          page_size: 20,
        });
      }

      dispatch(
        setCellViewDataset({ cellId: cellMetadata.id, viewDataset: dataset }),
      );
    } catch (error) {
      console.error('Error fetching view dataset:', error);
    }
  };

  useEffect(() => {
    fetchViewData();
  }, [
    selectedDatasetId,
    cellMetadata.result_dataframe,
    cellMetadata.input_dataset,
    cellMetadata.input_dataframe,
  ]);

  useEffect(() => {
    if (cellMetadata.input_dataframe) {
      dispatch(
        setCellSelectedDatasetId({
          cellId: cellMetadata.id,
          selectedDatasetId: `dataframe-${cellMetadata.input_dataframe}`,
        }),
      );
    } else if (cellMetadata?.input_dataset?.id) {
      dispatch(
        setCellSelectedDatasetId({
          cellId: cellMetadata.id,
          selectedDatasetId: `dataset-${cellMetadata?.input_dataset?.id}`,
        }),
      );
    }
  }, [cellMetadata]);

  const getDataSetIdToApplyFilter = () => {
    if (cellMetadata.input_dataframe)
      return [cellMetadata.input_dataframe.id, 'dataframe'];
    else if (cellMetadata.result_dataframe)
      return [cellMetadata.result_dataframe.id, 'dataframe'];
    else {
      return [
        selectedDatasetId.replace(/^dataframe-|^dataset-/, ''),
        selectedDatasetId.startsWith('dataframe-') ? 'dataframe' : 'dataset',
      ];
    }
  };

  const getInputDataType = () => {
    if (cellMetadata.input_dataset) return 'dataset';
    else if (cellMetadata.input_dataframe) return 'dataframe';
    else {
      return selectedDatasetId.startsWith('dataframe-')
        ? 'dataframe'
        : 'dataset';
    }
  };

  return (
    <div className='flex flex-col my-4 w-full'>
      <div className='relative pt-2 bg-accent border-[0.5px] border-blue-200 rounded-md mr-7'>
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
              onClick={() =>
                dispatch(toggleCellEditing({ cellId: cellMetadata.id }))
              }
              className='cursor-pointer'
              title='Click to edit'
            >
              {label}
            </span>
          )}
        </div>
        <div className='flex ml-2 mt-1'>
          {/* Dropdown to select dataset */}
          <select
            className='bg-gray-700 text-white rounded-sm mb-4 w-20 h-5 text-green-300 bg-green-500/[.2] mr-4'
            onChange={handleDatasetChange}
            value={selectedDatasetId || ''}
          >
            {/* Group for Dataframe Metadata */}
            {dataframeMetadataList.length > 0 ? (
              <optgroup label='DATAFRAMES'>
                {dataframeMetadataList?.map((metadata: any) => (
                  <option
                    key={`dataframe-${metadata.id}`}
                    value={`dataframe-${metadata.id}`}
                    className='text-blue-200'
                  >
                    {metadata.name}
                  </option>
                ))}
              </optgroup>
            ) : null}

            {/* Group for Datasets */}
            <optgroup label='DATASETS'>
              {datasetsList.map((dataset) => (
                <option
                  key={`dataset-${dataset.id}`}
                  value={`dataset-${dataset.id}`}
                  className='text-green-200'
                >
                  {dataset.file_name}
                </option>
              ))}
            </optgroup>
          </select>
          {/* Quick filter Option */}
          <div className='max-w4xl'>
            <QuickFilter
              cellId={cellMetadata.id}
              dataset_id={getDataSetIdToApplyFilter()[0]}
              colums={viewDataset?.latest_preview?.headers}
              data={viewDataset?.latest_preview?.preview}
              handleDatasetChange={handleDataChange}
              initialFilters={cellMetadata.input_dataframe?.transformations}
              dataType={getDataSetIdToApplyFilter()[1]}
              inputDataType={getInputDataType()}
            />
          </div>
        </div>
        {/* <hr className='border-t border-blue-200 mb-8 w-full' /> */}
        {/* Table */}
        <CellTableComponent
          headers={viewDataset?.latest_preview?.headers}
          data={viewDataset?.latest_preview?.preview}
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
