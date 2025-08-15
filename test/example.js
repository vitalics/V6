console.log("Hello", "runjs!");
console.error("Boom!");

defineConfig({
  iterations: Infinity,
  // iterations: 10,
  duration: 60,
  timeout: 10, // max timeout for each iteration
  vus: 1, // 2 Virtual Users
  iteration: async function () {
    // await setTimeout(1000);
    const res = await fetch("http://localhost:3001");
    console.log("Response. status", res.status);
  },
});
