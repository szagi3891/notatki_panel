import { exec } from 'child_process';

export interface ExecResultType {
    code: number;
    stdout: string,
    stderr: string
}



export const execCommand = async (command: string, cwd: string): Promise<ExecResultType> => {
    return new Promise((resolve, reject) => {
        exec(command, {cwd}, (error, stdout, stderr) => {
            if (error !== null) {
                const code = error.code;

                if (code === undefined) {
                    console.info('error command', error);
                    reject(error);
                    return;
                }

                resolve({
                    code: code,
                    stdout,
                    stderr
                });

                return;
            }

            resolve({
                code: 0,
                stdout,
                stderr
            });
        });
    });
};
