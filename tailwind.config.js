module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  // safelist: [
  //   {
  //     pattern: /^(bg|to|from)-/,
  //     // pattern: /^(.*)-/, // enable this only during development to avoid rebuild tailwind css every time you add a new class to project.
  //   },
  // ],
  theme: {
    extend: {},
  },
  plugins: [],
};
