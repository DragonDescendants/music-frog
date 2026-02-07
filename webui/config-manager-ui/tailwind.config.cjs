/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        ink: {
          900: 'rgb(var(--color-ink-900-rgb) / <alpha-value>)',
          700: 'rgb(var(--color-ink-700-rgb) / <alpha-value>)',
          500: 'rgb(var(--color-ink-500-rgb) / <alpha-value>)',
        },
        accent: {
          500: 'rgb(var(--color-accent-500-rgb) / <alpha-value>)',
          400: 'rgb(var(--color-accent-400-rgb) / <alpha-value>)',
          200: 'rgb(var(--color-accent-200-rgb) / <alpha-value>)',
          100: 'rgb(var(--color-accent-100-rgb) / <alpha-value>)',
          50: 'rgb(var(--color-accent-50-rgb) / <alpha-value>)',
        },
        sand: {
          200: 'rgb(var(--color-sand-200-rgb) / <alpha-value>)',
          100: 'rgb(var(--color-sand-100-rgb) / <alpha-value>)',
          50: 'rgb(var(--color-sand-50-rgb) / <alpha-value>)',
        },
        sun: {
          500: 'rgb(var(--color-sun-500-rgb) / <alpha-value>)',
          400: 'rgb(var(--color-sun-400-rgb) / <alpha-value>)',
        },
        primary: {
          500: 'rgb(var(--color-accent-500-rgb) / <alpha-value>)',
          200: 'rgb(var(--color-accent-200-rgb) / <alpha-value>)',
          50: 'rgb(var(--color-accent-50-rgb) / <alpha-value>)',
        },
      },
      boxShadow: {
        panel: 'var(--shadow-panel)',
      },
      fontFamily: {
        display: ['Manrope', 'sans-serif'],
      },
      keyframes: {
        pulseSoft: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.7' },
        },
      },
      animation: {
        'pulse-soft': 'pulseSoft 1.6s ease-in-out infinite',
      },
    },
  },
  plugins: [],
};
