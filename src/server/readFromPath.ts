import * as fs from 'fs';

export type ContentPathType = {
    type: 'file',
    lastWrite: number,
} | {
    type: 'dir',
    list: Array<string>,
};

export const readContentPath = async (path: string): Promise<ContentPathType | null> => {
    try {
        const state = await fs.promises.stat(path);

        if (state.isFile()) {
            //const content = await fs.promises.readFile(path);
            return {
                type: 'file',
                lastWrite: state.mtimeMs,
            };
        }

        if (state.isDirectory()) {
            const list = await fs.promises.readdir(path);
            return {
                type: 'dir',
                list: list.map((dirItem) => `${path}/${dirItem}`)
            };
        }
    } catch (error) {
        if (error.code === 'ENOENT' && error.syscall === 'stat') {
            return null;
        }

        throw error;
    }

    throw Error('Nieprawidłowy branch');
}