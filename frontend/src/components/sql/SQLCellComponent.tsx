/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useEffect, useState } from 'react';
import { getDatasetById } from '../../services/datasets';
import TableComponent from '../TableComponent';
import DownRightIcon from '../assets/down-right-icon.svg';
import CommentsIcon from '../assets/comment-icon.svg';
import MenuIcon from '../assets/menu-icon.svg';
import QuickFilter from '../QuickFilter';
import { getDataframeById } from '../../services/dataframe';
import { useDispatch, useSelector } from 'react-redux';
import {
  updateCellLabel,
  setCellSelectedDatasetId,
  setCellViewDataset,
  toggleCellEditing,
} from '../../app/slices/notebookSlice';
import { RootState } from '../../app/store';
import CellOptionsComponent from '../CellOptionsComponent';
import CellTableComponent from '../CellTableComponent';
import EChartComponent from './EChartComponent';
import ChartCellInputBar from './ChartCellInputBar';
import { GenerateChartParam, QuerySQLParam } from '../../utils/apiTypes';
import { generateChart } from '../../services/chartService';
import SQLEditor from './SQLEditor';
import { Button } from '@headlessui/react';
import { querySQL } from '../../services/cellService';

interface SQLCellComponentProps {
  datasetsList: any[];
  cellMetadata: any;
  notifyDatasetChange: (cellId: string) => void;
  dataframeMetadataList: any[];
}

const SQLCellComponent: React.FC<SQLCellComponentProps> = ({
  datasetsList,
  cellMetadata,
  notifyDatasetChange,
  dataframeMetadataList,
}) => {
  const dispatch = useDispatch();

  const [sqlQuery, setSqlQuery] = useState<QuerySQLParam>();

  let [chartData, setChartData] = useState<any>(null);
  let [selectedChartInput, setSelectedChartInput] =
    useState<GenerateChartParam>();

  const handleGenerateChartData = async (data: GenerateChartParam) => {
    try {
      const generatedChart = await generateChart(data);
      setChartData(generatedChart);
      setSelectedChartInput(data);
    } catch (error) {
      console.error('Error generating chart:', error);
    }
  };

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

  const handleRunQuery = async () => {
    try {
      if (sqlQuery) {
        sqlQuery.cell_id = cellMetadata.id;
        const dataframeId = await querySQL(sqlQuery);
        // For a given cell we will create dataframe only once so for further queries
        // selectedDatasetId will remain same so we need to manually call fetch
        if (selectedDatasetId) {
          fetchViewData();
        } else {
          dispatch(
            setCellSelectedDatasetId({
              cellId: cellMetadata.id,
              selectedDatasetId: `dataframe-${dataframeId}`,
            }),
          );
        }
      } else {
        console.error('SQL query is undefined');
      }
    } catch (error) {
      console.error('Error running SQL query:', error);
    }
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
    <div>
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
      <div className='flex justify-end items-center mr-2'>
        <span className='text-xs mr-2 text-text-secondary font-light'>
          2.5s
        </span>
        <button
          className='px-2 py-0.5 bg-green-glass border border-green-800 text-green-300 shadow-md backdrop-blur-md rounded-sm hover:bg-green-hover transition'
          onClick={handleRunQuery}
        >
          <div className='flex items-center'>
            <span className='text-xs'>Run</span>
          </div>
        </button>
      </div>
      <div className='flex'>
        <SQLEditor
          setQuery={setSqlQuery}
          datasetsList={datasetsList}
          dataframeMetadataList={dataframeMetadataList}
        />
      </div>
      {/* Table */}
      <CellTableComponent
        headers={viewDataset?.latest_preview?.headers}
        data={viewDataset?.latest_preview?.preview}
      />
    </div>
  );
};

export default SQLCellComponent;
