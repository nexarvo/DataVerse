// import { Menu, MenuButton, MenuItems } from '@headlessui/react';
// import { ChevronDownIcon } from '@heroicons/react/20/solid';
// import { useState } from 'react';
// import { CHART_TYPES } from '../utils/consts';

import { Menu, MenuButton, MenuItems } from '@headlessui/react';
import { ChevronDownIcon } from '@heroicons/react/20/solid';
import { useState } from 'react';

// const TabsInOutlineBorder = () => {
//   // State to track the active tab
//   const [activeTab, setActiveTab] = useState(1);

//   return (
//     <div className='max-w-3xl mx-auto mt-4'>
//       {/* Tab navigation */}
//       <div className='flex justify-center border-gray-200 mb-2'>
//         <button
//           onClick={() => setActiveTab(1)}
//           className={`${
//             activeTab === 1
//               ? 'border border-gray-600 text-white'
//               : 'bg-dark text-gray-300'
//           } px-10 py-1 inline-block text-xs rounded-sm hover:text-white transition-all`}
//         >
//           Data
//         </button>
//         <button
//           onClick={() => setActiveTab(2)}
//           className={`${
//             activeTab === 2 ? 'border border-gray-600' : 'bg-dark text-gray-300'
//           } px-10 py-1 inline-block text-xs rounded-sm hover:text-white transition-all`}
//         >
//           Style
//         </button>
//       </div>
//       <hr className='border-t border-gray-600 w-full mb-4' />

//       {/* Tab content */}
//       <div className='px-2'>
//         {activeTab === 1 && (
//           <div className='flex flex-col'>
//             <span className='text-md font-semibold text-white'>X-Axis</span>
//             <Menu as='div' className='relative inline-block text-left mt-2'>
//               <div>
//                 <MenuButton className='inline-flex w-full justify-start gap-x-1.5 rounded-sm bg-accent px-2 py-1 text-xs text-white ring-1 ring-inset ring-gray-600 hover:bg-third'>
//                   Select column...
//                   <ChevronDownIcon
//                     aria-hidden='true'
//                     className='-mr-1 size-5 text-white'
//                   />
//                 </MenuButton>
//               </div>

//               <MenuItems
//                 transition
//                 className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 transition focus:outline-none data-[closed]:scale-95 data-[closed]:transform data-[closed]:opacity-0 data-[enter]:duration-100 data-[leave]:duration-75 data-[enter]:ease-out data-[leave]:ease-in'
//               >
//                 <div className='py-1 h-40 overflow-y-auto custom-scrollbar'>
//                   {CHART_TYPES.map((chartType) => (
//                     <Menu.Item key={chartType.type} as='div'>
//                       <a
//                         href='#'
//                         className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                       >
//                         {chartType.name}
//                       </a>
//                     </Menu.Item>
//                   ))}
//                 </div>
//               </MenuItems>
//             </Menu>
//             <div className='flex justify-between items-center'>
//               <span className='text-gray-400 text-xs'>Sort</span>
//               <Menu as='div' className='relative inline-block text-left mt-2'>
//                 <div>
//                   <MenuButton className='inline-flex w-full justify-start gap-x-1.5 bg-accent px-2 py-1 text-xs text-gray-400'>
//                     Sort order...
//                     <ChevronDownIcon
//                       aria-hidden='true'
//                       className='-mr-1 size-5 text-gray-400'
//                     />
//                   </MenuButton>
//                 </div>

