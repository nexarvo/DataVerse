import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL;

export const uploadDataset = async (formData: FormData) => {
  try {
    const response = await axios.post(`${API_URL}/file/upload`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(
      error.response?.data?.message || 'Error applying transformations',
    );
  }
};

export const getDatasets = async () => {
  try {
    const response = await axios.get(`${API_URL}/datasets`);
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(error.response?.data?.message || 'Error getting datasets');
  }
};

export const getDatasetById = async (id: string) => {
  try {
    const response = await axios.get(`${API_URL}/datasets/${id}`);
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(
      error.response?.data?.message || `Error getting dataset with id: ${id}`,
    );
  }
};
