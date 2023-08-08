/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}', './public/index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        wasmcloudGreen: {
          light: '#493CAE',
          dark: '#261F5B'
        },
        wasmcloudGray: '#768693',
        goldenCream: 'rgba(246, 215, 116, 0.4)'
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
