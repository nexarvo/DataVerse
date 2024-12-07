import React, { useState, useRef } from 'react';
import DatasetIcon from '../assets/data-icon-accent.svg';
import RecentIconWhite from '../assets/recent-icon-white.svg';
import RecentIconAccent from '../assets/recent-icon-accent.svg';
import FavoriteIconAccent from '../assets/star-icon-accent.svg';
import FavoriteIconWhite from '../assets/star-icon-white.svg';
import DatabaseIconWhite from '../assets/database-icon-white.svg';
import DatabaseIconAccent from '../assets/database-icon-accent.svg';
import { Tab } from '@headlessui/react';
import DatasetSingleComponent from '../components/DatasetSingleComponent';
import DatasetPreview from '../components/DatasetPreview';

const DataPage: React.FC = () => {
  const [selectedTab, setSelectedTab] = useState('All');
  const [selectedDatasetIndex, setselectedDatasetIndex] = useState(0);
  const datasets = {
    All: [
      {
        title: 'WORKSPACE_FCT',
        description: 'Non-changing facts about workspaces and subscriptions',
        isFavorite: true,
      },
      {
        title: 'WORKSPACE_REV',
        description:
          'Monthly updating slowly-changing-dimension table of revenue for our workspace...',
        isFavorite: false,
      },
      {
        title: 'WORKSPACE_FCT',
        description: 'Non-changing facts about workspaces and subscriptions',
        isFavorite: false,
      },
    ],
    RecentlyUsed: [
      {
        title: 'ORDER_ITEMS',
        description:
          'Stores details of each item within an order, including item and inventory specifics and...',
        isFavorite: true,
      },
      {
        title: 'INVENTORY_ITEMS',
        description:
          'Holds inventory information for products from an example ECommerce dataset, including...',
        isFavorite: false,
      },
    ],
    Favorites: [
      {
        title: 'WORKSPACE_FCT',
        description: 'Non-changing facts about workspaces and subscriptions',
        isFavorite: true,
      },
      {
        title: 'ORDER_ITEMS',
        description:
          'Stores details of each item within an order, including item and inventory specifics and...',
        isFavorite: true,
      },
    ],
  };

  const [leftPaneWidth, setLeftPaneWidth] = useState(300); // initial width of left pane (in pixels)

  const containerRef = useRef<HTMLDivElement>(null);
  const dragRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = (e: React.MouseEvent) => {
    const startX = e.clientX;

    const onMouseMove = (moveEvent: MouseEvent) => {
      const diff = moveEvent.clientX - startX;
      const newWidth = leftPaneWidth + diff;

      // Prevent the left pane from becoming too small or too large
      if (
        newWidth > 150 &&
        newWidth < containerRef.current!.clientWidth - 150
      ) {
        setLeftPaneWidth(newWidth);
      }
    };

    const onMouseUp = () => {
      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
    };

    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', onMouseUp);
  };

  return (
    <div
      className='py-4 border w-full h-full rounded-md border-gray-600'
      ref={containerRef}
    >
      <div className='flex items-center py-4 px-6'>
        <img
          className='rounded-full border border-transparent bg-third p-1 h-8 w-8 mr-2'
          src={DatasetIcon}
          alt='$'
        />
        <p className='text-xl font-bold text-white'>Data browser</p>
      </div>

      {/* Tabs */}
      <Tab.Group
        selectedIndex={
          selectedTab === 'All' ? 0 : selectedTab === 'Recently Used' ? 1 : 2
        }
        onChange={(index) => {
          const tabNames = ['All', 'Recently Used', 'Favorites'];
          setSelectedTab(tabNames[index]);
        }}
      >
        <Tab.List className='flex space-x-4 pl-4 border-b border-gray-600'>
          <Tab
            className={({ selected }) =>
              selected
                ? 'text-sm text-fourth border-b-2 border-fourth'
                : 'text-sm text-white'
            }
            as='button'
            onFocus={(e) => e.target.blur()}
          >
            {({ selected }) => (
              <div className='flex items-center'>
                {selected ? (
                  <img src={DatabaseIconAccent} className='h-3 w-3 mr-2' />
                ) : (
                  <img src={DatabaseIconWhite} className='h-3 w-3 mr-2' />
                )}
                All
              </div>
            )}
          </Tab>
          <Tab
            className={({ selected }) =>
              selected
                ? 'text-sm text-fourth border-b-2 border-fourth'
                : 'text-sm text-white'
            }
            as='button'
            onFocus={(e) => e.target.blur()}
          >
            {({ selected }) => (
              <div className='flex items-center'>
                {selected ? (
                  <img src={RecentIconAccent} className='h-3 w-3 mr-2' />
                ) : (
                  <img src={RecentIconWhite} className='h-3 w-3 mr-2' />
                )}
                Recently used
              </div>
            )}
          </Tab>
          <Tab
            className={({ selected }) =>
              selected
                ? 'text-sm text-fourth border-b-2 border-fourth'
                : 'text-sm text-white'
            }
            as='button'
            onFocus={(e) => e.target.blur()}
          >
            {({ selected }) => (
              <div className='flex items-center'>
                {selected ? (
                  <img src={FavoriteIconAccent} className='h-3 w-3 mr-2' />
                ) : (
                  <img src={FavoriteIconWhite} className='h-3 w-3 mr-2' />
                )}
                Favorites
              </div>
            )}
          </Tab>
        </Tab.List>
      </Tab.Group>

      {/* Main content area */}
      <div className='flex space-x-6'>
        {/* Left pane */}
        <div className='p-4' style={{ width: leftPaneWidth }}>
          <ul className='space-y-2'>
            {datasets[selectedTab]?.map((dataset, index) => (
              <li key={index} className='cursor-pointer'>
                <DatasetSingleComponent dataset={dataset} />
              </li>
            ))}
          </ul>
        </div>

        {/* Divider */}
        <div
          ref={dragRef}
          onMouseDown={handleMouseDown}
          className='cursor-ew-resize bg-gray-600 w-[1px]'
        ></div>

        {/* Preview Pane */}
        <div className='flex-1 p-4'>
          <DatasetPreview
            dataset={datasets[selectedTab][selectedDatasetIndex]}
          />
        </div>
      </div>
    </div>
  );
};

export default DataPage;
