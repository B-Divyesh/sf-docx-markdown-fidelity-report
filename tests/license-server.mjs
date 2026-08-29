import { createServer } from 'node:http';

const server = createServer((request, response) => {
  const token = new URL(request.url, 'http://127.0.0.1').searchParams.get('license');
  response.setHeader('content-type', 'application/json');
  response.end(JSON.stringify(token === 'valid-token' ? { valid: true, reason: 'ok', expires_at: null } : { valid: false, reason: 'invalid', expires_at: null }));
});
server.listen(Number(process.env.PORT || 43999), '127.0.0.1', () => process.stdout.write('ready\n'));
