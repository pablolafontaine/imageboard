/** @type {import('tailwindcss').Config} */
const { fontFamily } = require('tailwindcss/defaultTheme')
module.exports = {
  darkMode: 'class',
  content: ["./src/**/*.{html,rs}", "./index.html"],
  theme: {
    extend: {},
	fontFamily: {
      sans: [
        '"Segoe UI"',
        'Roboto',
        'sans-serif',
      ],
    },
  },
  plugins: [],
}
