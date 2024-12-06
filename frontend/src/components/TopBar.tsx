import React, { useState } from 'react';

const TopBar: React.FC = () => {
  const [workspace, setWorkspace] = useState('Default Workspace');
  const workspaces = ['Workspace 1', 'Workspace 2', 'Workspace 3'];

  const handleWorkspaceChange = (newWorkspace: string) => {
    setWorkspace(newWorkspace);
  };

  return (
    <div className='h-16 bg-dark flex items-center justify-between px-6 shadow-md'>
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
