import React from 'react';
import LeftNavBar from '../components/LeftNavBar';
import TopBar from '../components/TopBar';

const Dashboard: React.FC = ({ children }) => {
  return (
    <div className='flex h-screen'>
      {/* Left Navigation Bar */}
      <LeftNavBar />
      {/* Main Content Area */}
      <div className='flex-1 flex flex-col'>
        {/* Top Bar */}
        <TopBar />
        {/* Dashboard Content */}
        <div className='flex-1 p-6 bg-gray-100'>{children}</div>
      </div>
    </div>
  );
};

export default Dashboard;
