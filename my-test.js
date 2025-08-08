console.log("Starting test: my-test.js");

defineConfig({
  iterations: 10,
  duration: 30,
  timeout: 5, // max timeout for each iteration
  vus: 2, // Virtual Users
  iteration: async function () {
    console.log("[JS] iteration starting");
    
    // Add your test logic here
    // Example: await setTimeout(1000);
    
    console.log("[JS] iteration completed");
  },
});
