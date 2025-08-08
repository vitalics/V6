console.log("Sleep timing test starting");

defineConfig({
  iterations: 5, // Just 5 iterations to test timing
  duration: 30,
  timeout: 10,
  vus: 1,
  iteration: async function () {
    const start = Date.now();
    console.log(`[${new Date().toISOString()}] Starting iteration - timestamp: ${start}`);
    
    // Sleep for 2 seconds
    await globalThis.sleep(2000);
    
    const end = Date.now();
    const elapsed = end - start;
    console.log(`[${new Date().toISOString()}] Finished iteration - elapsed: ${elapsed}ms`);
    
    // Verify timing is approximately correct (allow some variance)
    if (elapsed >= 1900 && elapsed <= 2100) {
      console.log("✅ Sleep timing is correct!");
    } else {
      console.log(`❌ Sleep timing issue: expected ~2000ms, got ${elapsed}ms`);
    }
  },
});