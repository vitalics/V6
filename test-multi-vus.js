console.log("Hello", "test-multi-vus!");
console.error("Starting test with multiple VUs!");

defineConfig({
  iterations: 10,
  duration: 60,
  timeout: 5,
  vus: 4,  // 4 Virtual Users
  iteration: function () {
    console.log("[JS] iteration running on VU - processing task...");
  },
});