import { makeAutoObservable } from "mobx";
import { MobxValue } from "../utils/MobxValue";
import { ContentState } from "./ContentState";

export class CurrentPath {
    readonly path: MobxValue<string>;
    readonly content: ContentState;

    constructor(path: string, content: ContentState) {
        makeAutoObservable(this); //for computed

        this.path = new MobxValue(path);
        this.content = content;
    }

    //computed
    // ===> Array<string>
}