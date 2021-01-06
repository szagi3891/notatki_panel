import { makeAutoObservable } from "mobx";
import { ContentState } from "./ContentState";
import { CounterState } from "./inne/CounterState";
import { CurrentPath } from "./CurrentPath";

export class AppState {
    private readonly counter: CounterState;
    readonly contentState: ContentState;
    readonly currentPath: CurrentPath;

    constructor() {
        makeAutoObservable(this); //for computed

        this.counter = new CounterState();
        this.contentState = new ContentState();
        this.currentPath = new CurrentPath([]);

        // (async () => {
        //     const resp = await apiGetPath('aaa/ffff');

        //     console.info('AAAPI', resp);

        // })().catch((error) => {
        //     console.error(error);
        // });
    }

    get counterValue(): number {
        return this.counter.counter.value;
    }

    get listDir(): Array<string> | null {
        const path = this.currentPath.currentPath;
        const parentCurrentPath = this.currentPath.parentCurrentPath;

        const content = this.contentState.getPath(path);

        if (content === null) {
            return null;
        }

        if (content.type === 'dir') {
            return content.list;
        }

        const contentPrev = this.contentState.getPath(parentCurrentPath);
        
        if (contentPrev === null) {
            return null;
        }

        if (contentPrev.type === 'dir') {
            return contentPrev.list;
        }

        throw Error('nieprawidłowe odgałęzienie');
    }
}
