/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    './templates/**/*.html',
    './src/**/*.rs'
  ],
  theme: {
    extend: {
      colors: {
        // Semantic surface tokens (dark-first mindset)
        surface: {
          base: 'rgb(15 23 42)',
          // layered surfaces
          sunken: 'rgb(2 6 23)',
          raised: 'rgb(30 41 59)',
          border: 'rgb(51 65 85)'
        }
      }
    }
  },
  plugins: [require('@tailwindcss/typography')]
};
