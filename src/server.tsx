import express from 'express';
import { indexContent } from './server/index.html';
import * as fs from 'fs';
import axios from 'axios';

const TIMEOUT = 10 * 1000;

const getClientJs = async (): Promise<string> => {
    const CLIENT_URL = process.env.CLIENT_URL;

    if (CLIENT_URL !== undefined) {
        const resp = await axios.request({
            method: 'GET',
            url: CLIENT_URL,
            //data: bodyParam === undefined ? undefined : JSON.stringify(bodyParam),
            //headers: getHeaders(backendToken, extraHeaders),
            transformResponse: [],
            validateStatus: () => true,
            timeout: TIMEOUT,
        });
    
        const respText = resp.data;

        if (typeof respText !== 'string') {
            console.error(respText);
            throw Error('String expected');
        }

        return respText;
    }

    const content = await fs.promises.readFile('./dist/client.js');
    return content.toString();
}

const app = express();
const port = 3000

app.get('/', (req, res) => {
    res.send(indexContent());
});

//https://github.com/szagi3891/notatki_panel/blob/master/dist/client.js?raw=true

console.info("process", process.argv);
console.info("env", process.env);


app.get('/static/client.js', async (req, res): Promise<void> => {
    try {
        const content = await getClientJs();
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

