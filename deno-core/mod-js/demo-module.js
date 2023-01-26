Deno.core.initializeAsyncOps();

// Zinnia SDK
const Zinnia = {
  log(msg) {
    Deno.core.ops.op_log(msg)
  },

  async sleep(durationInMs) {
    return Deno.core.ops.op_sleep(durationInMs);
  },
};

// DEMO MODULE

// Built-in Deno API
Deno.core.print('Hello via Deno logger\n')

// Our custom API would be wrapped by Zinnia SDK for JS/TS
Zinnia.log('Good night...')
Zinnia.sleep(1000).then(
  _ => Zinnia.log('Good morning!'),
  err => Deno.core.print(err.stack)
)
