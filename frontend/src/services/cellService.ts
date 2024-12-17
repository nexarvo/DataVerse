/* eslint-disable @typescript-eslint/no-explicit-any */
import axios from 'axios';
import { CellState } from '../app/slices/cellSlice';

const API_URL = import.meta.env.VITE_API_URL;

export const getCells = async () => {
  try {
    const response = await axios.get(`${API_URL}/cells`, {
      headers: {
        'Content-Type': 'application/json',
      },
    });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(error.response?.data?.message || 'Error getting cell');
  }
};

export const getCellById = async (cellId: string) => {
  try {
    const response = await axios.get(`${API_URL}/cells/${cellId}`, {
      headers: {
        'Content-Type': 'application/json',
      },
    });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(error.response?.data?.message || 'Error getting cell');
  }
};

export function mapFetchedCellsToCellState(fetchedCells: any[]): CellState[] {
  return fetchedCells.map((fetchedCell) => ({
    id: fetchedCell.id,
    label: fetchedCell.name || 'Untitled Cell',
    isEditing: false, // Default value
    viewDataset: fetchedCell.result_dataframe
      ? {
          id: fetchedCell.result_dataframe.id,
          file_name: '',
          file_type: '',
          row_count: 0,
          latest_preview: undefined,
          page: 0,
          page_size: 0,
          total_rows: 0,
        }
      : fetchedCell.input_dataframe
        ? {
            id: fetchedCell.input_dataframe.id,
            file_name: '',
            file_type: '',
            row_count: 0,
            latest_preview: undefined,
            page: 0,
            page_size: 0,
            total_rows: 0,
          }
        : fetchedCell.input_dataset
          ? {
              id: fetchedCell.input_dataset.id,
              file_name: fetchedCell.input_dataset.file_name,
              file_type: fetchedCell.input_dataset.file_type,
              row_count: fetchedCell.input_dataset.row_count || 0,
              latest_preview: undefined,
              page: 0,
              page_size: 0,
              total_rows: fetchedCell.input_dataset.row_count || 0,
            }
          : null, // Fallback to null if none exist
    selectedDatasetId: fetchedCell.input_dataset
      ? `dataset-${fetchedCell.input_dataset?.id}`
      : `dataframe-${fetchedCell.input_dataframe?.id}`,
    currentPage: 1, // Default value
    pageSize: 10, // Default value
    totalRows: fetchedCell.input_dataset?.row_count || 0,
    dataframeMetadataList: [],
  }));
}
