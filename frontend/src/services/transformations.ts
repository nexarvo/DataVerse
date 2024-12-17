import axios from 'axios';
import ApplyTransformationApiType from '../utils/apiTypes';

const API_URL = import.meta.env.VITE_API_URL;

export const applyTransformations = async (
  cell_id: string,
  datasetId: string,
  input_data_type: string,
  dataType: string,
  transformation: ApplyTransformationApiType,
) => {
  try {
    const response = await axios.post(
      `${API_URL}/cells/${cell_id}/apply-transformation`,
      {
        dataset_id: datasetId,
        input_data_type: input_data_type,
        data_type: dataType,
        transformation,
      },
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
