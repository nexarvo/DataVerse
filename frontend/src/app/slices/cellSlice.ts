/* eslint-disable @typescript-eslint/no-explicit-any */
import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import Dataset from '../../utils/types';

// Define the types for the state
export interface CellState {
  id: string;
  label: string;
  isEditing: boolean;
  viewDataset: Dataset | null;
  selectedDatasetId: string;
  currentPage: number;
  pageSize: number;
  totalRows: number;
  cell_type: string;
  cell_order: number;
}

// Define the initial state for the slice
const initialState: CellState = {
  id: '',
  label: 'Table 1',
  isEditing: false,
  viewDataset: null,
  selectedDatasetId: '',
  currentPage: 1,
  pageSize: 20,
  totalRows: 0,
  cell_type: 'table', //Default to a table cell
  cell_order: 0,
};

// Create the cellSlice with actions and reducers
const cellSlice = createSlice({
  name: 'cell',
  initialState,
  reducers: {
    setId(state, action: PayloadAction<string>) {
      state.id = action.payload;
    },
    setLabel(state, action: PayloadAction<string>) {
      state.label = action.payload;
    },
    toggleEditing(state) {
      state.isEditing = !state.isEditing;
    },
    setSelectedDatasetId(state, action: PayloadAction<string>) {
      state.selectedDatasetId = action.payload;
    },
    setViewDataset(state, action: PayloadAction<Dataset>) {
      state.viewDataset = action.payload;
    },
    setPage(state, action: PayloadAction<number>) {
      state.currentPage = action.payload;
    },
    setTotalRows(state, action: PayloadAction<number>) {
      state.totalRows = action.payload;
    },
    setCellType(state, action: PayloadAction<string>) {
      state.cell_type = action.payload;
    },
    setCellOrder(state, action: PayloadAction<number>) {
      state.cell_order = action.payload;
    },
  },
});

// Export the actions for use in the component
export const {
  setId,
  setLabel,
  toggleEditing,
  setSelectedDatasetId,
  setViewDataset,
  setPage,
  setTotalRows,
  setCellOrder,
  setCellType,
} = cellSlice.actions;

// Export the reducer to be added to the store
export default cellSlice.reducer;
