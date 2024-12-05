import React, { useState } from 'react';
import BackgroundShapes from '../components/BackgroundShapes';
import GoogleIcon from '../assets/google-icon.svg';
import PersonIcon from '../assets/person-icon.svg';
import { signIn, signUp } from '../services/authService';
import { useNavigate } from 'react-router-dom';

const AuthPage: React.FC = () => {
  const [isSignUp, setIsSignUp] = useState(true);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const navigate = useNavigate();

  const toggleForm = () => {
    setIsSignUp((prev) => !prev); // Toggle between sign-up and sign-in
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      let data;
      if (isSignUp) {
        data = await signUp(email, password);
        console.log('Sign Up Successful', data);
      } else {
        data = await signIn(email, password);
        console.log('Sign In Successful', data);
      }

      // Redirect to Dashboard
      navigate('/dashboard');
    } catch (error) {
      setErrorMessage((error as Error).message);
    }
  };

  return (
    <div className='relative min-h-screen flex flex-col items-center justify-center bg-retro'>
      {/* Background Shapes */}
      <BackgroundShapes />
      <h2 className='text-2xl font-bold text-center text-yellow-50 mb-6'>
        {isSignUp ? 'Create your account' : 'Log in'}
      </h2>
      <div className='w-96 max-w-lg p-8 space-y-6 bg-retro border border-blue-900 rounded-lg shadow-lg'>
        {/* Social Sign In Buttons */}
        <div className='flex flex-col items-center space-y-4 mt-6'>
          <button
            onClick={() => console.log('Sign in with Google')}
            className='w-80 py-2 px-4 border border-gray-300 text-white font-semibold rounded-md hover:bg-red-500 flex items-center justify-center gap-3'
          >
            <img src={GoogleIcon} alt='Google' className='w-5 h-5' />{' '}
            {/* Google icon */}
            <span>Continue with Google</span> {/* Text */}
          </button>
        </div>

        {/* Action buttons */}
        <div className='flex flex-col items-center space-y-4'>
          <button
            onClick={() => console.log('Continue as Guest')}
            className='w-80 py-2 px-4 border border-gray-300 text-white font-semibold rounded-md hover:bg-gray-50 flex items-center justify-center gap-3'
          >
            <img src={PersonIcon} alt='Person' className='w-6 h-6' />
            <span>Continue as Guest</span> {/* Text */}
          </button>
        </div>
        {/* Email/Password Form */}
        <div className='flex flex-col items-center space-y-4 mt-7'>
          <div className='flex justify-center'>
            <p className='text-sm text-gray-600'>or continue with email</p>
          </div>
        </div>
        <div className='space-y-6 mt-6'>
          <form onSubmit={handleSubmit}>
            <div className='flex justify-center'>
              <input
                type='email'
                id='email'
                placeholder='you@email.com'
                onChange={(e) => setEmail(e.target.value)}
                className='mt-2 w-80 p-1 pl-2 border border-gray-300 bg-transparent placeholder:text-sm rounded-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500'
              />
            </div>
            <div className='flex justify-center'>
              <input
                type='password'
                id='password'
                placeholder='password'
                onChange={(e) => setPassword(e.target.value)}
                className='mt-2 w-80 p-1 pl-2 border border-gray-300 bg-transparent placeholder:text-sm rounded-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500'
              />
            </div>
            {errorMessage && (
              <p className='text-red-500 text-sm'>{errorMessage}</p>
            )}
            <div className='flex flex-col items-center space-y-4 mt-6'>
              <button className='w-80 py-2 px-4 bg-indigo-600 text-white font-semibold rounded-md hover:bg-indigo-500'>
                {isSignUp ? 'Sign Up' : 'Sign In'}
              </button>
            </div>
          </form>

          {/* Toggle between sign-in and sign-up */}
          <div className='text-center mt-4'>
            <p className='text-sm text-gray-500'>
              {isSignUp
                ? 'Already have an account? '
                : "Don't have an account? "}
              <span
                onClick={toggleForm}
                className='text-indigo-600 cursor-pointer hover:underline'
              >
                {isSignUp ? 'Sign In' : 'Sign Up'}
              </span>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AuthPage;
