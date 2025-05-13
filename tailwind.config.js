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

// 添加到您的CSS文件中或者style标签中
// .animate-float {
//   animation: float 6s ease-in-out infinite;
// }
// 
// .animate-pulse-subtle {
//   animation: pulse-subtle 3s ease-in-out infinite;
// }
// 
// .animate-bounce-subtle {
//   animation: bounce-subtle 1s ease-in-out infinite;
// }
// 
// @keyframes float {
//   0%, 100% { transform: translateY(0); }
//   50% { transform: translateY(-15px); }
// }
// 
// @keyframes pulse-subtle {
//   0%, 100% { opacity: 1; }
//   50% { opacity: 0.8; }
// }
// 
// @keyframes bounce-subtle {
//   0%, 100% { transform: translateY(0); }
//   50% { transform: translateY(-3px); }
// }
