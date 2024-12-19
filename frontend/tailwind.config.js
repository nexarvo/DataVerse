export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'sans-serif'], // Example for Google Fonts
        custom: ['CustomFont', 'sans-serif'], // Example for a local font
      },
      colors: {
        // dark: '#0F172A',
        dark: '#1E1E1E',
        // light: '#f5f5f5',
        light: '#252525',
        accent: '#252525',
        // third: '#334155',
        third: '#2C2C2C',
        // fourth: '#64748B',
        fourth: '#373737',
        'green-glass': 'rgba(72, 187, 120, 0.2)', // Light green transparent
        'green-glass-hover': 'rgba(72, 187, 120, 0.4)', // Slightly darker green on hover
        text: {
          primary: '#E5E5E5',
          secondary: '#B5B5B5',
          muted: '#8A8A8A',
          accent: '#4ADE80', // Or #FFD700
        },
      },
    },
  },
  plugins: [],
};
