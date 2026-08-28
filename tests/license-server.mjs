import http from 'node:http';

const server = http.createServer((request, response) => {
  const token = new URL(request.url, 'http://127.0.0.1').searchParams.get('license');
  response.writeHead(200, { 'content-type': 'application/json' });
  response.end(JSON.stringify({ valid: token === 'valid-token', reason: token === 'valid-token' ? 'ok' : 'invalid', expires_at: null }));
});
server.listen(Number(process.env.PORT || 43999), '127.0.0.1', () => process.stdout.write('ready\n'));
