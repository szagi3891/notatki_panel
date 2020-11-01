import * as fs from 'fs';
import axios from 'axios';

const TIMEOUT = 10 * 1000;

export const getClientJs = async (CLIENT_URL: string): Promise<string> => {
    if (CLIENT_URL.startsWith('http')) {
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

    const content = await fs.promises.readFile(CLIENT_URL);
    return content.toString();
}
