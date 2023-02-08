/** @type {import('tailwindcss').Config} */
const { fontFamily } = require('tailwindcss/defaultTheme')
module.exports = {
  darkMode: 'class',
  content: ["./src/**/*.{html,rs}", "./index.html"],
  theme: {
	  colors: {
		 transparent: 'transparent',
 		 current: 'currentColor',
		'white': '#ffffff',
		'dark-purple': '#2d2038',
		'darkish-purple': '#322740',
		'purple': '#413651',
		'grey-purple': '#483e57',
		'hot-pink': '#f071e6',
		'soft-pink': '#f0718f',
	  },
    extend: {},
	fontFamily: {
      sans: [
        'sans-serif',
      ],
    },
  },
  plugins: [],
}
