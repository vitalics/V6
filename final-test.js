console.log("Starting test: final-test.js");

defineConfig({
  iterations: Infinity,
  duration: 3,
  timeout: 30, // max timeout for each iteration
  vus: 2, // Virtual Users
  iteration: async function () {
    console.log("[JS] iteration starting");
    
    // Add your test logic here
    // Example: await setTimeout(1000);
    
    console.log("[JS] iteration completed");
  },
});
