module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      fontSize: {
        body: "clamp(3rem, 5vmin, 4rem)",
        keysmall: "clamp(1.5rem, 4vmin, 3rem)",
        keybig: "clamp(1rem, 3vmin, 2rem)",
      }
    }
  },
  variants: {},
  plugins: [],
};
