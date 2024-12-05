// components/Navbar.tsx
import React, { useState } from 'react';
import BackgroundShapes from './BackgroundShapes';

const Navbar = () => {
  const [open, setOpen] = useState(false);

  return (
    <header className='absolute inset-x-0 top-0 z-50'>
      <nav
        className='flex items-center justify-between p-6 lg:px-8'
        aria-label='Global'
      >
        <div className='flex lg:flex-1'>
          <a href='#' className='-m-1.5 p-1.5'>
            <span className='sr-only'>Your Company</span>
            <img
              className='h-8 w-auto'
              src='https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=600'
              alt=''
            />
          </a>
        </div>
        <div className='flex lg:hidden'>
          <button
            type='button'
            className='-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-white-700'
            onClick={() => setOpen(!open)}
          >
            <span className='sr-only'>Open main menu</span>
            <svg
              className='size-6'
              fill='none'
              viewBox='0 0 24 24'
              strokeWidth='1.5'
              stroke='currentColor'
              aria-hidden='true'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                d='M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5'
              />
            </svg>
          </button>
        </div>
        <div className='hidden lg:flex lg:gap-x-12'>
          <a href='#' className='text-sm/6 font-semibold text-white-900'>
            Features
          </a>
          <a href='#' className='text-sm/6 font-semibold text-white-900'>
            Mission
          </a>
          <a href='#' className='text-sm/6 font-semibold text-white-900'>
            GitHub
          </a>
        </div>
        <div className='hidden lg:flex lg:flex-1 lg:justify-end'>
          <a href='#' className='text-sm/6 font-semibold text-white-900'>
            Log in <span aria-hidden='true'>&rarr;</span>
          </a>
        </div>
      </nav>

      {/* Mobile Menu */}
      {open && (
        <div className='lg:hidden' role='dialog' aria-modal='true'>
          <div className='fixed inset-0 z-50 bg-retro px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10'>
            <BackgroundShapes />
            <div className='flex items-center justify-between'>
              <a href='#' className='-m-1.5 p-1.5'>
                <span className='sr-only'>Your Company</span>
                <img
                  className='h-8 w-auto'
                  src='https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=600'
                  alt=''
                />
              </a>
              <button
                type='button'
                className='-m-2.5 rounded-md p-2.5 text-white-700'
                onClick={() => setOpen(false)}
              >
                <span className='sr-only'>Close menu</span>
                <svg
                  className='size-6'
                  fill='none'
                  viewBox='0 0 24 24'
                  strokeWidth='1.5'
                  stroke='currentColor'
                  aria-hidden='true'
                >
                  <path
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    d='M6 18 18 6M6 6l12 12'
                  />
                </svg>
              </button>
            </div>
            <div className='mt-6 flow-root'>
              <div className='-my-6 divide-y divide-gray-500/10'>
                <div className='space-y-2 py-6'>
                  <a
                    href='#'
                    className='-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-white-900 hover:bg-gray-50'
                  >
                    Features
                  </a>
                  <a
                    href='#'
                    className='-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-white-900 hover:bg-gray-50'
                  >
                    Mission
                  </a>
                  <a
                    href='#'
                    className='-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-white-900 hover:bg-gray-50'
                  >
                    GitHub
                  </a>
                </div>
                <div className='py-6'>
                  <a
                    href='#'
                    className='-mx-3 block rounded-lg px-3 py-2.5 text-base/7 font-semibold text-white-900 hover:bg-gray-50'
                  >
                    Log in
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </header>
  );
};

export default Navbar;
