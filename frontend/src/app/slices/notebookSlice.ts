/* eslint-disable @typescript-eslint/no-explicit-any */
import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { v4 as uuidv4 } from 'uuid';
import { CellState } from './cellSlice';

// Define initial state for the notebook

interface NotebookState {
  title: string;
  description: string;
  filters: string[];
  cells: CellState[];
  datasets: any[]; // Update this to match your dataset structure
  dataframeMetadataList: any[];
}

const initialState: NotebookState = {
  title: 'Untitled Project',
  description: '',
  filters: [],
  cells: [
    {
      id: uuidv4().toString(),
      label: 'Untitled Cell',
      isEditing: false,
      viewDataset: null,
      selectedDatasetId: '',
      currentPage: 0,
      pageSize: 20,
      totalRows: 0,
    },
  ], // Initial cell
  datasets: [],
  dataframeMetadataList: [],
};

const notebookSlice = createSlice({
  name: 'notebook',
  initialState,
  reducers: {
    setTitle: (state, action: PayloadAction<string>) => {
      state.title = action.payload;
    },
    setDescription: (state, action: PayloadAction<string>) => {
      state.description = action.payload;
    },
    setFilters: (state, action: PayloadAction<string[]>) => {
      state.filters = action.payload;
    },
    setCells: (state, action: PayloadAction<CellState[]>) => {
      state.cells = action.payload;
    },
    addCell: (state) => {
      state.cells.push({
        id: uuidv4(),
        label: '',
        isEditing: false,
        viewDataset: null,
        selectedDatasetId: '',
        currentPage: 0,
        pageSize: 10,
        totalRows: 0,
      });
    },
    removeCell: (state, action: PayloadAction<string>) => {
      state.cells = state.cells.filter((cell) => cell.id !== action.payload);
    },
    setDatasets: (state, action: PayloadAction<any[]>) => {
      state.datasets = action.payload;
    },
    setDataframeMetadataList(state, action: PayloadAction<any[]>) {
      state.dataframeMetadataList = action.payload;
    },
    updateCellLabel: (
      state,
      action: PayloadAction<{ cellId: string; label: string }>,
    ) => {
      const { cellId, label } = action.payload;
      const cell = state.cells.find((cell) => cell.id === cellId);
      if (cell) {
        cell.label = label;
      }
    },
    toggleCellEditing: (state, action: PayloadAction<{ cellId: string }>) => {
      const { cellId } = action.payload;
      const cell = state.cells.find((cell) => cell.id === cellId);
      if (cell) {
        cell.isEditing = !cell.isEditing;
      }
    },
    setCellSelectedDatasetId: (
      state,
      action: PayloadAction<{ cellId: string; selectedDatasetId: string }>,
    ) => {
      const { cellId, selectedDatasetId } = action.payload;
      const cell = state.cells.find((cell) => cell.id === cellId);
      if (cell) {
        cell.selectedDatasetId = selectedDatasetId;
      }
    },
    setCellViewDataset: (
      state,
      action: PayloadAction<{ cellId: string; viewDataset: any }>,
    ) => {
      const { cellId, viewDataset } = action.payload;
      const cell = state.cells.find((cell) => cell.id === cellId);
      if (cell) {
        cell.viewDataset = viewDataset;
      }
    },
    setCellPage: (
      state,
      action: PayloadAction<{ cellId: string; currentPage: number }>,
    ) => {
      const { cellId, currentPage } = action.payload;
      const cell = state.cells.find((cell) => cell.id === cellId);
      if (cell) {
        cell.currentPage = currentPage;
      }
    },
    setCellTotalRows: (
      state,
      action: PayloadAction<{ cellId: string; totalRows: number }>,
    ) => {
      const { cellId, totalRows } = action.payload;
      const cell = state.cells.find((cell) => cell.id === cellId);
      if (cell) {
        cell.totalRows = totalRows;
      }
    },
  },
});

export const {
  setTitle,
  setDescription,
  setFilters,
  setCells,
  addCell,
  removeCell,
  setDatasets,
  setDataframeMetadataList,
  updateCellLabel,
  toggleCellEditing,
  setCellPage,
  setCellSelectedDatasetId,
  setCellTotalRows,
  setCellViewDataset,
} = notebookSlice.actions;

export default notebookSlice.reducer;
