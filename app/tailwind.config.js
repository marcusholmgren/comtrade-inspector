/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'primary-color': 'var(--primary-color)'
      }
    },
  },
  plugins: [],
}
