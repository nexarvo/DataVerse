import React from 'react';
import NotificationIcon from '../assets/notification-icon.svg';

const TopBar: React.FC = () => {
  return (
    <div className='h-16 bg-dark flex items-center justify-between px-6 shadow-md'>
      {/* Search Bar */}
      <div className='relative w-1/3 ml-72'>
        <input
          type='text'
          className='w-full bg-third text-white placeholder-gray-400 text-sm rounded-sm py-1 pl-10 pr-8 focus:outline-none'
          placeholder='Search you workspace...'
        />
        <svg
          xmlns='http://www.w3.org/2000/svg'
          fill='none'
          viewBox='0 0 24 24'
          strokeWidth='1.5'
          stroke='currentColor'
          className='absolute left-3 top-1.5 h-5 w-5 text-gray-400'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            d='M15.75 15.75L19.5 19.5m-4.5-3.75a6 6 0 100-12 6 6 0 000 12z'
          />
        </svg>
      </div>

      {/* Right Section */}
      <div className='flex items-center space-x-4'>
        <button className='text-gray-400 text-sm hover:bg-transparent'>
          <img src={NotificationIcon} alt='icon' className='h-4 w-4' />
        </button>
        <button className='px-2 py-1 bg-green-glass border border-green-800 text-green-300 shadow-md backdrop-blur-md rounded-sm hover:bg-green-hover transition'>
          <div className='flex items-center'>
            <svg
              width='20'
              height='20'
              viewBox='0 0 24 24'
              fill='none'
              xmlns='http://www.w3.org/2000/svg'
            >
              <path
                d='M12 5V19'
                stroke='rgb(134, 239, 172)'
                stroke-width='2'
                stroke-linecap='round'
                stroke-linejoin='round'
              />
              <path
                d='M5 12H19'
                stroke='rgb(134, 239, 172)'
                stroke-width='2'
                stroke-linecap='round'
                stroke-linejoin='round'
              />
            </svg>
            <span className='text-sm ml-1'>New</span>

            {/* Partition */}
            <div className='mx-2 border-l border-green-300 h-4'></div>

            {/* Down Arrow */}
            <svg
              width='20'
              height='20'
              viewBox='0 0 24 24'
              fill='none'
              xmlns='http://www.w3.org/2000/svg'
            >
              <path
                d='M12 15L8 11H16L12 15Z'
                stroke='rgb(134, 239, 172)'
                stroke-width='2'
                stroke-linecap='round'
                stroke-linejoin='round'
              />
            </svg>
          </div>
        </button>
      </div>
    </div>
  );
};

export default TopBar;