//                 <MenuItems
//                   transition
//                   className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 transition focus:outline-none data-[closed]:scale-95 data-[closed]:transform data-[closed]:opacity-0 data-[enter]:duration-100 data-[leave]:duration-75 data-[enter]:ease-out data-[leave]:ease-in'
//                 >
//                   <div className='py-1 h-20 overflow-y-auto custom-scrollbar'>
//                     <Menu.Item key='ASC' as='div'>
//                       <a
//                         href='#'
//                         className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                       >
//                         Ascending
//                       </a>
//                     </Menu.Item>
//                     <Menu.Item key='DESC' as='div'>
//                       <a
//                         href='#'
//                         className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                       >
//                         Descending
//                       </a>
//                     </Menu.Item>
//                   </div>
//                 </MenuItems>
//               </Menu>
//             </div>
//             <span className='text-md font-semibold text-white mt-4'>
//               Y-Axis
//             </span>
//             <Menu as='div' className='relative inline-block text-left mt-2'>
//               <div>
//                 <MenuButton className='inline-flex w-full justify-start gap-x-1.5 rounded-sm bg-accent px-2 py-1 text-xs text-white ring-1 ring-inset ring-gray-600 hover:bg-third'>
//                   Select column...
//                   <ChevronDownIcon
//                     aria-hidden='true'
//                     className='-mr-1 size-5 text-white'
//                   />
//                 </MenuButton>
//               </div>

//               <MenuItems
//                 transition
//                 className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 transition focus:outline-none data-[closed]:scale-95 data-[closed]:transform data-[closed]:opacity-0 data-[enter]:duration-100 data-[leave]:duration-75 data-[enter]:ease-out data-[leave]:ease-in'
//               >
//                 <div className='py-1 h-40 overflow-y-auto custom-scrollbar'>
//                   {CHART_TYPES.map((chartType) => (
//                     <Menu.Item key={chartType.type} as='div'>
//                       <a
//                         href='#'
//                         className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                       >
//                         {chartType.name}
//                       </a>
//                     </Menu.Item>
//                   ))}
//                 </div>
//               </MenuItems>
//             </Menu>
//             <div className='flex justify-between items-center'>
//               <span className='text-gray-400 text-xs'>Aggregate</span>
//               <Menu as='div' className='relative inline-block text-left mt-2'>
//                 <div>
//                   <MenuButton className='inline-flex w-full justify-start gap-x-1.5 bg-accent px-2 py-1 text-xs text-gray-400'>
//                     Select agg...
//                     <ChevronDownIcon
//                       aria-hidden='true'
//                       className='-mr-1 size-5 text-gray-400'
//                     />
//                   </MenuButton>
//                 </div>

//                 <MenuItems
//                   transition
//                   className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 transition focus:outline-none data-[closed]:scale-95 data-[closed]:transform data-[closed]:opacity-0 data-[enter]:duration-100 data-[leave]:duration-75 data-[enter]:ease-out data-[leave]:ease-in'
//                 >
//                   <div className='py-1 h-20 overflow-y-auto custom-scrollbar'>
//                     <Menu.Item key='ASC' as='div'>
//                       <a
//                         href='#'
//                         className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                       >
//                         Sum
//                       </a>
//                     </Menu.Item>
//                     <Menu.Item key='DESC' as='div'>
//                       <a
//                         href='#'
//                         className='block px-4 py-2 text-sm text-white hover:bg-fourth data-[focus]:bg-gray-100 data-[focus]:text-gray-900 data-[focus]:outline-none'
//                       >
//                         Mean
//                       </a>
//                     </Menu.Item>
//                   </div>
//                 </MenuItems>
//               </Menu>
//             </div>
//           </div>
//         )}
//         {activeTab === 2 && (
//           <div>
//             <h2 className='text-2xl font-bold'>Tab 2 Content</h2>
//             <p>This is the content for Tab 2.</p>
//           </div>
//         )}
//       </div>
//     </div>
//   );
// };

// export default TabsInOutlineBorder;

