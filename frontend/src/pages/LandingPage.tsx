import React, { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import Navbar from '../components/Navbar';
import Footer from '../components/Footer';
import HeroSection from '../components/HeroSection';
import BackgroundShapes from '../components/BackgroundShapes';

const LandingPage: React.FC = () => {
  return (
    <div className='bg-retro text-light font-sans'>
      <Navbar />
      <HeroSection />
      <BackgroundShapes />

      {/* Features Section */}
      <section className='py-20 bg-light text-dark'>
        <div className='max-w-6xl mx-auto px-8'>
          <h2 className='text-4xl font-bold text-center mb-12'>
            Why Choose DataVerse?
          </h2>
          <div className='grid grid-cols-1 md:grid-cols-3 gap-8'>
            <div className='p-8 bg-dark text-light rounded shadow'>
              <h3 className='text-2xl font-bold mb-4'>
                Real-Time Collaboration
              </h3>
              <p>
                Work with your team to transform and analyze data seamlessly.
              </p>
            </div>
            <div className='p-8 bg-dark text-light rounded shadow'>
              <h3 className='text-2xl font-bold mb-4'>AI-Powered Insights</h3>
              <p>
                Leverage AI to uncover insights and accelerate decision-making.
              </p>
            </div>
            <div className='p-8 bg-dark text-light rounded shadow'>
              <h3 className='text-2xl font-bold mb-4'>Scalable Solutions</h3>
              <p>Handle large datasets with ease and efficiency.</p>
            </div>
          </div>
        </div>
      </section>

      {/* Showcase Section */}
      <section className='py-20 relative'>
        <div className='absolute top-0 left-0 w-full h-full bg-gradient-to-b from-dark via-transparent to-dark opacity-50'></div>
        <div className='max-w-6xl mx-auto px-8 relative z-10'>
          <div className='grid grid-cols-1 md:grid-cols-2 items-center gap-12'>
            <div>
              <h2 className='text-4xl font-bold mb-4'>
                Transform Your Workflow
              </h2>
              <p>
                Discover a seamless way to work with data. Our tools are
                designed to make your workflow smarter and faster.
              </p>
            </div>
            <div className='rounded overflow-hidden shadow-lg'>
              <img
                src='https://via.placeholder.com/600x400'
                alt='Showcase'
                className='w-full h-auto'
              />
            </div>
          </div>
        </div>
      </section>

      {/* Call to Action */}
      <section className='py-20 text-center bg-accent text-dark'>
        <h2 className='text-4xl font-bold mb-4'>
          Join the Future of Data Collaboration
        </h2>
        <p className='mb-8'>
          Start transforming your data today with DataVerse.
        </p>
        <button className='px-6 py-3 bg-dark text-light font-bold rounded hover:bg-gray-700'>
          Sign Up Now
        </button>
      </section>

      <Footer />
    </div>
  );
};

export default LandingPage;
