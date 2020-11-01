import * as path from 'path';

const getBasePath = (): string => {
    const PATH_SERVER = path.dirname(process.argv[1]);
    return path.join(PATH_SERVER, '..');
};

export const getAbsolutePath = (src: string): string => {
    const basePath = getBasePath();
    return path.join(basePath, src);
};
