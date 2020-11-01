import { getClientJs } from './server/getClientJs';
import { indexContent } from './server/index.html';
import { getEnvParams } from './server/lib/getEnvParams';
import { SyncState } from './server/SyncState';
import * as http from 'http';

const port = 3000;

const envParams = getEnvParams();

const sync = new SyncState(envParams.GIT_REPO);
sync.run().catch((error) => {
    console.error(error);
});

const getBody = (request: http.IncomingMessage): Promise<string> => {
    return new Promise((resolve) => {
        const body: Array<string> = [];
        request.on('data', chunk => {
            body.push(chunk.toString());
        });
        request.on('end', () => {
            resolve(body.join(''));
        });
    });
};


http.createServer(async (request, response) => {
    try {
        const method: string | undefined = request.method;
        const url: string | undefined = request.url;
        //const post = request.

        if (url === '/' && method === 'GET') {
            response.write(indexContent());
            response.end();
            return;
        }

        if (url === '/static/client.js' && method === 'GET') {
            const content = await getClientJs(envParams.CLIENT_URL);
            response.writeHead(200, {
                'Content-Type': 'text/javascript'
            });
            response.write(content);
            response.end();
            return;
        }

        if (url === '/api' && method === 'POST') {
            const body = await getBody(request);
            console.info('/api body', body);

            response.write(JSON.stringify({}));
            response.end();
            return;
        }
        console.info(method, url);


        // response.writeHead(200, {
        //     'Content-Type': 'application/json',
        //     'X-Powered-By': 'bacon'
        // });
        // response.write('<html>');
        // response.write('<body>');
        // response.write('<h1>Hello, World!</h1>');
        // response.write('</body>');
        // response.write('</html>');
        // response.end();

        response.writeHead(501);
        response.write(`Nieobsłuzony handler ${method} ${url}`);
        response.end();
    } catch (err) {
        console.error(err);
        response.writeHead(500);
        response.write(`Coś poszło nie tak ${err}`);
        response.end();
        return;
    }

}).listen(port);


console.info('Starting ...');
console.log(`Server app listening at http://localhost:${port}`);




