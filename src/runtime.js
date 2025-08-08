globalThis.setTimeout = async (delay) => {
  await core.ops.op_set_timeout(delay);
};

globalThis.sleep = async (delay) => {
  await core.ops.op_set_timeout(delay);
};

const { core } = Deno;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

globalThis.console = {
  log: (...args) => {
    core.print(`[JS Runtime. out]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    core.print(`[JS Runtime. err]: ${argsToMessage(...args)}\n`, true);
  },
};

globalThis.currentConfig = null;
function noop() {}
globalThis.defineConfig = (config) => {
  if (!config.iteration || typeof config.iteration !== "function") {
    throw new Error("Invalid iteration definition. Expected a function");
  }
  globalThis.currentConfig = {
    setup: config.setup || noop,
    teardown: config.teardown || noop,
    iteration: config.iteration,
    iterations: config.iterations || 1,
    vus: config.vus || 1,
    duration: config.duration || 10,
    timeout: config.timeout || 30,
  };
};
// fetch

globalThis.fetch = (input, init) => {
  return core.ops.op_fetch(input, init);
};

// import * as headers from "ext:deno_fetch/20_headers.js";
// import * as formData from "ext:deno_fetch/21_formdata.js";
// import * as request from "ext:deno_fetch/23_request.js";
// import * as response from "ext:deno_fetch/23_response.js";
// import * as fetch from "ext:deno_fetch/26_fetch.js";
// import * as eventSource from "ext:deno_fetch/27_eventsource.js";

// Set up the callback for Wasm streaming ops
// Deno.core.setWasmStreamingCallback(fetch.handleWasmStreaming);

// Object.defineProperty(globalThis, "fetch", {
//   value: fetch.fetch,
//   enumerable: true,
//   configurable: true,
//   writable: true,
// });

// Object.defineProperty(globalThis, "Request", {
//   value: request.Request,
//   enumerable: false,
//   configurable: true,
//   writable: true,
// });

// Object.defineProperty(globalThis, "Response", {
//   value: response.Response,
//   enumerable: false,
//   configurable: true,
//   writable: true,
// });

// Object.defineProperty(globalThis, "Headers", {
//   value: headers.Headers,
//   enumerable: false,
//   configurable: true,
//   writable: true,
// });

// Object.defineProperty(globalThis, "FormData", {
//   value: formData.FormData,
//   enumerable: false,
//   configurable: true,
//   writable: true,
// });
