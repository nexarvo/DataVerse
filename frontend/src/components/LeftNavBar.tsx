import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import DataIcon from '../assets/data-icon.svg';
import HomeIcon from '../assets/home-icon.svg';
import ProjectIcon from '../assets/project-icon.svg';
import ApprovalIcon from '../assets/approved-icon.svg';

const LeftNavBar: React.FC = () => {
  const navItems = [
    { name: 'Home', path: '/home', imgLink: HomeIcon },
    { name: 'Projects', path: '/projects', imgLink: ProjectIcon },
    { name: 'Data', path: '/data', imgLink: DataIcon },
    { name: 'Approved', path: '/approved', imgLink: ApprovalIcon },
  ];
  const [isDropdownOpen, setDropdownOpen] = useState(false);

  const favoriteProjects = [
    { name: 'Project A', path: '/project-a' },
    { name: 'Project B', path: '/project-b' },
    { name: 'Project C', path: '/project-c' },
  ];

  const toggleDropdown = () => setDropdownOpen(!isDropdownOpen);

  return (
    <div className='h-screen w-64 bg-dark text-white flex flex-col'>
      <div className='p-4 text-2xl font-bold'>MyApp</div>
      <nav className='flex-1 p-2'>
        {navItems.map((item) => (
          <Link
            key={item.name}
            to={item.path}
            className='block px-2 py-2 text-sm rounded-sm hover:bg-slate-800 flex items-center space-x-2'
          >
            <img src={item.imgLink} alt={item.name} className='w-4 h-4' />
            <span className='text-sm'>{item.name}</span>
          </Link>
        ))}
        <button
          className='flex justify-start w-64 px-2 py-2 text-sm rounded-sm hover:bg-slate-800'
          onClick={toggleDropdown}
        >
          <svg
            xmlns='http://www.w3.org/2000/svg'
            className={`h-5 w-5 pr-2 transform transition-transform ${
              isDropdownOpen ? 'rotate-180' : 'rotate-90'
            }`}
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
          <span className='text-xs/[17px]'>FAVORITE PROJECTS</span>
        </button>
        {isDropdownOpen && (
          <div className='mt-2 space-y-1'>
            {favoriteProjects.map((project) => (
              <Link
                key={project.name}
                to={project.path}
                className='block px-4 py-2 text-sm rounded-sm hover:bg-slate-800 flex items-center'
              >
                <span>{project.name}</span>
              </Link>
            ))}
          </div>
        )}
      </nav>
      <div className='p-4 border-t border-gray-700'>
        <button className='w-full px-4 py-2 bg-red-600 rounded-md hover:bg-red-500'>
          Log Out
        </button>
      </div>
    </div>
  );
};

export default LeftNavBar;
