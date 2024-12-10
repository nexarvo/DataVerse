/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState, useRef, useEffect } from 'react';
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
import { getDatasets } from '../services/datasets';

const DataPage: React.FC = () => {
  const [selectedTab, setSelectedTab] = useState('All');
  const [selectedDatasetIndex, setselectedDatasetIndex] = useState(0);

  const [newDatasets, setDatasets] = useState<any>({
    All: [],
    RecentlyUsed: [],
    Favorites: [],
  });

  useEffect(() => {
    const fetchDatasets = async () => {
      try {
        const fetchedDatasets = await getDatasets();
        setDatasets({
          All: fetchedDatasets,
          RecentlyUsed: fetchedDatasets,
          Favorites: fetchedDatasets,
        });
      } catch (error) {
        console.error('Error fetching datasets:', error);
      }
    };

    fetchDatasets();
  }, []);

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
          const tabNames = ['All', 'Recently Used', 'Favorites', 'Samples'];
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
                Samples
              </div>
            )}
          </Tab>
        </Tab.List>
      </Tab.Group>

      {/* Main content area */}
      <div className='parent h-[72vh] flex space-x-6'>
        {/* Left pane */}
        <div
          className='child p-4 max-h-full overflow-y-auto custom-scrollbar'
          style={{ width: leftPaneWidth }}
        >
          <ul className='space-y-2'>
            {newDatasets[selectedTab]?.map((dataset: any, index: number) => (
              <li
                key={index}
                className={`cursor-pointer ${selectedDatasetIndex === index ? 'bg-third' : ''}`}
                onClick={() => setselectedDatasetIndex(index)}
              >
                <DatasetSingleComponent dataset={dataset} />
              </li>
            ))}
          </ul>
        </div>

        {/* Divider */}
        <div
          ref={dragRef}
          onMouseDown={handleMouseDown}
          className='cursor-ew-resize bg-gray-600'
          style={{ width: '0px', margin: '2px', padding: '1px' }}
        ></div>

        {/* Preview Pane */}
        <div className='flex-1 p-4'>
          <DatasetPreview
            dataset={newDatasets[selectedTab][selectedDatasetIndex]}
          />
        </div>
      </div>
    </div>
  );
};

export default DataPage;
