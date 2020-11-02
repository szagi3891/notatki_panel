import * as t from 'io-ts';
import { buildValidatorWithUnwrap } from 'src/common/buildValidator';
import { fetchRequest } from "src/common/fetchRequest";

export const ApiGetPathParamsIO = t.interface({
    path: t.string,
});

export const decodeApiGetPathParams = buildValidatorWithUnwrap('ApiGetPathParamsIO', ApiGetPathParamsIO);

type ApiGetPathParamsType = t.TypeOf<typeof ApiGetPathParamsIO>;
export const ApiGetPathResponseIO = t.union([
    t.interface({
        type: t.literal('file'),
        lastWrite: t.number,
    }),
    t.interface({
        type: t.literal('dir'),
        list: t.array(t.string)
    }),
    t.null
]);

export type ApiGetPathResponseType = t.TypeOf<typeof ApiGetPathResponseIO>;

export const decodeApiGetPathResponse = buildValidatorWithUnwrap('ApiGetPathResponseIO', ApiGetPathResponseIO);

export const apiGetPath = async (path: string) => {
    const params: ApiGetPathParamsType = {
        path,
    };

    return await fetchRequest('POST', '/api/get-path', params, (status, data) => {
        if (status === 200 && data.type === 'json') {
            return decodeApiGetPathResponse(data.json);
        }

        throw Error(`Niespodziewana zwrotka z api ${status} ${data.type}`);
    });
};
