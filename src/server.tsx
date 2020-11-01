import express from 'express';
import { getClientJs } from './server/getClientJs';
import { indexContent } from './server/index.html';
import { SyncState } from './server/SyncState';

const CLIENT_URL = process.env.CLIENT_URL;          //http adres, lub relatywna ścieka
const GIT_REPO = process.env.GIT_REPO;              //relatwyna ściezka do repo

if (typeof CLIENT_URL !== 'string') {
    console.error('Brakuje zmiennej środowiskowej: "CLIENT_URL"');
    process.exit(1);
}

if (typeof GIT_REPO !== 'string') {
    console.error('Brakuje zmiennej środowiskowej: "GIT_REPO"');
    process.exit(1);
}

console.info('Params env', {
    CLIENT_URL,
    GIT_REPO
});

const app = express();
const port = 3000


const sync = new SyncState(GIT_REPO);
sync.run().catch((error) => {
    console.error(error);
});


app.get('/', (req, res) => {
    res.send(indexContent());
});

app.get('/static/client.js', async (req, res): Promise<void> => {
    try {
        const content = await getClientJs(CLIENT_URL);
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

