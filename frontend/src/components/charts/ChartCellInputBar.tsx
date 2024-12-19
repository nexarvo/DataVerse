/* eslint-disable @typescript-eslint/no-explicit-any */
// import React from 'react';
// import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/react';
// import { ChevronDownIcon } from '@heroicons/react/20/solid';
// import { CHART_TYPES } from '../../utils/consts';
// import TabsInPills from '../TabsInOutlineBorder';

// const ChartCellInputBar: React.FC = () => {
//   return (
//     <div className='flex w-2/5'>
//       <div className='flex-col'>
//         <div className='flex justify-between mt-3 mx-2'>
//           <span className='text-gray-300 text-xs mr-4 mt-1'>Type</span>

//           <Menu as='div' className='relative inline-block text-left'>
//             <div>
//               <MenuButton className='inline-flex w-48 justify-start gap-x-1.5 rounded-sm bg-accent px-2 py-1 text-xs text-white ring-1 ring-inset ring-gray-600 hover:bg-third'>
//                 Chart type
//                 <ChevronDownIcon
//                   aria-hidden='true'
//                   className='-mr-1 size-5 text-white'
//                 />
//               </MenuButton>
//             </div>

//             <MenuItems
//               transition
//               className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 transition focus:outline-none data-[closed]:scale-95 data-[closed]:transform data-[closed]:opacity-0 data-[enter]:duration-100 data-[leave]:duration-75 data-[enter]:ease-out data-[leave]:ease-in'
//             >
//               <div className='py-1 h-40 overflow-y-auto custom-scrollbar'>
//                 {CHART_TYPES.map((chartType) => (
//                   <Menu.Item key={chartType.type} as='div'>
//                     <a
//                       href='#'
//                       className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                     >
//                       {chartType.name}
//                     </a>
//                   </Menu.Item>
//                 ))}
//               </div>
//             </MenuItems>
//           </Menu>
//         </div>
//         <TabsInPills />
//       </div>
//       {/* Vertical Line */}
//       <div className='border-l border-gray-600 h-full'></div> {/* Bar Chart */}
//     </div>
//   );
// };

// export default ChartCellInputBar;

import React, { useEffect, useState } from 'react';
import { Menu, MenuButton, MenuItems } from '@headlessui/react';
import { ChevronDownIcon } from '@heroicons/react/20/solid';
import { CHART_INPUTS } from '../../utils/consts';
import TabsInPills from '../TabsInOutlineBorder';

interface ChartCellInputBarProps {
  onSubmit: (data: any) => void;
  cellMetadata: any;
}

const ChartCellInputBar: React.FC<ChartCellInputBarProps> = ({
  onSubmit,
  cellMetadata,
}) => {
  const [selectedChartType, setSelectedChartType] = useState<string>('bar');
  const [xColumn, setXColumn] = useState('');
  const [yColumn, setYColumn] = useState('');

  const handleYColumnChange = (column) => {
    setYColumn(column);
  };

  const handleXColumnChange = (column) => {
    setXColumn(column);
  };

  const handleSubmit = () => {
    onSubmit({
      dataset_id: cellMetadata.selectedDatasetId.replace(
        /^dataframe-|^dataset-/,
        '',
      ),
      is_dataset: cellMetadata.selectedDatasetId.startsWith('dataframe-')
        ? false
        : true,
      chart_type: selectedChartType,
      x_column: xColumn,
      y_column: yColumn || undefined,
    });
  };

  // Effect hook to trigger the API call when both x and y columns are selected
  useEffect(() => {
    if (xColumn && yColumn) {
      handleSubmit();
    }
  }, [xColumn, yColumn]); // Only runs when xColumn or yColumn changes

  const currentChartInputs =
    CHART_INPUTS.find((chart) => chart.type === selectedChartType)?.inputs ||
    [];

  return (
    <div className='flex w-2/5'>
      <div className='flex-col'>
        <div className='flex justify-between mt-3 mx-2'>
          <span className='text-text-secondary text-xs mr-4 mt-1'>Type</span>

          <Menu as='div' className='relative inline-block text-left'>
            <div>
              <MenuButton className='inline-flex w-48 justify-start gap-x-1.5 rounded-sm bg-accent px-2 py-1 text-xs text-text-primary ring-1 ring-inset ring-fourth hover:bg-third'>
                {CHART_INPUTS.find((chart) => chart.type === selectedChartType)
                  ?.name || 'Chart type'}
                <ChevronDownIcon
                  aria-hidden='true'
                  className='-mr-1 size-5 text-text-primary'
                />
              </MenuButton>
            </div>

            <MenuItems className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 focus:outline-none'>
              <div className='py-1 h-40 overflow-y-auto custom-scrollbar'>
                {CHART_INPUTS.map((chart) => (
                  <Menu.Item key={chart.type} as='div'>
                    <a
                      href='#'
                      onClick={() => setSelectedChartType(chart.type)}
                      className='block px-4 py-2 text-sm text-text-primary hover:bg-fourth'
                    >
                      {chart.name}
                    </a>
                  </Menu.Item>
                ))}
              </div>
            </MenuItems>
          </Menu>
        </div>
        <TabsInPills
          chartInputs={currentChartInputs}
          setXColumn={handleXColumnChange}
          setYColumn={handleYColumnChange}
          columns={cellMetadata?.viewDataset?.latest_preview?.headers}
        />
      </div>
    </div>
  );
};

export default ChartCellInputBar;
