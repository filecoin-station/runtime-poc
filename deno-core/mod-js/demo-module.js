
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

// Using Zinnia API
Zinnia.log('Good night...')
await Zinnia.sleep(1000);
Zinnia.log('Good morning!')
