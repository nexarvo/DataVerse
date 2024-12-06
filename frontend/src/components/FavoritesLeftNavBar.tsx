/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';
import { Link } from 'react-router-dom';

interface FavoritesLeftNavBarProps {
  favoriteItems: any[];
  favoritesTitle: string;
}

const FavoritesLeftNavBar: React.FC<FavoritesLeftNavBarProps> = ({
  favoriteItems,
  favoritesTitle,
}) => {
  const [isDropdownOpen, setDropdownOpen] = useState(false);
  const toggleDropdown = () => setDropdownOpen(!isDropdownOpen);

  return (
    <div>
      <button
        className='flex items-center w-64 px-2 py-auto mt-4 text-sm rounded-sm'
        onClick={toggleDropdown}
      >
        <svg
          xmlns='http://www.w3.org/2000/svg'
          className={`h-3 w-3 mr-2 mb-2 transform transition-transform duration-300 ease-in-out text-gray-400 font-bold ${
            isDropdownOpen ? 'rotate-90' : 'rotate-0'
          }`}
          style={{ transformOrigin: 'center' }}
          fill='none'
          viewBox='0 0 24 24'
          stroke='currentColor'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            strokeWidth={2}
            d='M8 4l8 8-8 8'
          />
        </svg>
        <span className='text-[10px] font-semibold text-gray-400 pb-2'>
          {favoritesTitle}
        </span>
      </button>
      {isDropdownOpen && (
        <div className='space-y-1'>
          {favoriteItems.map((item: any) => (
            <Link
              key={item.name}
              to={item.path}
              className='block px-2 py-2 text-sm rounded-sm hover:bg-slate-800 flex items-center'
            >
              {item.imgUrl ? (
                <img src={item.imgUrl} alt='none' className='w-4 h-4 mr-2' />
              ) : null}

              <span className='text-[12px]'>{item.name}</span>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
};

export default FavoritesLeftNavBar;
