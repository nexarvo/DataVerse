import axios from 'axios';
import { GetDataframeByIdParam } from '../utils/apiTypes';

const API_URL = import.meta.env.VITE_API_URL;

export const getDataframeById = async (params: GetDataframeByIdParam) => {
  try {
    const response = await axios.get(`${API_URL}/dataframe`, {
      params,
      headers: {
        'Content-Type': 'application/json',
      },
    });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(error.response?.data?.message || 'Error getting dataframe');
  }
};

export const getDataframesMetadata = async () => {
  try {
    const response = await axios.get(`${API_URL}/dataframes-metadata`, {
      headers: {
        'Content-Type': 'application/json',
      },
    });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(
      error.response?.data?.message || 'Error getting dataframes metadata',
    );
  }
};
