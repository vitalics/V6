console.log("Hello", "test-single-vu!");
console.error("Starting test with single VU!");

defineConfig({
  iterations: 5,
  duration: 60,
  timeout: 5,
  vus: 1,  // 1 Virtual User (default)
  iteration: function () {
    console.log("[JS] iteration running on single VU - processing task...");
  },
});