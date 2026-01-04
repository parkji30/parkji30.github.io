/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      fontFamily: {
        'cormorant': ['"Cormorant Garamond"', 'Georgia', 'serif'],
      },
      colors: {
        // Dark palette
        'bg-dark': '#0a0a0a',
        'bg-medium': '#111111',
        'building-dark': '#151515',
        'building-medium': '#1a1a1a',
        'building-light': '#202020',
        'lake-dark': '#0d0d0d',
        'lake-light': '#141414',
        'tree-dark': '#0a150a',
        'tree-medium': '#101a10',
        'tree-light': '#151f15',
        'path': '#1a1a1a',
        // UI colors
        'gray-dark': '#1a1a1a',
        'gray-medium': '#333333',
        'gray-light': '#666666',
        'gray-lighter': '#999999',
        'gray-pale': '#cccccc',
        'off-white': '#f0f0f0',
      },
      zIndex: {
        '100': '100',
        '200': '200',
      },
      backdropBlur: {
        'xs': '2px',
        'sm': '4px',
      },
      animation: {
        'fall': 'fall var(--fall-duration, 10s) linear infinite',
        'drift': 'drift var(--fall-duration, 10s) ease-in-out infinite',
        'spin-slow': 'spin var(--rotation-duration, 6s) linear infinite',
      },
      keyframes: {
        fall: {
          '0%': { top: '-20px' },
          '100%': { top: '100vh' },
        },
        drift: {
          '0%, 100%': { transform: 'translateX(0)' },
          '25%': { transform: 'translateX(8px)' },
          '75%': { transform: 'translateX(-8px)' },
        },
      },
    },
  },
  plugins: [],
}

