// services/authService.ts
import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL;

export const signUp = async (email: string, password: string) => {
  try {
    console.log(API_URL);
    const response = await axios.post(`${API_URL}/signup`, {
      email,
      password_hash: password,
    });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    // Now `error` is typed as `any`, so it can access any properties.
    throw new Error(error.response?.data?.message || 'Signup failed');
  }
};

export const signIn = async (email: string, password: string) => {
  try {
    const response = await axios.post(`${API_URL}/signin`, { email, password });
    return response.data;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    throw new Error(error.response?.data?.message || 'Sign-in failed');
  }
};
