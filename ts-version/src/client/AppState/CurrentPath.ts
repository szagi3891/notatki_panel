import { makeAutoObservable } from "mobx";
import { MobxValue } from "src/common/MobxValue";

export class CurrentPath {
    readonly path: MobxValue<Array<string>>;

    constructor(path: Array<string>) {
        makeAutoObservable(this); //for computed

        this.path = new MobxValue(path);
    }

    get pathChunks(): Array<string> {
        return this.path.value;
    }

    get currentPath(): string {
        return this.path.value.join('/');
    }

    get parentCurrentPath(): string {
        const path = this.path.value.concat([]);
        path.pop();
        return path.join('/');
    }
}