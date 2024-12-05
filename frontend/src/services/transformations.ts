import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL;

export const applyTransformations = async (formData: FormData) => {
  try {
    const dataset_id = '';
    const response = await axios.post(
      `${API_URL}/${dataset_id}/apply-transformation`,
      formData,
      {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      },
    );
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(
      error.response?.data?.message || 'Error applying transformations',
    );
  }
};

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
