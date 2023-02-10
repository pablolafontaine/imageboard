/** @type {import('tailwindcss').Config} */
const { fontFamily } = require('tailwindcss/defaultTheme')
module.exports = {
  darkMode: 'class',
  content: ["./src/**/*.{html,rs}", "./index.html"],
  theme: {
    extend: {
	  colors: {
		'custom-white': '#E6E7EC',
		'custom-gray': {
		'500': '#343541',
		'600': '#2E3136',
		'700': '#2A2B2E',
		'800': '#202123',
		},
		'custom-black': '#1B1B1C',
	  },
    },
	fontFamily: {
      sans: [
        'sans-serif',
      ],
    },
  },
  plugins: [],
}
