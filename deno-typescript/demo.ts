const response = await fetch('https://www.filstation.app')
console.log(`Status: ${response.status} ${response.statusText}`)

const file = await Deno.create(new URL(import.meta.resolve('./station.html')))
const writer = file.writable.getWriter()

for await (const chunk of response.body) {
  await writer.write(chunk)
}

file.close()
