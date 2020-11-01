import { getAbsolutePath } from './getAbsolutePath';

interface EnvParamsType {
    GIT_REPO: string,
    CLIENT_URL: string,
}

export const getEnvParams = (): EnvParamsType => {
        
    const CLIENT_URL = process.env.CLIENT_URL;          //http adres, lub relatywna ścieka
    const GIT_REPO  = process.env.GIT_REPO;             //relatwyna ściezka do repo

    if (typeof CLIENT_URL !== 'string') {
        console.error('Brakuje zmiennej środowiskowej: "CLIENT_URL"');
        process.exit(1);
    }

    if (typeof GIT_REPO !== 'string') {
        console.error('Brakuje zmiennej środowiskowej: "GIT_REPO"');
        process.exit(1);
    }

    const resp: EnvParamsType = {
        GIT_REPO: getAbsolutePath(GIT_REPO),
        CLIENT_URL
    };

    console.info('Params env', resp);

    return resp;
};

