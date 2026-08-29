import { createReadStream } from 'node:fs';
import { stat } from 'node:fs/promises';
import { createServer } from 'node:http';
import { extname, resolve, sep } from 'node:path';

const argumentsList = process.argv.slice(2);
const option = (name, fallback) => {
  const index = argumentsList.indexOf(name);
  return index >= 0 && argumentsList[index + 1] ? argumentsList[index + 1] : fallback;
};
const host = option('--host', '127.0.0.1');
const port = Number(option('--port', '4173'));
const root = resolve(import.meta.dirname, '../dist/site');
const appRoutes = new Set(['/', '/demo', '/privacy', '/terms']);
const contentTypes = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.png': 'image/png',
  '.svg': 'image/svg+xml',
  '.txt': 'text/plain; charset=utf-8',
  '.webp': 'image/webp',
  '.woff2': 'font/woff2',
  '.xml': 'application/xml; charset=utf-8',
};

const headers = {
  'Content-Security-Policy': "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'",
  'X-Content-Type-Options': 'nosniff',
  'Referrer-Policy': 'strict-origin-when-cross-origin',
  'Permissions-Policy': 'camera=(), microphone=(), geolocation=()',
};

const sendFile = (response, filePath, statusCode) => {
  response.writeHead(statusCode, {
    ...headers,
    'Content-Type': contentTypes[extname(filePath)] || 'application/octet-stream',
    'Cache-Control': filePath.includes(`${sep}assets${sep}`)
      ? 'public, max-age=31536000, immutable'
      : 'public, must-revalidate, max-age=30',
  });
  createReadStream(filePath).pipe(response);
};

createServer(async (request, response) => {
  if (!['GET', 'HEAD'].includes(request.method || '')) {
    response.writeHead(405, headers).end();
    return;
  }
  const pathname = decodeURIComponent(new URL(request.url || '/', `http://${host}:${port}`).pathname);
  const normalizedRoute = pathname.length > 1 ? pathname.replace(/\/$/, '') : pathname;
  if (appRoutes.has(normalizedRoute)) {
    sendFile(response, resolve(root, 'index.html'), 200);
    return;
  }
  const requestedPath = resolve(root, `.${pathname}`);
  if (requestedPath.startsWith(`${root}${sep}`)) {
    try {
      if ((await stat(requestedPath)).isFile()) {
        sendFile(response, requestedPath, 200);
        return;
      }
    } catch {
      // The designed 404 below handles missing files.
    }
  }
  sendFile(response, resolve(root, '404.html'), 404);
}).listen(port, host, () => {
  process.stdout.write(`Preview server listening at http://${host}:${port}\n`);
});
