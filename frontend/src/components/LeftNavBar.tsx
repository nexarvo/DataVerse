import { Link } from 'react-router-dom';
import DataIcon from '../assets/data-icon-white.svg';
import HomeIcon from '../assets/home-icon.svg';
import ProjectIcon from '../assets/project-icon.svg';
import ApprovalIcon from '../assets/approved-icon.svg';
import SpreadsheetIcon from '../assets/spreadsheet-icon.svg';
import TrashIcon from '../assets/recycling-icon.svg';
import LogoutIcon from '../assets/logout-icon.svg';
import SettingsIcon from '../assets/settings-icon.svg';
import ArchiveIcon from '../assets/inventory-icon.svg';

import FavoritesLeftNavBar from './favoritesLeftNavBar';

interface LeftNavBarProps {
  isCollapsed: boolean;
  projectView: boolean;
}
const LeftNavBar: React.FC<LeftNavBarProps> = ({
  isCollapsed = false,
  projectView = false,
}) => {
  const navItems = [
    { name: 'Home', path: '/home', imgLink: HomeIcon },
    { name: 'Projects', path: '/projects', imgLink: ProjectIcon },
    { name: 'Data', path: 'data', imgLink: DataIcon },
    { name: 'Approved', path: '/approved', imgLink: ApprovalIcon },
  ];

  const footerNavItems = [
    {
      name: 'Archive',
      path: '/archive',
      imgLink: ArchiveIcon,
    },
    { name: 'Trash', path: '/trash', imgLink: TrashIcon },
    { name: 'Settings', path: '/settings', imgLink: SettingsIcon },
    { name: 'Logout', path: '/logout', imgLink: LogoutIcon },
  ];

  const favoriteProjects = [
    { name: 'Project A', path: '/project-a' },
    { name: 'Project B', path: '/project-b' },
    { name: 'Project C', path: '/project-c' },
  ];

  const favoriteDatasets = [
    { name: 'INVENTORY_ITEMS', path: '/project-a', imgUrl: SpreadsheetIcon },
    { name: 'USERS', path: '/project-b', imgUrl: SpreadsheetIcon },
    { name: 'ORDER_ITEMS', path: '/project-c', imgUrl: SpreadsheetIcon },
  ];

  return (
    <div
      className={`${projectView ? 'pt-10' : null} h-screen bg-accent text-text-primary flex flex-col`}
    >
      {!projectView && <div className='p-4 text-md font-bold'>DataVerse</div>}
      <nav className='flex-1 p-2'>
        {navItems.map((item) => (
          <Link
            key={item.name}
            to={item.path}
            className='block px-2 py-2 text-sm rounded-sm hover:bg-slate-800 flex items-center space-x-2'
          >
            <img src={item.imgLink} alt={item.name} className='w-4 h-4' />
            {!isCollapsed && <span className='text-xs'>{item.name}</span>}
          </Link>
        ))}

        {!projectView && (
          <FavoritesLeftNavBar
            favoritesTitle='FAVORITE PROJECTS'
            favoriteItems={favoriteProjects}
          />
        )}

        {!projectView && (
          <FavoritesLeftNavBar
            favoritesTitle='FAVORITE DATA'
            favoriteItems={favoriteDatasets}
          />
        )}
      </nav>
      <div className='p-2 border-t border-fourth'>
        {footerNavItems.map((item) => (
          <Link
            key={item.name}
            to={item.path}
            className='block px-2 py-2 text-sm rounded-sm hover:bg-slate-800 flex items-center space-x-2'
          >
            <img src={item.imgLink} alt={item.name} className='w-4 h-4' />
            {!isCollapsed && <span className='text-xs'>{item.name}</span>}
          </Link>
        ))}
      </div>
    </div>
  );
};

export default LeftNavBar;
