import { makeAutoObservable } from "mobx";
import { ContentState } from "./ContentState";
import { CounterState } from "./inne/CounterState";
import { CurrentPath } from "./CurrentPath";
import { apiGetPath } from "../api/apiGetPath";

export class AppState {
    private readonly counter: CounterState;
    readonly currentPath: CurrentPath;

    constructor() {
        makeAutoObservable(this); //for computed

        this.counter = new CounterState();
        this.currentPath = new CurrentPath('', new ContentState());

        (async () => {
            const resp = await apiGetPath('aaa/ffff');

            console.info('AAAPI', resp);

        })().catch((error) => {
            console.error(error);
        });
    }

    get counterValue(): number {
        return this.counter.counter.value;
    }
}
