import axios from 'axios';
import ApplyTransformationApiType from '../utils/apiTypes';

const API_URL = import.meta.env.VITE_API_URL;

export const applyTransformations = async (
  dataset_id: string,
  transformation: ApplyTransformationApiType,
) => {
  try {
    const response = await axios.post(
      `${API_URL}/datasets/${dataset_id}/apply-transformation`,
      transformation,
      {
        headers: {
          'Content-Type': 'application/json',
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
