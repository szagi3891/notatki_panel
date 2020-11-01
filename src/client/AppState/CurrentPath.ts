import { makeAutoObservable } from "mobx";
import { MobxValue } from "../../common/MobxValue";
import { ContentState } from "./ContentState";

export class CurrentPath {
    readonly path: MobxValue<string>;
    readonly content: ContentState;

    constructor(path: string, content: ContentState) {
        makeAutoObservable(this); //for computed

        this.path = new MobxValue(path);
        this.content = content;
    }

    get pathChunks(): Array<string> {
        return this.path.value
            .split('/')
            .map((item) => item.trim())
            .filter(item => item !== '')
        ;
    }
}