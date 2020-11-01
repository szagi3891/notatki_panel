import express from 'express';
import { getClientJs } from './server/getClientJs';
import { indexContent } from './server/index.html';
import { getEnvParams } from './server/lib/getEnvParams';
import { SyncState } from './server/SyncState';

const app = express();
const port = 3000

const envParams = getEnvParams();

const sync = new SyncState(envParams.GIT_REPO);
sync.run().catch((error) => {
    console.error(error);
});

app.get('/', (_req, res) => {
    res.send(indexContent());
});

app.get('/static/client.js', async (_req, res): Promise<void> => {
    try {
        const content = await getClientJs(envParams.CLIENT_URL);
        res.setHeader('Content-Type', 'text/javascript');
        res.status(200).send(content);
    } catch (err) {
        console.error(err);
        res.status(500).send(`Coś poszło nie tak ${err}`);
    }
});

console.info('Starting ...');

app.listen(port, () => {
    console.log(`Server app listening at http://localhost:${port}`)
});

