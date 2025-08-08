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
