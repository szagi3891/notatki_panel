

import { checkCommit } from "./lib/checkCommit";
import { execCommand, ExecResultType } from "./lib/execCommand";
import { timeout } from "./lib/timeout";

// class SyncMobxState {
//     isSyncEnable            -- dla widoku
// }

export class SyncState {
    readonly GIT_REPO: string;
    command: Array<() => Promise<void>>;
    lastSync: number;
    syncEnable: boolean;

    constructor(GIT_REPO: string) {
        this.GIT_REPO = GIT_REPO;
        this.command = [];
        this.lastSync = new Date().getTime();
        this.syncEnable = true;
    }

    runCommand(command: () => Promise<void>) {
        this.command.push(command);
    }


    async execCommand(command: string): Promise<ExecResultType> {
        console.info(`EXEC COMMAND: "${command}"`);
        return execCommand(command, this.GIT_REPO);
    }

    async execCommandAndShow(label: string, command: string): Promise<void> {
        const result = await this.execCommand(command);
        console.info({
            label,
            command,
            ...result,
        });
    }

    async execCommandWithSuccess(command: string): Promise<string> {
        console.info(`EXEC COMMAND: "${command}"`);
        const result = await execCommand(command, this.GIT_REPO);

        if (result.code !== 0 || result.stderr !== '') {
            console.error("Błąd wykonywania:", {
                command,
                result
            });
            throw Error("Błąd wykonywania");
        }

        return result.stdout;
    }

    async getCurrentBranch(): Promise<string> {
        const current = await this.execCommandWithSuccess('git branch --show-current');
        return current.trim();                                                          //trim ucina równiez entery
    }

    async getLocalCommit(branch: string): Promise<string> {
        const commit = await this.execCommandWithSuccess(`git log -1 origin/${branch} --pretty=format:"%H"`);
        checkCommit(commit);
        return commit;
    }

    async getRemoteCommit(branch: string): Promise<string> {
        const commit = await this.execCommandWithSuccess(`git log -1 ${branch} --pretty=format:"%H"`);
        checkCommit(commit);
        return commit;
    }
    
    async commitAsSynchronized(currentBranch: string): Promise<boolean> {
        const localCommit = await this.getLocalCommit(currentBranch);
        const remoteCommit = await this.getRemoteCommit(currentBranch);
        const comitEqual = localCommit === remoteCommit;

        if (comitEqual) {
            console.info(`CommitTest -> Equal ${localCommit}`);
        } else {
            console.info(`CommitTest -> NotEqual local="${localCommit}" remoteCommit="${remoteCommit}"`);
        }

        return comitEqual;
    }

    async trySync(): Promise<void> {
        const currentBranch = await this.getCurrentBranch();
        console.info(`Current branch = ${currentBranch}`);

        await this.execCommand(`git fetch origin ${currentBranch}`);

        if (await this.commitAsSynchronized(currentBranch)) {
            return;
        }

        console.info('Próba pull-a ...');

        await this.execCommandAndShow('GitPull', `git pull origin ${currentBranch}`);
        await this.execCommandAndShow('GitPullAbort', 'git merge --abort');

        if (await this.commitAsSynchronized(currentBranch)) {
            return;
        }

        console.info('Próba rebejsa ...');

        await this.execCommandAndShow('GitRebase', `git rebase origin/${currentBranch}`);
        await this.execCommandAndShow('GitRebaseAbort', 'git rebase --abort');
        await this.execCommandAndShow('GitRebasePush', `git push origin ${currentBranch}:${currentBranch}`);

        if (await this.commitAsSynchronized(currentBranch)) {
            return;
        }

        console.info('Próba rebejsowania nieudana !!!!! - wyłączam synchronizację');


        //jak się nie uda rebase, to trzeba wejść w tryb zawieszenia UI
        //czyli mozna po prostu wyłączyć główny proces synchronizujacy

        // setTimeout(() => {
        //     this.syncEnable = true;

        //     console.info('RESTORE....');
        // }, 10000);

        this.syncEnable = false;
    }


    async syncCommand(): Promise<void> {
        while (true) {
            const command = this.command.shift();

            if (command === undefined) {
                return;
            }

            await command();
        }
    }

    async run(): Promise<void> {
        while (true) {
            try {
                if (this.syncEnable) {
                    await this.syncCommand();
                                                                            //Synchronizuj z serwerem co 5s
                    const currentTime = new Date().getTime();
                    if (currentTime - this.lastSync > 5000) {
                        this.lastSync = currentTime;
                        await this.trySync();
                    }
                }

                await timeout(100);
            } catch (err) {
                console.error("Wywaliło cały proces synchronizujący - ponawiam", err);
            }
        }
    }
}