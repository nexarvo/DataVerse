import axios from 'axios';
import { GenerateChartParam } from '../utils/apiTypes';

const API_URL = import.meta.env.VITE_API_URL;

export const generateChart = async (params: GenerateChartParam) => {
  try {
    const response = await axios.post(
      `${API_URL}/charts/generate-chart`,
      params,
      {
        headers: {
          'Content-Type': 'application/json',
        },
      },
    );
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(error.response?.data?.message || 'Error generating chart');
  }
};
