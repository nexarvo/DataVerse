import { configureStore } from '@reduxjs/toolkit';

import notebookSlice from './slices/notebookSlice';
import cellSlice from './slices/cellSlice';

const store = configureStore({
  reducer: {
    notebook: notebookSlice,
    cell: cellSlice,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
