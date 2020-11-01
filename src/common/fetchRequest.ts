import axios from 'axios';

const TIMEOUT = 10 * 1000;

export type ResponseType = {
    type: 'json',
    json: unknown,
} | {
    type: 'text',
    text: string,
};

const jsonParse = (body: string): ResponseType => {
    try {
        const bodyJson = JSON.parse(body);

        return {
            type: 'json',
            json: bodyJson,
        };
    } catch (err) {
        return {
            type: 'text',
            text: body,
        };
    }
};

export const fetchRequest = async <P extends Record<string, string | number | boolean>, R>(
    method: 'GET' | 'POST',
    url: string,
    params: P | undefined,
    decode: (status: number, data: ResponseType) => R
): Promise<R> => {

    const response = await axios.request({
        method,
        url: url,
        data: params === undefined ? undefined : JSON.stringify(params),
        //headers: getHeaders(backendToken, extraHeaders),
        transformResponse: [],
        validateStatus: () => true,
        timeout: TIMEOUT,
    });

    const respData = response.data;

    if (typeof respData !== 'string') {
        console.error(respData);
        throw Error('String expected');
    }

    const parsedResponse = jsonParse(respData);
    return decode(response.status, parsedResponse);
};

