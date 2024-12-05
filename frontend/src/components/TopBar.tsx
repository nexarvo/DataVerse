import React, { useState } from 'react';

const TopBar: React.FC = () => {
  const [workspace, setWorkspace] = useState('Default Workspace');
  const workspaces = ['Workspace 1', 'Workspace 2', 'Workspace 3'];

  const handleWorkspaceChange = (newWorkspace: string) => {
    setWorkspace(newWorkspace);
  };

  return (
    <div className='h-16 bg-dark flex items-center justify-between px-6 shadow-md'>
      {/* Left Side: Workspace Dropdown */}
      <div className='relative'>
        <button className='flex items-center font-bold text-gray-100 bg-dark px-4 py-2 rounded-md hover:bg-gray-200'>
          {workspace}
          <svg
            xmlns='http://www.w3.org/2000/svg'
            className='h-5 w-5 ml-2'
            fill='none'
            viewBox='0 0 24 24'
            stroke='currentColor'
          >
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              strokeWidth={2}
              d='M19 9l-7 7-7-7'
            />
          </svg>
        </button>
        <div className='absolute left-0 mt-2 w-48 bg-dark border border-gray-200 rounded-md shadow-lg'>
          {workspaces.map((workspace) => (
            <button
              key={workspace}
              onClick={() => handleWorkspaceChange(workspace)}
              className='w-full text-left px-4 py-2 hover:bg-gray-100'
            >
              {workspace}
            </button>
          ))}
        </div>
      </div>

      {/* Right Side: User Profile (Placeholder) */}
      <div className='flex items-center space-x-4'>
        <img
          src='https://via.placeholder.com/40'
          alt='User Avatar'
          className='h-10 w-10 rounded-full'
        />
        <span className='text-gray-700'>User Name</span>
      </div>
    </div>
  );
};

export default TopBar;
