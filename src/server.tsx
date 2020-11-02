import { getClientJs } from './server/getClientJs';
import { indexContent } from './server/index.html';
import { getEnvParams } from './server/lib/getEnvParams';
import { SyncState } from './server/SyncState';
import * as http from 'http';
import { getBody } from './server/lib/getBody';
import { readContentPath } from './server/readFromPath';
import { ApiGetPathResponseType, decodeApiGetPathParams } from './client/api/apiGetPath';
import { jsonParse } from './common/fetchRequest';

const port = 3000;
const envParams = getEnvParams();

console.info('Starting ...');

const sync = new SyncState(envParams.GIT_REPO);
sync.run().catch((error) => {
    console.error(error);
});

http.createServer(async (request, response) => {
    try {
        const method: string | undefined = request.method;
        const url: string | undefined = request.url;

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

        if (typeof url === 'string' && url.startsWith('/api') && method === 'POST') {
            const body = await getBody(request);
            const bodyJson = jsonParse(body);

            console.info('/api/get-path body', body);

            if (bodyJson.type === 'text') {
                response.writeHead(400);
                response.write(`Oczekiwano jsona na wejściu ${method} ${url}`);
                response.end();
                return;
            }

            if (url === '/api/get-path') {
                const methodAndParams = decodeApiGetPathParams(bodyJson.json);
                const data: ApiGetPathResponseType = await readContentPath(`${envParams.GIT_REPO}/${methodAndParams.path}`);
    
                console.info('data', {
                    path: methodAndParams.path,
                    data
                });

                response.write(JSON.stringify(data));
                response.end();
                return;
            }
        }

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

console.log(`Server app listening at http://localhost:${port}`);



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
