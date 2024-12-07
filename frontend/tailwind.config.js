export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
      colors: {
        dark: '#0F172A',
        light: '#f5f5f5',
        accent: '#1E293B',
        third: '#334155',
        fourth: '#64748B',
        'green-glass': 'rgba(72, 187, 120, 0.2)', // Light green transparent
        'green-glass-hover': 'rgba(72, 187, 120, 0.4)', // Slightly darker green on hover
      },
    },
  },
  plugins: [],
};