const TabsInOutlineBorder: React.FC<{
  chartInputs: any[];
  setXColumn: React.Dispatch<React.SetStateAction<string>>;
  setYColumn: React.Dispatch<React.SetStateAction<string>>;
  columns: string[];
}> = ({ chartInputs, setXColumn, setYColumn, columns }) => {
  const [activeTab, setActiveTab] = useState(1);
  const [selectedXColumn, setSelectedXColumn] = useState<string | null>(null);
  const [selectedYColumn, setSelectedYColumn] = useState<string | null>(null);

  const handleColumnSelect = (column: string, type: string) => {
    if (type === 'x') {
      console.log('Selected X column:', column);
      // Only set the X column when selected
      setSelectedXColumn(column);
      setXColumn(column); // Update the X-axis column state
    } else if (type === 'y') {
      console.log('Selected Y column:', column);
      // Only set the Y column when selected
      setSelectedYColumn(column);
      setYColumn(column); // Update the Y-axis column state
    }
  };

  return (
    <div className='max-w-3xl mx-auto mt-4'>
      {/* Tab navigation */}
      <div className='flex justify-center border-gray-200 mb-2'>
        <button
          onClick={() => setActiveTab(1)}
          className={`${
            activeTab === 1
              ? 'border border-fourth text-text-primary'
              : 'bg-dark text-text-primary'
          } px-10 py-1 inline-block text-xs rounded-sm hover:text-text-primary transition-all`}
        >
          Data
        </button>
        <button
          onClick={() => setActiveTab(2)}
          className={`${
            activeTab === 2
              ? 'border border-fourth text-text-primary'
              : 'bg-dark text-text-primary'
          } px-10 py-1 inline-block text-xs rounded-sm hover:text-white transition-all`}
        >
          Style
        </button>
      </div>
      <hr className='border-t border-fourth w-full mb-4' />

      {/* Tab content */}
      <div className='px-2'>
        {activeTab === 1 && (
          <div className='flex flex-col space-y-4'>
            {chartInputs?.map((input) => (
              <div key={input.id}>
                <label className='text-text-secondary text-sm'>
                  {input.label}
                </label>
                {/* Render input based on type */}
                {input.type === 'column' && (
                  <div>
                    {input.id === 'xAxis' ? (
                      <Menu
                        as='div'
                        className='relative inline-block text-left mt-2'
                      >
                        <MenuButton className='inline-flex w-full justify-start gap-x-1.5 bg-accent px-2 py-1 text-xs text-text-primary ring-1 ring-inset ring-gray-600 hover:bg-third'>
                          {selectedXColumn || 'Select X Column...'}
                          <ChevronDownIcon
                            aria-hidden='true'
                            className='-mr-1 size-5 text-text-primary'
                          />
                        </MenuButton>
                        <MenuItems className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 focus:outline-none'>
                          {columns?.map((column) => (
                            <Menu.Item key={column}>
                              <button
                                onClick={() => handleColumnSelect(column, 'x')}
                                className='block w-full px-4 py-2 text-sm text-text-primary hover:bg-fourth text-left'
                              >
                                {column}
                              </button>
                            </Menu.Item>
                          ))}
                        </MenuItems>
                      </Menu>
                    ) : (
                      <Menu
                        as='div'
                        className='relative inline-block text-left mt-2'
                      >
                        <MenuButton className='inline-flex w-full justify-start gap-x-1.5 bg-accent px-2 py-1 text-xs text-text-primary ring-1 ring-inset ring-gray-600 hover:bg-third'>
                          {selectedYColumn || 'Select Y Column...'}
                          <ChevronDownIcon
                            aria-hidden='true'
                            className='-mr-1 size-5 text-text-primary'
                          />
                        </MenuButton>
                        <MenuItems className='absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-third shadow-lg ring-1 ring-black/5 focus:outline-none'>
                          {columns?.map((column) => (
                            <Menu.Item key={column}>
                              <button
                                onClick={() => handleColumnSelect(column, 'y')}
                                className='block w-full px-4 py-2 text-sm text-text-primary hover:bg-fourth text-left'
                              >
                                {column}
                              </button>
                            </Menu.Item>
                          ))}
                        </MenuItems>
                      </Menu>
                    )}
                  </div>
                )}
                {input.type === 'dropdown' && (
                  <select className='w-full bg-accent text-text-primary p-2 rounded-sm'>
                    {input.options?.map((option) => (
                      <option key={option} value={option}>
                        {option}
                      </option>
                    ))}
                  </select>
                )}
                {input.type === 'number' && (
                  <input
                    type='number'
                    className='w-full bg-accent text-text-primary p-2 rounded-sm'
                  />
                )}
              </div>
            ))}
          </div>
        )}
        {activeTab === 2 && <div>Style settings go here.</div>}
      </div>
    </div>
  );
};

export default TabsInOutlineBorder;
