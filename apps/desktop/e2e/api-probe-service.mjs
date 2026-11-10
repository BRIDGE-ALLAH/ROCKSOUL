export default class WebDriverCommandProbe {
  before(_capabilities, _specs, browser) {
    const commandNames = [
      "execute",
      "executeAsyncScript",
      "executeAsync",
      "executeScript",
      "getWindowHandle",
      "getWindowHandles",
      "url",
      "getTitle",
      "$",
      "$$",
    ];
    const commandTypes = Object.fromEntries(
      commandNames.map((name) => [name, typeof browser?.[name]]),
    );
    const prototypeChain = [];
    let current = browser;
    while (current && current !== Object.prototype) {
      prototypeChain.push({
        constructor: current.constructor?.name ?? "unknown",
        scriptKeys: Object.getOwnPropertyNames(current).filter((key) =>
          /execute|script/i.test(key),
        ),
      });
      current = Object.getPrototypeOf(current);
    }
    console.log(
      "[WDIO_COMMAND_PROBE]",
      JSON.stringify({
        browserType: typeof browser,
        constructor: browser?.constructor?.name,
        sessionId: browser?.sessionId,
        isMultiRemote: browser?.isMultiRemote,
        isMultiremote: browser?.isMultiremote,
        instances: browser?.instances,
        getInstance: typeof browser?.getInstance,
        capabilities: browser?.capabilities,
        commandTypes,
        prototypeChain,
      }),
    );
  }
}
