import * as http from 'http';

export const getBody = (request: http.IncomingMessage): Promise<string> => {
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
