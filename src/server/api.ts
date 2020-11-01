import * as t from 'io-ts';
import { ApiGetPathResponseIO } from 'src/client/api/apiGetPath';
import { buildValidatorWithUnwrap } from 'src/common/buildValidator';

const ParamsIO = t.interface({
    method: t.literal('getPath'),
    params: ApiGetPathResponseIO
});


export const decodeParams = buildValidatorWithUnwrap('ParamsIO', ParamsIO);

