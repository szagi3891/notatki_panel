import * as fs from 'fs';
import { fetchRequest } from 'src/common/fetchRequest';


export const getClientJs = async (CLIENT_URL: string): Promise<string> => {
    if (CLIENT_URL.startsWith('http')) {

        const respText = await fetchRequest('GET', CLIENT_URL, undefined, (status, data) => {
            if (status === 200 && data.type === 'text') {
                return data.text;
            }

            throw Error(`Niespodziewana odpowiedź ${status} ${data.type}`);
        });

        return respText;
    }

    const content = await fs.promises.readFile(CLIENT_URL);
    return content.toString();
}
