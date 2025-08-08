console.log("Starting test: api-load-test.js");

defineConfig({
  iterations: Infinity,
  duration: 10,
  timeout: 15, // max timeout for each iteration
  vus: 5, // Virtual Users
  iteration: async function () {
    console.log("[JS] iteration starting");
    
    // Add your test logic here
    // Example: await setTimeout(1000);
    
    console.log("[JS] iteration completed");
  },
});
