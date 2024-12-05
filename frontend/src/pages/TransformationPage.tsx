import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  applyTransformations,
  uploadDataset,
} from '../services/transformations'; // We'll create this service

const TransformationsPage: React.FC = () => {
  const navigate = useNavigate();
  const [dataset, setDataset] = useState<File | null>(null);
  const [transformations, setTransformations] = useState<string[]>([]); // Store selected transformations
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [transformedData, setTransformedData] = useState<any>(null);

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    let file: File | undefined;
    if (e.target.files) {
      file = e.target.files[0];
      setDataset(file);
    }
    if (file) {
      try {
        const formData = new FormData();
        formData.append('file', file);
        await uploadDataset(formData);
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (error: any) {
        setErrorMessage('An error occurred while uploading dataset. ' + error);
      }
    } else {
      setErrorMessage('No file selected.');
    }
  };

  const handleTransformationChange = (
    e: React.ChangeEvent<HTMLSelectElement>,
  ) => {
    const value = e.target.value;
    setTransformations((prev) => [...prev, value]);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!dataset) {
      setErrorMessage('Please upload a dataset.');
      return;
    }

    try {
      const formData = new FormData();
      formData.append('transformations', JSON.stringify(transformations));

      const result = await applyTransformations(formData); // Call to API
      setTransformedData(result); // Assuming the API returns the transformed data
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (error) {
      setErrorMessage('An error occurred while applying transformations.');
    }
  };

  return (
    <div className='p-4'>
      <h1 className='text-2xl font-bold'>Apply Transformations to Dataset</h1>
      {errorMessage && <div className='text-red-500'>{errorMessage}</div>}

      <form onSubmit={handleSubmit} className='space-y-4 mt-6'>
        <div>
          <label
            htmlFor='file'
            className='block text-sm font-medium text-gray-700'
          >
            Upload Dataset
          </label>
          <input
            type='file'
            id='file'
            className='mt-2 w-full p-2 border border-gray-300 rounded-md'
            onChange={handleFileChange}
          />
        </div>

        <div>
          <label
            htmlFor='transformations'
            className='block text-sm font-medium text-gray-700'
          >
            Choose Transformations
          </label>
          <select
            id='transformations'
            className='mt-2 w-full p-2 border border-gray-300 rounded-md'
            onChange={handleTransformationChange}
            multiple
          >
            <option value='transformation1'>Transformation 1</option>
            <option value='transformation2'>Transformation 2</option>
            <option value='transformation3'>Transformation 3</option>
          </select>
        </div>

        <button
          type='submit'
          className='mt-4 w-full py-2 px-4 bg-indigo-600 text-white font-semibold rounded-md hover:bg-indigo-500'
        >
          Apply Transformations
        </button>
      </form>

      {transformedData && (
        <div className='mt-6'>
          <h2 className='text-xl font-semibold'>Transformed Data</h2>
          <pre>{JSON.stringify(transformedData, null, 2)}</pre>
        </div>
      )}
    </div>
  );
};

export default TransformationsPage;
