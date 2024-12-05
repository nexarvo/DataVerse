// components/HeroSection.tsx
import React, { useEffect, useState } from 'react';
import GridBackground from './GridBackground';
import { Link } from 'react-router-dom';

const HeroSection = () => {
  const [tagline, setTagline] = useState('Teams');
  const [key, setKey] = useState(0); // To trigger re-render for animation

  useEffect(() => {
    const interval = setInterval(() => {
      setTagline((prevTagline) => {
        if (prevTagline === 'Everyone') return 'Teams';
        if (prevTagline === 'Teams') return 'Founders';
        return 'Everyone';
      });
      setKey((prevKey) => prevKey + 1); // Trigger animation on text change
    }, 2000); // Change every 2 seconds

    return () => clearInterval(interval);
  }, []);
  return (
    <div className='relative isolate px-6 pt-14 lg:px-8'>
      <GridBackground />
      <div
        className='absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80'
        aria-hidden='true'
      >
        <div
          className='relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]'
          style={{
            clipPath:
              'polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)',
          }}
        ></div>
      </div>

      <div className='mx-auto max-w-2xl py-32 sm:py-48 lg:py-56'>
        <div className='text-center'>
          {/* Tagline Section */}
          <div className='flex items-center justify-center'>
            <h1 className='text-balance text-5xl font-semibold tracking-tight text-white-900 sm:text-7xl'>
              Data for
            </h1>
            <h1 className='ml-2 text-balance text-5xl font-semibold tracking-tight text-blue-400 sm:text-7xl'>
              {tagline}
            </h1>
          </div>

          {/* Description */}
          <p className='mt-8 text-pretty text-lg font-light text-white-500 sm:text-xl/8'>
            Go end-to-end from quick queries to deep-dive analyses to beautiful
            interactive data apps – all in one collaborative, AI-powered
            workspace.
          </p>

          {/* Action Buttons */}
          <div className='mt-10 flex items-center justify-center gap-x-6'>
            <Link
              to='/signup'
              className='rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600'
            >
              Get started
            </Link>
            <a href='#' className='text-sm/6 font-semibold text-white-900'>
              Learn more <span aria-hidden='true'>&rarr;</span>
            </a>
          </div>
        </div>
      </div>
    </div>
  );
};

export default HeroSection;
