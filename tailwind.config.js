/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-15px)' },
        },
        'pulse-subtle': {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.8' },
        },
        'bounce-subtle': {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-3px)' },
        },
      },
      animation: {
        'float': 'float 6s ease-in-out infinite',
        'pulse-subtle': 'pulse-subtle 3s ease-in-out infinite',
        'bounce-subtle': 'bounce-subtle 1s ease-in-out infinite',
      },
    },
  },
  plugins: [],
}