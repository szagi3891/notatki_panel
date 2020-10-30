import express from 'express';
import { indexContent } from './server/index.html';
import * as fs from 'fs';

const app = express();
const port = 3000

app.get('/', (req, res) => {
    res.send(indexContent());
});

app.get('/static/client.js', async (req, res): Promise<void> => {
    try {
        const content = await fs.promises.readFile('./dist/client.js');
        res.setHeader('Content-Type', 'text/javascript');
        res.status(200).send(content);
    } catch (err) {
        console.error(err);
        res.status(500).send(`Coś poszło nie tak ${err}`);
    }
});

app.listen(port, () => {
    console.log(`Server app listening at http://localhost:${port}`)
});

