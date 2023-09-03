/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}', './public/index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        cosmonicPurple: {
          light: '#8177c7',
          dark: '#685BC7'
        },
        cosmonicGray: '#768692',
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
