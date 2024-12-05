// src/pages/Dashboard.tsx
import React from 'react';

const Dashboard: React.FC = () => {
  return (
    <div className='flex items-center justify-center min-h-screen bg-gray-100'>
      <h1 className='text-3xl font-bold text-indigo-600'>
        Welcome to Your Dashboard!
      </h1>
      <p className='mt-4 text-lg text-gray-600'>
        You are logged in and ready to explore the app.
      </p>
    </div>
  );
};

export default Dashboard;
