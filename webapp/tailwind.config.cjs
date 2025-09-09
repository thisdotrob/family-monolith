module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "../apps/web/**/*.{js,ts,jsx,tsx}",
    "../shared/**/*.{js,ts,jsx,tsx}",
  ],
  safelist: [
    // Safelist commonly used classes in external app modules (placeholder)
    'min-h-screen', 'bg-gray-100', 'flex', 'items-center', 'justify-center',
    'bg-white', 'p-8', 'rounded-lg', 'shadow-md', 'w-full', 'max-w-md', 'text-center',
    'text-2xl', 'font-bold', 'mb-6', 'mt-6', 'bg-red-500', 'hover:bg-red-700', 'text-white',
    'py-2', 'px-4', 'rounded', 'focus:outline-none', 'focus:shadow-outline'
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
