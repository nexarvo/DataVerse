import React from 'react';
import LeftNavBar from '../components/LeftNavBar';
import TopBar from '../components/TopBar';
import { Routes, Route, Outlet } from 'react-router-dom';
import DataPage from './DatasetsPage';

const Dashboard: React.FC = ({ children }) => {
  return (
    <div className='flex h-screen bg-dark'>
      {/* Left Navigation Bar */}
      <LeftNavBar />
      {/* Main Content Area */}
      <div className='flex-1 flex flex-col'>
        {/* Top Bar */}
        <TopBar />
        {/* Dashboard Content */}
        <div className='flex-1 bg-accent'>
          <Outlet />
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
