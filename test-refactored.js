console.log("Starting test: test-refactored.js");

defineConfig({
  iterations: 3,
  duration: 10,
  timeout: 30, // max timeout for each iteration
  vus: 1, // Virtual Users
  iteration: async function () {
    console.log("[JS] iteration starting");
    
    // Add your test logic here
    // Example: await setTimeout(1000);
    
    console.log("[JS] iteration completed");
  },
});
